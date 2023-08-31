use std::collections::HashMap;

use rspotify::{
    model::FullTrack,
    prelude::{BaseClient, OAuthClient, PlayableId},
    AuthCodeSpotify,
};

use crate::{
    constants, plan_command,
    traits::{OptionExtension, ResultExtension},
};

use super::args;

pub async fn handle_apply_snapshot(
    cmd: &args::ApplyCommand,
    spotify: &AuthCodeSpotify,
) -> Result<(), anyhow::Error> {
    let gv = plan_command::read_snapshot(cmd.id, "edit")?;
    let all_actions = plan_command::create_execution_plan(&gv)?;

    // TODO:For better performance, maybe create a list of tracks and use refs in the map like
    // let mut map: HashMap<String, Vec<&rspotify::model::FullTrack>> = HashMap::new();

    let mut map: HashMap<String, Vec<rspotify::model::FullTrack>> = HashMap::new();
    let mut node_to_playlist_id: HashMap<String, String> = HashMap::new();

    let user = spotify
        .current_user()
        .await
        .or_error_str("failed to fetch user")?;

    for actions in &all_actions {
        for action in actions {
            log::info!(
                "{:?} from {} for/to {} and idx is {}",
                action.action_type,
                action.node,
                action.for_node,
                action.idx,
            );
        }
    }

    for actions in all_actions {
        for action in actions {
            if action.for_node == constants::MIXIFY_TEMPORARY_ROOT_NODE_NAME {
                continue;
            }

            match action.action_type {
                plan_command::ActionType::CreatePlaylist => {
                    let playlist = spotify
                        .user_playlist_create(
                            user.id.as_ref(),
                            action.node.as_str(),
                            Some(false),
                            Some(false),
                            Some("test"),
                        )
                        .await
                        .or_error_str("failed to create playlist")?;
                    log::info!("Created playlist {:?}", playlist);

                    map.insert(action.node.clone(), vec![]);
                    node_to_playlist_id.insert(
                        action.node.clone(),
                        playlist
                            .id
                            .to_string()
                            .split(":")
                            .last()
                            .unwrap()
                            .to_string(),
                    );
                }
                plan_command::ActionType::QuerySongsPlaylist(url) => {
                    if let Some(_) = map.get(&action.node) {
                        log::info!("Playlist {:?} already has tracks", action.node);
                        continue;
                    }

                    let playlist_id_str = url.split("/").last().unwrap();
                    let playlist_id = rspotify::model::PlaylistId::from_id(playlist_id_str.clone())
                        .or_error(format!(
                            "failed to parse playlist id correctly from url {}. the parsed id {}",
                            url.clone(),
                            playlist_id_str
                        ))?;

                    node_to_playlist_id.insert(
                        action.node.clone(),
                        playlist_id
                            .to_string()
                            .split(":")
                            .last()
                            .unwrap()
                            .to_string(),
                    );

                    // TODO: There is a field arg that can be passed, idk what it is.
                    // If stuff fails, this might be the reason.
                    let playlist = spotify
                        .user_playlist(user.id.as_ref(), Some(playlist_id), None)
                        .await
                        .or_error_str("failed to fetch playlist")?;

                    let tracks = playlist
                        .tracks
                        .items
                        .into_iter()
                        .filter_map(|item| {
                            let item = item.track.or_error(format!(
                                "could not work with a song from the playlist id of {}",
                                playlist_id_str
                            ));

                            if let Err(e) = item {
                                log::warn!("{}", e);
                                return None;
                            }

                            match item.unwrap() {
                                rspotify::model::PlayableItem::Track(track) => Some(track),
                                rspotify::model::PlayableItem::Episode(_) => {
                                    let msg = format!(
                                        "Skipping episode from playlist {:?}",
                                        playlist_id_str
                                    );
                                    log::warn!("{}", msg);

                                    None
                                }
                            }
                        })
                        .collect::<Vec<FullTrack>>();

                    map.insert(action.node, tracks);
                }
                plan_command::ActionType::CopySongs => {
                    // NOTE: Again using refs would be better than cloning.
                    let tracks = map.get(&action.node).unwrap().clone();
                    let target = map.get_mut(&action.for_node).unwrap();
                    let playlist_id_str = node_to_playlist_id.get(&action.for_node).unwrap();

                    let playlist_id = rspotify::model::PlaylistId::from_id(playlist_id_str)
                        .or_error(format!(
                            "failed to parse playlist id correctly from node {}. the parsed id {}",
                            &action.node, playlist_id_str
                        ))?;

                    for t in &tracks {
                        target.push(t.clone());
                    }

                    let items = tracks
                        .iter()
                        .map(|t| PlayableId::Track(t.id.clone().unwrap()))
                        .collect::<Vec<rspotify::model::PlayableId>>();

                    log::info!("Adding songs to playlist {:?}", &playlist_id);
                    let res = spotify.playlist_add_items(playlist_id, items, None).await;

                    if let Err(e) = res {
                        return Err(anyhow::anyhow!("Failed to add songs to playlist {:?}", e));
                    }

                    log::info!("Added songs successfully");
                }
                plan_command::ActionType::RemoveSongs => {
                    // NOTE: Again using refs would be better than cloning.
                    let tracks = map.get(&action.node).unwrap().clone();
                    let target = map.get_mut(&action.for_node).unwrap();
                    let playlist_id_str = node_to_playlist_id.get(&action.for_node).unwrap();

                    let playlist_id = rspotify::model::PlaylistId::from_id(playlist_id_str)
                        .or_error(format!(
                            "failed to parse playlist id correctly from node {}. the parsed id {}",
                            &action.node, playlist_id_str
                        ))?;

                    target.retain(|t| !tracks.contains(t));

                    let songs_to_remove = tracks;
                    let items = songs_to_remove
                        .iter()
                        .map(|t| PlayableId::Track(t.id.clone().unwrap()))
                        .collect::<Vec<rspotify::model::PlayableId>>();

                    log::info!("Removing songs to playlist {:?}", &playlist_id);
                    let res = spotify
                        .playlist_remove_all_occurrences_of_items(playlist_id, items, None)
                        .await;

                    if let Err(e) = res {
                        return Err(anyhow::anyhow!(
                            "Failed to remove songs to playlist {:?}",
                            e
                        ));
                    }

                    log::info!("Removed songs successfully");
                }
            }
        }
    }

    return Ok(());
}
