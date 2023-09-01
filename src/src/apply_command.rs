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
    let content = plan_command::read_snapshot_file(cmd.id, "edit")?;
    let gv =
        graphviz_dot_parser::parse(&content).or_error(String::from("failed to parse graph"))?;
    let graph = gv.to_directed_graph().unwrap();
    let all_actions = plan_command::create_execution_plan(&gv)?;

    // TODO:For better performance, maybe create a list of tracks and use refs in the map like
    // let mut map: HashMap<String, Vec<&rspotify::model::FullTrack>> = HashMap::new();
    let mut nodes_with_missing_playlists: Vec<String> = Vec::new();
    let mut map: HashMap<String, Vec<rspotify::model::FullTrack>> = HashMap::new();
    let mut node_to_playlist_id: HashMap<String, String> = HashMap::new();

    let user = spotify
        .current_user()
        .await
        .or_error_str("failed to fetch user")?;

    for actions in &all_actions {
        for action in actions {
            log::info!("{}", action,);
        }
    }

    for actions in all_actions {
        for action in actions {
            if action.for_node == constants::MIXIFY_TEMPORARY_ROOT_NODE_NAME {
                continue;
            }

            match action.action_type {
                plan_command::ActionType::CreatePlaylist => {
                    let node_index = graph
                        .node_indices()
                        .find(|i| graph[*i] == *action.node)
                        .unwrap();
                    let nei = graph.neighbors_directed(node_index, petgraph::Direction::Incoming);
                    let names = nei.map(|i| graph[i].clone()).collect::<Vec<String>>();
                    let description = format!(
                        "Generated mixstack using mixify. This playlist consists of: {}",
                        names.join(", ")
                    );

                    let playlist = spotify
                        .user_playlist_create(
                            user.id.as_ref(),
                            format!("{}™", action.node).as_str(),
                            Some(false),
                            Some(false),
                            Some(description.as_str()),
                        )
                        .await
                        .or_error_str("failed to create playlist")?;
                    log::info!("Created playlist {:?}", playlist);

                    nodes_with_missing_playlists.push(action.node.clone());
                    map.insert(action.node.clone(), vec![]);
                    node_to_playlist_id
                        .insert(action.node.clone(), parse_id_from_playlist_id(&playlist.id));
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

                    node_to_playlist_id
                        .insert(action.node.clone(), parse_id_from_playlist_id(&playlist_id));

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

    log::info!("Successfully applied snapshot");

    let paths = plan_command::list_snapshot_files(cmd.id, "edit")?;
    let path = paths.get(0).unwrap().to_str().unwrap();
    let pre_apply_path = path.replace("edit", "pre.apply");
    let post_apply_path = path.replace("edit", "post.apply");

    let new_content = create_post_apply_file(
        &content,
        &node_to_playlist_id,
        &nodes_with_missing_playlists,
        &spotify,
    )
    .await?;

    std::fs::rename(path, pre_apply_path)?;
    std::fs::write(post_apply_path, new_content)?;
    return Ok(());
}

async fn create_post_apply_file(
    content: &String,
    node_to_playlist_id: &HashMap<String, String>,
    nodes_with_missing_playlists: &Vec<String>,
    spotify: &AuthCodeSpotify,
) -> Result<String, anyhow::Error> {
    let mut idx = 0;
    let mut new_content = content.clone();

    for (node, playlist_id) in node_to_playlist_id {
        if !nodes_with_missing_playlists.contains(node) {
            continue;
        }

        let chars = node.chars().collect::<Vec<char>>();
        let content_chars = new_content.chars().collect::<Vec<char>>();
        for (i, l) in content_chars.iter().enumerate() {
            if *l != chars[idx] {
                idx = 0;
                continue;
            } else if idx != chars.len() - 1 {
                idx += 1;
                continue;
            }

            let s = content_chars
                .iter()
                .skip(i - node.len())
                .take(node.len() + 1)
                .collect::<String>();

            let s = s.trim();

            idx = 0;
            if s != *node {
                continue;
            }

            for (j, c) in content.chars().skip(i).enumerate() {
                if c != '[' && c != ';' {
                    continue;
                }

                let has_attr = c == '[';
                let idxxx = match has_attr {
                    true => i + j + 1,
                    false => i + j,
                };

                let s = match has_attr {
                    true => format!(
                        "URL=\"https://open.spotify.com/playlist/{}\", ",
                        playlist_id
                    ),
                    false => format!(
                        " [URL=\"https://open.spotify.com/playlist/{}\"]",
                        playlist_id
                    ),
                };

                new_content.insert_str(idxxx, s.as_str());
                break;
            }

            // TODO: Remove later
            // let id = rspotify::model::PlaylistId::from_id(playlist_id.as_str().clone());
            // let r = spotify.playlist_unfollow(id.unwrap()).await;

            // match r {
            //     Ok(_) => log::info!("removed playlist successfully"),
            //     Err(_) => log::error!("did not remove playlist successfully"),
            // }

            break;
        }
    }

    Ok(new_content)
}

fn parse_id_from_playlist_id(playlist_id: &rspotify::model::PlaylistId) -> String {
    playlist_id
        .to_string()
        .split(":")
        .last()
        .unwrap()
        .to_string()
}
