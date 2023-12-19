use std::collections::{HashMap, HashSet};

use futures_util::stream::StreamExt;

use rspotify::model::{
    ArtistId, PlaylistItem, SavedAlbum, SavedTrack, SimplifiedArtist, SimplifiedPlaylist, TrackId,
};
use rspotify::ClientError;
use rspotify::{
    prelude::{BaseClient, OAuthClient, PlayableId},
    AuthCodeSpotify,
};

use crate::{constants, plan_command, traits::ResultExtension, types};

use super::args;

pub async fn handle_apply_snapshot(
    cmd: &args::ApplyCommand,
    spotify: &AuthCodeSpotify,
    allow_delete: bool,
    is_sync: bool,
) -> Result<(), anyhow::Error> {
    let file_suffix = match is_sync {
        true => {
            log::info!("Syncing snapshot {}", cmd.id);
            "post.apply"
        }
        false => {
            log::info!("Applying snapshot {}", cmd.id);
            "edit"
        }
    };

    let content = plan_command::read_snapshot_file(cmd.id, file_suffix)?;
    let gv =
        graphviz_dot_parser::parse(&content).or_error(String::from("failed to parse graph"))?;
    let graph = gv.to_directed_graph().unwrap();
    let (all_actions, nodes) = plan_command::create_execution_plan(&gv)?;

    // TODO:For better performance, maybe create a list of tracks and use refs in the map like
    // let mut map: HashMap<String, Vec<&rspotify::model::FullTrack>> = HashMap::new();
    let mut map: HashMap<String, Vec<TrackId>> = HashMap::new();
    let mut node_to_playlist_id: HashMap<String, String> = HashMap::new();
    let mut nodes_with_missing_playlists: Vec<String> = Vec::new();

    let mut albums: Vec<Result<SavedAlbum, ClientError>> = vec![];
    let mut playlists: Vec<Result<SimplifiedPlaylist, ClientError>> = vec![];
    let mut liked_songs: Vec<Result<SavedTrack, ClientError>> = vec![];
    let mut is_cached = false;

    let mut all_songs: Vec<PlaylistItem> = vec![];
    let mut songs_are_cached = false;

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

            log::info!("Applying action {:?}", action);

            match action.action_type {
                types::ActionType::CreatePlaylist => {
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
                types::ActionType::QuerySongs(url) => {
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

                    let playlist = spotify.playlist_items(playlist_id.clone(), None, None);
                    let songs = playlist.collect::<Vec<_>>().await;

                    let tracks = songs
                        .into_iter()
                        .filter_map(|t| {
                            let item = t.or_error(format!(
                                "could not work with a song from the playlist id of {}",
                                playlist_id_str
                            ));

                            if let Err(e) = item {
                                log::warn!("{}", e);
                                return None;
                            }
                            match item.unwrap().track.unwrap() {
                                rspotify::model::PlayableItem::Track(track) => {
                                    if track.is_local {
                                        log::warn!(
                                            "Skipping local track {} from playlist {}",
                                            track.name,
                                            playlist_id_str
                                        );
                                        return None;
                                    }

                                    Some(track.id.unwrap())
                                }
                                rspotify::model::PlayableItem::Episode(e) => {
                                    log::warn!("Skipping episode {:?}", e);
                                    None
                                }
                            }
                        })
                        .collect::<Vec<_>>();

                    map.insert(action.node.clone(), tracks.clone());

                    let has_songs = map.get(&to_local(&action.node)).unwrap().len() > 0;
                    if !has_songs {
                        map.insert(to_local(&action.node), tracks);
                    }
                }
                types::ActionType::CopySongs => {
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
                types::ActionType::RemoveSongs => {
                    let remote = map.get(&action.node).unwrap().clone();
                    let local = map.get_mut(&to_local(&action.for_node)).unwrap();
                    local.retain(|t| !remote.contains(t));
                }
                types::ActionType::SaveChanges(_) => {
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
                            .into_iter()
                            .map(|t| PlayableId::Track(t))
                            .collect::<Vec<rspotify::model::PlayableId>>();

                        // TODO: If too many, spotify will return an error.
                        // We should chunk the requests.
                        // Do the same for the remove action.

                        let mut chunks = ids.chunks(100);
                        while let Some(chunk) = chunks.next() {
                            let items = chunk.iter().map(|y| y.clone_static()).collect::<Vec<_>>();

                            let res = spotify
                                .playlist_add_items(playlist_id.clone(), items, None)
                                .await;
                            if let Err(e) = res {
                                return Err(anyhow::anyhow!(
                                    "Failed to add songs to playlist {:?}",
                                    e
                                ));
                            }
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
                            .into_iter()
                            .map(|t| PlayableId::Track(t))
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
                types::ActionType::QuerySongsByArtist(q) => {
                    if !is_cached {
                        let fetched_albums = spotify.current_user_saved_albums(None);
                        let a = fetched_albums.collect::<Vec<_>>().await;
                        albums = a;

                        let fetched_playlists = spotify.current_user_playlists();
                        let p = fetched_playlists.collect::<Vec<_>>().await;
                        playlists = p;

                        let fetched_liked_songs = spotify.current_user_saved_tracks(None);
                        let ls = fetched_liked_songs.collect::<Vec<_>>().await;
                        liked_songs = ls;
                    }

                    is_cached = true;
                    let mut tracks = vec![];

                    let artist_id =
                        rspotify::model::ArtistId::from_id(q.artist_id.clone()).or_error(
                            format!("failed to parse artist id correctly from {}", q.artist_id),
                        )?;

                    if (q.source.is_none()
                        || *q.source.as_ref().unwrap() == types::QuerySource::LikedSongs)
                        || (q.must_be_liked.is_none() || q.must_be_liked.unwrap_or(false))
                    {
                        liked_songs.iter().for_each(|t| {
                            if let Err(e) = t {
                                let x = format!("failed to fetch liked song {:?}", e);
                                panic!("{}", x);
                            }

                            let t = t.as_ref().unwrap().clone().track;

                            if t.is_local {
                                log::warn!("Skipping local track {} from liked songs", t.name,);
                                return;
                            }

                            let is_main_artist = is_main_artist(&t.artists, &artist_id);
                            if !is_main_artist {
                                return;
                            }

                            let skip: bool = match q.include_features {
                                Some(v) => {
                                    if v {
                                        if is_main_artist {
                                            true
                                        } else {
                                            false
                                        }
                                    } else {
                                        if is_main_artist {
                                            false
                                        } else {
                                            true
                                        }
                                    }
                                }
                                None => false,
                            };

                            if skip {
                                return;
                            }

                            tracks.push(t.id.unwrap());
                        });
                    }

                    if q.source.is_none()
                        || *q.source.as_ref().unwrap() == types::QuerySource::Albums
                    {
                        let res = albums
                            .iter()
                            .map(|a| a.as_ref().unwrap().clone().album)
                            .inspect(|a| {
                                if a.tracks.items.len() as u32 != a.tracks.total {
                                    log::error!(
                                        "album {:?} has {} songs but the total is {}. exiting...",
                                        a.name,
                                        a.tracks.items.len(),
                                        a.tracks.total
                                    );
                                    panic!();
                                }
                            })
                            .collect::<Vec<_>>();

                        for a in res {
                            for t in a.tracks.items {
                                let is_main_artist = is_main_artist(&a.artists, &artist_id);
                                if !is_main_artist {
                                    continue;
                                }

                                let skip: bool = match q.include_features {
                                    Some(v) => {
                                        if v {
                                            if is_main_artist {
                                                true
                                            } else {
                                                false
                                            }
                                        } else {
                                            if is_main_artist {
                                                false
                                            } else {
                                                true
                                            }
                                        }
                                    }
                                    None => false,
                                };

                                let id = t.id.unwrap();

                                if skip {
                                    continue;
                                } else if let Some(v) = q.must_be_liked {
                                    let found = liked_songs.iter().any(|tra| {
                                        let tra = tra.as_ref().unwrap().clone().track;
                                        tra.id.unwrap() == id
                                    });

                                    if v {
                                        if !found {
                                            continue;
                                        }
                                    } else {
                                        if found {
                                            continue;
                                        }
                                    }
                                }

                                tracks.push(id);
                            }
                        }
                    }

                    if q.source.is_none() || q.source.unwrap() == types::QuerySource::Playlists {
                        if !songs_are_cached {
                            let playlists = playlists
                                .iter()
                                .map(|p| {
                                    let p = p.as_ref().unwrap().clone();
                                    (p.id, p.name)
                                })
                                .collect::<Vec<_>>();

                            for (id, name) in playlists {
                                let x = spotify.playlist_items(id, None, None);
                                let songs = x.collect::<Vec<_>>().await;

                                let total_len = songs.len();

                                let songs = songs
                                    .into_iter()
                                    .filter_map(|r| match r {
                                        Ok(s) => Some(s),
                                        Err(e) => {
                                            log::warn!("failed to fetch song {:?}", e);
                                            None
                                        }
                                    })
                                    .collect::<Vec<_>>();

                                if total_len != songs.len() {
                                    log::warn!(
                                    "Playlist {} has {} songs and {} songs could not be fetched and will be skipped...",
                                    name,
                                    songs.len(),
                                    total_len - songs.len(),
                                );
                                }

                                all_songs.extend(songs);
                            }
                            songs_are_cached = true;
                        }

                        all_songs
                            .iter()
                            .filter_map(|t| {
                                if t.track.is_none() {
                                    log::warn!("Skipping song {:?}", t);
                                    return None;
                                }

                                match t.track.clone().unwrap() {
                                    rspotify::model::PlayableItem::Track(track) => Some(track),
                                    rspotify::model::PlayableItem::Episode(e) => {
                                        log::warn!("Skipping episode {:?}", e);
                                        None
                                    }
                                }
                            })
                            .for_each(|t| {
                                if t.is_local {
                                    log::warn!("Skipping local track {}", t.name,);
                                    return;
                                }

                                let is_main_artist = is_main_artist(&t.album.artists, &artist_id);
                                if !is_main_artist {
                                    return;
                                }

                                let skip: bool = match q.include_features {
                                    Some(v) => {
                                        if v {
                                            if is_main_artist {
                                                true
                                            } else {
                                                false
                                            }
                                        } else {
                                            if is_main_artist {
                                                false
                                            } else {
                                                true
                                            }
                                        }
                                    }
                                    None => false,
                                };

                                let id = t.id.unwrap();

                                if skip {
                                    return;
                                } else if let Some(v) = q.must_be_liked {
                                    let found = liked_songs.iter().any(|tra| {
                                        let tra = tra.as_ref().unwrap().clone().track;
                                        tra.id.unwrap() == id
                                    });

                                    if v {
                                        if !found {
                                            return;
                                        }
                                    } else {
                                        if found {
                                            return;
                                        }
                                    }
                                }

                                tracks.push(id);
                            });
                    }

                    // TODO: There is a chance that there are duplicates but of different ids.
                    // This could happen if the same song is in two albums. (A single and an album for example)
                    // However the external_ids.isrc field should still match. And used to remove duplicates.
                    let tracks = tracks
                        .into_iter()
                        .collect::<HashSet<_>>()
                        .into_iter()
                        .collect::<Vec<_>>();

                    map.insert(to_local(&action.node), tracks);
                }
            }
        }
    }

    log::info!("Successfully applied snapshot");

    if is_sync {
        return Ok(());
    }

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
    let pfx = content
        .chars()
        .take_while(|&ch| ch != '{')
        .collect::<String>();

    let sfx2 = content
        .chars()
        .skip_while(|&ch| ch != '}')
        .skip(1)
        .collect::<String>();

    let inner = content
        .chars()
        .skip_while(|&ch| ch != '{')
        .skip(1)
        .take_while(|&ch| ch != '}')
        .collect::<String>();

    let parts = inner
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

    let new_content = format!("{}{}{}{}{}", pfx, "{", res_parts.join(";"), "}", sfx2);
    return Ok(new_content);
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

// Sometimes an artist is a combination of multiple artists.
// Like baby gravy or Huncho Jack.
// In these cases, we can check the owner(s) of the album.
// In which all the artists are listed. (With the exception of the "Various Artists")
// Example here: https://open.spotify.com/album/0mDeN57X1YtJHfXNdYlJbw
fn is_main_artist(a: &Vec<SimplifiedArtist>, artist_id: &ArtistId) -> bool {
    a.iter().any(|a| a.id.clone().unwrap() == *artist_id)
}
