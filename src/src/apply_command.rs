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
    let mut map: HashMap<String, Vec<rspotify::model::FullTrack>> = HashMap::new();
    let mut node_to_playlist_id: HashMap<String, String> = HashMap::new();
    let mut nodes_with_missing_playlists: Vec<String> = Vec::new();

    let user = spotify
        .current_user()
        .await
        .or_error_str("failed to fetch user")?;

    log::info!("------------------");
    log::info!("list of actions:");
    for actions in &all_actions {
        for action in actions {
            log::info!("{}", action);

            if map.get(&action.node).is_none() {
                map.insert(action.node.clone(), vec![]);
                map.insert(conv(&action.node.clone()), vec![]);

                if let Some(url) = &action.playlist_url {
                    let id = url.split("/").last().unwrap();
                    node_to_playlist_id.insert(action.node.clone(), id.to_string());
                } else {
                    nodes_with_missing_playlists.push(action.node.clone());
                }
            }
        }
    }
    log::info!("------------------");

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

                    node_to_playlist_id
                        .insert(action.node.clone(), parse_id_from_playlist_id(&playlist.id));
                }
                plan_command::ActionType::QuerySongs(url) => {
                    if let Some(songs) = map.get(&action.node) {
                        // By default, the playlist should be empty.
                        if songs.len() > 0 {
                            log::warn!(
                                "Playlist {:?} already has been queried. Skipping...",
                                action.node
                            );
                            continue;
                        }
                    }

                    log::info!("Querying songs for playlist {:?}", action.node);

                    let url = url
                        .or_else(|| node_to_playlist_id.get(&action.node).cloned())
                        .unwrap();

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
                        .user_playlist(user.id.as_ref(), Some(playlist_id.clone()), None)
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

                    let node_index = graph
                        .node_indices()
                        .find(|i| graph[*i] == *action.node)
                        .unwrap();
                    let nei = graph.neighbors_directed(node_index, petgraph::Direction::Incoming);

                    if nei.count() == 0 {
                        log::warn!(
                            "Playlist {:?} has no incoming edges. Skipping...",
                            action.node
                        );
                        map.insert(action.node, tracks);
                        continue;
                    }

                    let new_playlist_songs = map.get(conv(&action.node).as_str()).unwrap();
                    map.insert(action.node.clone(), new_playlist_songs.clone());

                    let new_playlist_songs = map.get(conv(&action.node).as_str()).unwrap();
                    let mut songs_to_remove = tracks.clone();
                    songs_to_remove.retain(|t| !new_playlist_songs.contains(t));

                    let mut songs_to_add = new_playlist_songs.clone();
                    songs_to_add.retain(|t| !tracks.contains(t));

                    if songs_to_add.len() != 0 {
                        let ids = songs_to_add
                            .iter()
                            .map(|t| PlayableId::Track(t.id.clone().unwrap()))
                            .collect::<Vec<rspotify::model::PlayableId>>();

                        let res = spotify
                            .playlist_add_items(playlist_id.clone(), ids, None)
                            .await;
                        if let Err(e) = res {
                            return Err(anyhow::anyhow!("Failed to add songs to playlist {:?}", e));
                        }

                        log::info!("Added songs successfully");
                    } else {
                        log::info!("No songs to add to playlist {:?}", &playlist_id);
                    }

                    if songs_to_remove.len() != 0 {
                        let ids = songs_to_remove
                            .iter()
                            .map(|t| PlayableId::Track(t.id.clone().unwrap()))
                            .collect::<Vec<rspotify::model::PlayableId>>();

                        log::info!("Removing songs to playlist {:?}", &playlist_id);
                        let res = spotify
                            .playlist_remove_all_occurrences_of_items(playlist_id, ids, None)
                            .await;

                        if let Err(e) = res {
                            return Err(anyhow::anyhow!(
                                "Failed to remove songs to playlist {:?}",
                                e
                            ));
                        }

                        log::info!("Removed songs successfully");
                    } else {
                        log::info!("No songs to remove to playlist {:?}", &playlist_id);
                    }
                }
                plan_command::ActionType::CopySongs => {
                    let tracks = map.get(&action.node).unwrap().clone();
                    let target = map
                        .get_mut(conv(action.for_node.as_str()).as_str())
                        .unwrap();

                    for t in tracks.iter() {
                        target.push(t.clone());
                    }
                }
                plan_command::ActionType::RemoveSongs => {
                    let tracks = map.get(&action.node).unwrap().clone();
                    let target = map.get_mut(conv(&action.for_node).as_str()).unwrap();
                    target.retain(|t| !tracks.contains(t));
                }
                plan_command::ActionType::SaveChanges(_) => {
                    // let tracks = map.get(&action.node).unwrap().clone();
                    // let target = map.get_mut(conv(&action.for_node).as_str()).unwrap();
                    // *target = tracks;
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

    // TODO: bring this back
    // std::fs::rename(path, pre_apply_path)?;
    // std::fs::write(post_apply_path, new_content)?;
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

fn conv(s: &str) -> String {
    format!("local-{}", s)
}
