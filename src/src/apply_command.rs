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
    allow_delete: bool,
) -> Result<(), anyhow::Error> {
    let content = plan_command::read_snapshot_file(cmd.id, "edit")?;
    let gv =
        graphviz_dot_parser::parse(&content).or_error(String::from("failed to parse graph"))?;
    let graph = gv.to_directed_graph().unwrap();
    let (all_actions, nodes) = plan_command::create_execution_plan(&gv)?;

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
                map.insert(to_local(&action.node.clone()), vec![]);

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
                        "Generated mixstack using mixify. This playlist consists of: {}.",
                        names.join(", ")
                    );

                    let (_, attr) = nodes
                        .iter()
                        .find(|(name, _)| *name == *action.node)
                        .unwrap();

                    let playlist_name_attr = attr
                        .iter()
                        .find(|(k, _)| k == constants::LABEL_ATTRIBUTE_KEY);

                    let mut playlist_name = action.node.clone();
                    if let Some((_, v)) = playlist_name_attr {
                        playlist_name = v.clone();
                    }

                    let playlist = spotify
                        .user_playlist_create(
                            user.id.clone(),
                            &format!("{}™", playlist_name),
                            Some(false),
                            Some(false),
                            Some(&description),
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
                                "Playlist {:?} already has been queried. This should never happen. Skipping...",
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

                    map.insert(action.node.clone(), tracks.clone());

                    let has_songs = map.get(&to_local(&action.node)).unwrap().len() > 0;
                    if !has_songs {
                        map.insert(to_local(&action.node), tracks.clone());
                    }
                }
                plan_command::ActionType::CopySongs => {
                    let tracks = map.get(&to_local(&action.node)).unwrap().clone();
                    let target = map.get_mut(&to_local(&action.for_node)).unwrap();

                    for t in &tracks {
                        if target.contains(t) {
                            continue;
                        }

                        target.push(t.clone());
                    }
                }
                // We dont care if the song was added by the user or the bot we remove it anyway.
                plan_command::ActionType::RemoveSongs => {
                    let remote = map.get(&action.node).unwrap().clone();
                    let local = map.get_mut(&to_local(&action.for_node)).unwrap();
                    local.retain(|t| !remote.contains(t));
                }
                plan_command::ActionType::SaveChanges(_) => {
                    let remote = map.get(&action.node).unwrap().clone();
                    let local = map.get_mut(&to_local(&action.for_node)).unwrap();

                    let mut songs_to_add = local.clone();
                    songs_to_add.retain(|t| !remote.contains(t));

                    let mut songs_to_remove = remote.clone();
                    songs_to_remove.retain(|t| !local.contains(t));

                    let playlist_id = node_to_playlist_id.get(&action.node).unwrap();
                    let playlist_id = rspotify::model::PlaylistId::from_id(playlist_id).unwrap();

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

                    log::info!("Option allow to delete is {:?}", allow_delete);
                    if !allow_delete {
                        log::info!(
                            "Skipping removing songs {:?} from playlist {:?}",
                            songs_to_remove.len(),
                            &playlist_id
                        );
                    } else if songs_to_remove.len() != 0 {
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

                    // Set the updated playlist state.
                    let state = local.clone();
                    map.insert(to_local(&action.node), state);
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
    )?;

    std::fs::rename(path, pre_apply_path)?;
    std::fs::write(post_apply_path, new_content)?;
    return Ok(());
}

pub fn create_post_apply_file(
    content: &String,
    node_to_playlist_url: &HashMap<String, String>,
    nodes_with_missing_playlists: &Vec<String>,
) -> Result<String, anyhow::Error> {
    let parts = content
        .split(";")
        .map(|part| part.to_string())
        .collect::<Vec<String>>();

    let mut res_parts = parts.clone();
    for (node, playlist_url) in node_to_playlist_url {
        if !nodes_with_missing_playlists.contains(node) {
            continue;
        }

        let mut item: Option<(usize, usize, String)> = None;
        for (idx, part) in parts.iter().enumerate() {
            if part.contains("->") {
                continue;
            }

            let trimmed = part.trim_matches(|c| c == ' ' || c == '\n');
            let node_with_quotes = format!("\"{}\"", node);

            if trimmed.starts_with(node) || trimmed.starts_with(&node_with_quotes) {
                part.chars().enumerate().for_each(|(i, c)| {
                    if c != '[' || item.is_some() {
                        return;
                    }

                    let mut end = ", ";
                    if !part.contains("]") {
                        end = "]";
                    }

                    let s = format!(
                        "URL=\"https://open.spotify.com/playlist/{}\"{}",
                        playlist_url, end,
                    );
                    item = Some((idx, i + 1, s));
                });

                if item.is_none() {
                    item = Some((
                        idx,
                        part.len(),
                        format!(
                            " [URL=\"https://open.spotify.com/playlist/{}\"]",
                            playlist_url
                        ),
                    ));
                }
            }
        }

        if let Some(item) = item {
            let (idx, i, s) = item;
            res_parts[idx].insert_str(i, &s);

            log::info!("Successfully added playlist url to node {:?}", node);
        } else {
            log::error!(
                "Could not find node {:?} in the graph and could not add the playlist url to it",
                node
            );
        }
    }

    return Ok(res_parts.join(";"));
}

fn parse_id_from_playlist_id(playlist_id: &rspotify::model::PlaylistId) -> String {
    playlist_id
        .to_string()
        .split(":")
        .last()
        .unwrap()
        .to_string()
}

fn to_local(s: &str) -> String {
    format!("local-{}", s)
}
