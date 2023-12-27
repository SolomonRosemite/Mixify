use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use futures_util::stream::StreamExt;

use rspotify::model::{
    ArtistId, FullAlbum, PlaylistId, PlaylistItem, SavedAlbum, SavedTrack, SimplifiedPlaylist,
    SimplifiedTrack, TrackId,
};
use rspotify::ClientError;
use rspotify::{
    prelude::{BaseClient, OAuthClient, PlayableId},
    AuthCodeSpotify,
};

use crate::types::{QuerySongsByArtist, Track, TrackTuple};
use crate::{constants, plan_command, traits::ResultExtension, types};

use super::args;

pub async fn handle_apply_snapshot(
    cmd: &args::ApplyCommand,
    spotify: &AuthCodeSpotify,
    allow_delete: bool,
    mixstack_suffix: String,
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

    // TODO: For better performance, maybe create a list of tracks and use refs in the map like
    let mut map: HashMap<String, Vec<TrackTuple>> = HashMap::new();
    let mut node_to_playlist_id: HashMap<String, String> = HashMap::new();
    let mut nodes_with_missing_playlists: Vec<String> = Vec::new();

    let mut albums: Vec<Result<SavedAlbum, ClientError>> = vec![];
    let mut playlists: Vec<SimplifiedPlaylist> = vec![];
    let mut liked_songs: Vec<Result<SavedTrack, ClientError>> = vec![];
    let mut is_cached = false;

    let mut all_songs: Vec<PlaylistItem> = vec![];
    let mut songs_are_cached = false;

    let user = spotify
        .current_user()
        .await
        .or_error_str("failed to fetch user")?;

    log::debug!("------------------");
    log::debug!("list of actions:");
    for actions in &all_actions {
        for action in actions {
            log::debug!("{}", action);

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
    log::debug!("------------------");

    for actions in all_actions {
        for action in actions {
            if action.for_node == constants::MIXIFY_TEMPORARY_ROOT_NODE_NAME {
                continue;
            }

            log::debug!("Applying action {:?}", action);

            match action.action_type {
                types::ActionType::CreatePlaylist => {
                    let node_index = graph
                        .node_indices()
                        .find(|i| graph[*i] == *action.node)
                        .unwrap();
                    let nei = graph.neighbors_directed(node_index, petgraph::Direction::Incoming);
                    let names = nei.map(|i| graph[i].clone()).collect::<Vec<String>>();
                    // TODO: Description should only be set if wanted. (Set in the env file)
                    // TODO: If the playlists does not have children, remomve the list of names from the description.
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
                            &format!("{}{}", playlist_name, mixstack_suffix),
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
                    let playlist_id =
                        PlaylistId::from_id(playlist_id_str.clone()).or_error(format!(
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

                                    Some(TrackTuple {
                                        id: track.id.unwrap(),
                                        name: track.name,
                                        album_name: track.album.name,
                                        artist_id: track.artists[0].id.clone().unwrap(),
                                    })
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
                    target.extend(tracks);
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

                    // Remove duplicates. (Including same songs although they may be in different albums)
                    local.sort_unstable_by_key(|item| (item.name.clone(), !item.is_single()));
                    local.dedup();

                    let mut songs_to_add = local.clone();
                    songs_to_add.retain(|t| !remote.contains(t));

                    let songs_to_add = songs_to_add
                        .into_iter()
                        .map(|t| t.id)
                        .collect::<HashSet<_>>();

                    let mut songs_to_remove =
                        remote.iter().map(|t| t.id.clone()).collect::<Vec<_>>();
                    let con_local = local.iter().map(|t| t.id.clone()).collect::<Vec<_>>();

                    songs_to_remove.retain(|t| !con_local.contains(t));

                    let playlist_id = node_to_playlist_id.get(&action.node).unwrap();
                    let playlist_id = PlaylistId::from_id(playlist_id).unwrap();

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

                        // TODO: Use playlist_remove_specific_occurrences_of_items instead.
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
                    // TODO: Playlists should be cached, on a database or something.
                    // TODO: Backup before starting to apply the snapshot. (Backup songs for each playlist).

                    let now = Instant::now();
                    if !is_cached {
                        log::info!(
                            "Fetching all songs from user library. Since there is no cache yet."
                        );
                        let now = Instant::now();
                        let fetched_albums = spotify.current_user_saved_albums(None);
                        albums = fetched_albums.collect::<Vec<_>>().await;
                        log::info!("Took {}ms to fetch all albums", now.elapsed().as_millis());

                        let now = Instant::now();
                        let fetched_liked_songs = spotify.current_user_saved_tracks(None);
                        liked_songs = fetched_liked_songs.collect::<Vec<_>>().await;
                        log::info!(
                            "Took {}ms to fetch all liked songs",
                            now.elapsed().as_millis()
                        );

                        let now = Instant::now();
                        let fetched_playlists = spotify.current_user_playlists();
                        let p = fetched_playlists.collect::<Vec<_>>().await;
                        log::info!(
                            "Took {}ms to fetch all playlists",
                            now.elapsed().as_millis()
                        );

                        for p in p {
                            if let Ok(p) = p {
                                let name = p.name.clone();
                                if name.ends_with(&mixstack_suffix) {
                                    log::warn!(
                                        "Skipping playlist {:?} because the playlist suffix indicates that it was generated by mixify. Suffix: {:?}",
                                        p.name, mixstack_suffix
                                    );
                                    continue;
                                }

                                playlists.push(p);
                                continue;
                            }

                            log::warn!("failed to fetch playlist {}", p.err().unwrap());
                        }
                    }

                    log::info!("Took {}ms to fetch all songs", now.elapsed().as_millis());

                    is_cached = true;
                    let mut tracks: Vec<TrackTuple> = vec![];

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

                            let name = t.as_ref().unwrap().clone().track.name;
                            let album_name = t.as_ref().unwrap().clone().track.album.name;
                            let t = t.as_ref().unwrap().clone().track;

                            if let Some(id) =
                                should_add_song(&t.into(), &artist_id, &q, &liked_songs)
                            {
                                tracks.push(TrackTuple {
                                    id: id.clone(),
                                    name,
                                    album_name,
                                    artist_id: artist_id.clone(),
                                });
                            }
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
                            for t in &a.tracks.items {
                                if let Some(id) = should_add_song(
                                    &simplified_track_to_track(t.clone(), &a),
                                    &artist_id,
                                    &q,
                                    &liked_songs,
                                ) {
                                    tracks.push(TrackTuple {
                                        id: id.clone(),
                                        name: t.name.clone(),
                                        album_name: a.name.clone(),
                                        artist_id: artist_id.clone(),
                                    });
                                }
                            }
                        }
                    }

                    if q.source.is_none()
                        || *q.source.as_ref().unwrap() == types::QuerySource::Playlists
                    {
                        if !songs_are_cached {
                            let mut playlists = playlists
                                .iter()
                                .map(|p| {
                                    let p = p.clone();
                                    (parse_id_from_playlist_id(&p.id), p.name)
                                })
                                .collect::<Vec<_>>();

                            let total = Instant::now();
                            let (success, failed) =
                                fetch_songs_from_playlists(&spotify, playlists.clone()).await;
                            all_songs.extend(success);

                            playlists.retain(|(id, _)| failed.contains(id));

                            if failed.len() != 0 {
                                log::warn!(
                                    "Failed to fetch songs from the {} playlists, {:?}. Trying again in 30s...",
                                    failed.len(),
                                    playlists.iter().map(|(_, name)| name.clone()).collect::<Vec<_>>().join(", ")
                                );

                                tokio::time::sleep(Duration::from_secs(30)).await;

                                let (success, failed) =
                                    fetch_songs_from_playlists(&spotify, playlists.clone()).await;
                                all_songs.extend(success);

                                playlists.retain(|(id, _)| !failed.contains(id));
                                if failed.len() != 0 {
                                    log::warn!(
                                        "Failed to fetch songs from the {} playlists, {:?}.",
                                        failed.len(),
                                        playlists
                                            .iter()
                                            .map(|(_, name)| name.clone())
                                            .collect::<Vec<_>>()
                                            .join(", ")
                                    );
                                }
                            }

                            log::info!(
                                "Took {}s to fetch uncached songs from.",
                                total.elapsed().as_secs(),
                            );
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
                                let name = t.name.clone();
                                let album_name = t.album.name.clone();

                                if let Some(id) =
                                    should_add_song(&t.into(), &artist_id, &q, &liked_songs)
                                {
                                    tracks.push(TrackTuple {
                                        id: id.clone(),
                                        album_name,
                                        name,
                                        artist_id: artist_id.clone(),
                                    });
                                }
                            });
                    }

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

fn parse_id_from_playlist_id(playlist_id: &PlaylistId) -> String {
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
// In which all the artists are listed. (This also works if the albums says "Various Artists")
// Example here: https://open.spotify.com/album/0mDeN57X1YtJHfXNdYlJbw
fn is_main_artist(a: &Vec<ArtistId>, artist_id: &ArtistId) -> bool {
    a.iter().any(|a| a == artist_id)
}

fn should_add_song(
    t: &Track,
    artist_id: &ArtistId,
    q: &QuerySongsByArtist,
    liked_songs: &Vec<Result<SavedTrack, ClientError>>,
) -> Option<TrackId<'static>> {
    if t.is_local {
        log::warn!("Skipping local track {}", t.name);
        return None;
    }

    let is_main_artist = is_main_artist(&t.album_artists_ids, &artist_id);
    let is_in_song = t
        .artists
        .iter()
        .any(|a| a.id.clone().unwrap() == *artist_id);

    if !is_main_artist && !is_in_song {
        return None;
    }

    let skip: bool = match q.include_features {
        Some(should_not_be_main) => {
            if should_not_be_main {
                is_main_artist
            } else {
                !is_main_artist
            }
        }
        None => false,
    };

    let id = t.id.as_ref().unwrap();

    if skip {
        return None;
    } else if let Some(v) = q.must_be_liked {
        let found = liked_songs.iter().any(|tra| {
            let track = tra.as_ref().unwrap().clone().track;
            track.id.unwrap() == *id
        });

        if v {
            if !found {
                return None;
            }
        } else {
            if found {
                return None;
            }
        }
    }

    return Some(id.clone());
}

fn simplified_track_to_track(t: SimplifiedTrack, album: &FullAlbum) -> Track {
    Track {
        id: t.id,
        name: t.name,
        is_local: t.is_local,
        artists: t.artists,
        album_artists_ids: album
            .artists
            .iter()
            .map(|artist| artist.clone().id.unwrap())
            .collect::<Vec<_>>(),
    }
}

async fn process_fetch_playlist_songs_chunks(
    spotify: AuthCodeSpotify,
    chunks: Vec<(String, String)>,
) -> (Vec<PlaylistItem>, Vec<String>, usize, usize) {
    let mut p = Vec::new();
    let mut failed_req = Vec::new();
    let mut success = 0;
    let mut failed = 0;

    for (id, name) in chunks {
        let songs = fetch_songs_from_playlist(&spotify, id, name).await;

        match songs {
            Ok(content) => {
                success += 1;
                p.extend(content)
            }
            Err((err, id)) => {
                log::warn!(
                    "Failed to fetch songs from playlist {:?}. Error: {:?}. Sleeping for 10s to prevent rate limit...",
                    id,
                    err
                );
                failed += 1;
                tokio::time::sleep(Duration::from_secs(10)).await;
                failed_req.push(id)
            }
        }
    }

    return (p, failed_req, success, failed);
}

async fn fetch_songs_from_playlist(
    spotify: &AuthCodeSpotify,
    id: String,
    name: String,
) -> Result<Vec<PlaylistItem>, (ClientError, String)> {
    let id = PlaylistId::from_id(id).unwrap();

    let single = Instant::now();
    let items = spotify.playlist_items(id.clone(), None, None);
    let songs = items.collect::<Vec<_>>().await;

    if songs.iter().any(|t| t.is_err()) {
        let err = songs.into_iter().find(|t| t.is_err()).unwrap().unwrap_err();
        return Err((err, parse_id_from_playlist_id(&id)));
    }

    log::info!(
        "Took {}ms to fetch songs from playlist {}",
        single.elapsed().as_millis(),
        name
    );

    let total_len = songs.len();
    let songs = songs
        .into_iter()
        .filter_map(|r| match r {
            Ok(s) => Some(s),
            _ => None,
        })
        .collect::<Vec<_>>();

    if total_len != songs.len() {
        log::warn!(
            "Playlist {} has {} songs and {} songs could not be fetched and will be skipped...",
            name,
            songs.len(),
            total_len - songs.len()
        );
    }

    Ok(songs)
}

async fn fetch_songs_from_playlists(
    spotify: &AuthCodeSpotify,
    playlists: Vec<(String, String)>,
) -> (Vec<PlaylistItem>, Vec<String>) {
    let total = Instant::now();

    let num_of_playlists = playlists.len();
    let chunks: Vec<Vec<_>> = playlists
        .clone()
        .chunks(num_of_playlists / 5)
        .into_iter()
        .map(|chunk| chunk.clone().to_vec())
        .collect();

    let chunks = chunks.clone();

    log::info!(
        "Starting to fetch songs from spotify. Ids {:?}.",
        playlists
            .into_iter()
            .map(|e| e.0)
            .collect::<Vec<String>>()
            .join(", "),
    );

    log::info!(
        "Splitting {} playlists into {} chunks",
        num_of_playlists,
        chunks.len()
    );

    let mut handles = Vec::new();
    for chunk in chunks {
        log::info!("Spawning thread to fetch {} playlists", chunk.len());
        handles.push(tokio::spawn(process_fetch_playlist_songs_chunks(
            spotify.clone(),
            chunk,
        )));
    }

    let mut success = vec![];
    let mut failed = vec![];
    let mut num_of_success = 0;
    let mut num_of_failure = 0;
    for handle in handles {
        let res = handle.await.unwrap();

        match res {
            (p, f, ns, nf) => {
                num_of_success += ns;
                num_of_failure += nf;

                success.extend(p);
                failed.extend(f);
            }
        }
    }

    let total_len = num_of_failure + num_of_success;
    log::info!(
        "Took {}s to fetch songs. Total num of playlists: {:?} of which {} failed and {} succeeded. Success rate: {}%",
        total.elapsed().as_secs(),
        total_len,
        num_of_failure,
        num_of_success,
        (num_of_success as f32 / total_len as f32) * 100.0
    );

    return (success, failed);
}
