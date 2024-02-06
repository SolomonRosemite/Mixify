use std::{collections::HashMap, fs, io};

use anyhow::anyhow;
use graphviz_dot_parser::types::{GraphAST, Stmt};
use url::Url;

use crate::{
    constants,
    traits::ResultExtension,
    types::{Action, ActionType, QuerySongsByArtist, QuerySource},
};

use super::args;

type EdgeData = (String, String, graphviz_dot_parser::types::Attributes);
type NodeData = (String, graphviz_dot_parser::types::Attributes);

pub fn handle_plan_snapshot(cmd: &args::PlanCommand) -> Result<(), anyhow::Error> {
    let content = match read_snapshot_file(cmd.id, "edit") {
        Ok(v) => v,
        Err(err) => {
            log::warn!("failed to find edit snapshot. see error: {:?}", err);
            log::info!("trying to find post snapshot instead");

            read_snapshot_file(cmd.id, "post.apply").or_error(format!(
                "failed to find post snapshot. maybe this id {} doesn't exist?.",
                cmd.id
            ))?
        }
    };

    let gv =
        graphviz_dot_parser::parse(&content).or_error(String::from("failed to parse graph"))?;
    let (res, _) = create_execution_plan(&gv)?;

    for actions in res {
        let mut idx = 0;
        for action in actions {
            if idx != action.idx {
                log::info!("------------------------------------");
                idx = action.idx;
            }

            log::info!("{}", action);
        }
    }

    return Ok(());
}

pub fn create_execution_plan(
    gv: &GraphAST,
) -> Result<(Vec<Vec<Action>>, Vec<NodeData>), anyhow::Error> {
    let mixify_root_node = (
        constants::MIXIFY_TEMPORARY_ROOT_NODE_NAME.to_string(),
        vec![],
    );

    let mut nodes: Vec<NodeData> = vec![mixify_root_node];
    let mut edges: Vec<EdgeData> = Vec::new();
    let mut root_nodes: Vec<String> = Vec::new();

    // NOTE: Important for this to work. All nodes must be defined in the graph. Otherwise it will panic.
    // In other words, edges that dont point to a node explicitly defined in the graph will cause a panic.
    // This is why we validate the graph before we do anything else.
    validate_graph(&gv)?;
    let mut graph = gv.to_directed_graph().unwrap();

    let mut error: Option<anyhow::Error> = None;
    gv.stmt.iter().for_each(|stmt| match stmt {
        Stmt::Edge(from, to, attrs) => {
            edges.push((from.to_string(), to.to_string(), attrs.clone()));
        }
        Stmt::Node(node, attrs) => {
            let node_index = graph.node_indices().find(|i| graph[*i] == *node).unwrap();
            let number_of_outgoing_edges = graph
                .neighbors_directed(node_index, petgraph::Direction::Outgoing)
                .count();
            let number_of_incoming_edges = graph
                .neighbors_directed(node_index, petgraph::Direction::Incoming)
                .count();

            if number_of_outgoing_edges == 0 {
                root_nodes.push(node.to_string());
            }

            // Base nodes, that are not of type query, should have a spotify url attribute.
            if number_of_incoming_edges == 0 && number_of_outgoing_edges > 0 {
                let attr = attrs
                    .iter()
                    .find(|(k, _)| k == constants::URL_ATTRIBUTE_KEY);

                if attr.is_none() {
                    let is_query = attr
                        .iter()
                        .any(|(k, v)| k == constants::TYPE_ATTRIBUTE_KEY && v == "query");

                    if is_query {
                        error = Some(anyhow!(
                            "Node {:?} is a base node and should have a spotify url attribute",
                            node
                        ));
                    }
                } else {
                    let (_, url) = attr.unwrap();
                    let _ = Url::parse(url).expect(&format!(
                        "the url attribute of {:?} is not a valid url",
                        node
                    ));
                }
            }

            nodes.push((node.to_string(), attrs.clone()));
        }
        _ => {}
    });

    if let Some(e) = error {
        return Err(e);
    }

    log::debug!("nodes: {:?}", nodes);

    let root = constants::MIXIFY_TEMPORARY_ROOT_NODE_NAME.to_string();
    let idx = graph.add_node(root.clone());
    for root in &root_nodes {
        graph.add_edge(
            graph.node_indices().find(|i| graph[*i] == *root).unwrap(),
            idx,
            (),
        );
    }

    let mut all_actions: Vec<Vec<Action>> = Vec::new();
    let mut memo: Vec<String> = Vec::new();
    let res = create_node_execution_plan(1, &root, &nodes, &edges, &graph, &mut memo)?;
    all_actions.push(res);

    return Ok((all_actions, nodes.clone()));
}

fn create_node_execution_plan(
    idx: usize,
    current_node: &String,
    nodes: &Vec<NodeData>,
    edges: &Vec<EdgeData>,
    graph: &petgraph::Graph<String, ()>,
    playlists_created_memo: &mut Vec<String>,
) -> Result<Vec<Action>, anyhow::Error> {
    let mut actions: Vec<Action> = Vec::new();

    let node_index = graph
        .node_indices()
        .find(|i| graph[*i] == *current_node)
        .unwrap();
    let nei = graph.neighbors_directed(node_index, petgraph::Direction::Incoming);
    let names = nei.map(|i| graph[i].clone()).collect::<Vec<String>>();
    let has_neighbors = names.len() > 0;

    let mut edges_with_subtraction: Vec<&EdgeData> = Vec::new();
    let mut is_query_node = false;

    let mut final_node_actions: Vec<Action> = Vec::new();
    for from_node in &names {
        let subtraction_edge = edges.iter().find(|(from, to, attr)| {
            *from == *from_node
                && *to == *current_node
                && attr
                    .iter()
                    .any(|(k, v)| k == constants::SUBTRACT_ATTRIBUTE_KEY && v == "true")
        });

        // Edges with subtraction are being done at tht end.
        if let Some(v) = subtraction_edge {
            edges_with_subtraction.push(v);
            continue;
        }

        let r = create_node_execution_plan(
            idx + 1,
            from_node,
            nodes,
            edges,
            graph,
            playlists_created_memo,
        )?;
        for action in r {
            actions.push(action);
        }

        final_node_actions.push(Action {
            action_type: ActionType::CopySongs,
            playlist_url: get_playlist_url(nodes, from_node),
            node: from_node.to_string(),
            idx,
            for_node: current_node.clone(),
        });
    }

    for (n, _, _) in edges_with_subtraction {
        let r =
            create_node_execution_plan(idx + 1, &n, nodes, edges, graph, playlists_created_memo)?;
        for action in r {
            actions.push(action);
        }

        let action = Action {
            action_type: ActionType::RemoveSongs,
            node: n.clone(),
            idx,
            for_node: current_node.clone(),
            playlist_url: get_playlist_url(nodes, n),
        };
        final_node_actions.push(action);
    }

    let (_, attr) = nodes
        .iter()
        .find(|(name, _)| *name == *current_node)
        .unwrap();
    let playlist_already_exists = attr.iter().find(|(k, _)| k == constants::URL_ATTRIBUTE_KEY);

    if !has_neighbors {
        let is_query = attr
            .iter()
            .any(|(k, v)| k == constants::TYPE_ATTRIBUTE_KEY && v == "query");

        if !is_query {
            if playlist_already_exists.is_none() {
                return Err(anyhow!(
                    "Node {:?} is a base node and should have a spotify url attribute",
                    current_node
                ));
            }
        } else {
            let must_be_liked = attr
                .iter()
                .find(|(k, _)| k == constants::MUST_BE_LIKED_ATTRIBUTE_KEY)
                .map(|(_, v)| v.as_str().parse::<bool>());

            let must_be_liked = match must_be_liked {
                Some(Ok(v)) => Some(v),
                Some(Err(e)) => {
                    return Err(anyhow!(
                        "Failed to parse must_be_liked attribute of node {:?} with error: {:?}",
                        current_node,
                        e
                    ));
                }
                None => None,
            };

            let artist_id = attr
                .iter()
                .find(|(k, _)| k == constants::ARTIST_ID_ATTRIBUTE_KEY)
                .map(|(_, v)| v.clone());

            let artist_id = match artist_id {
                Some(v) => v,
                None => {
                    return Err(anyhow!(
                        "Node {:?} is a query node and should have a artist_id attribute",
                        current_node
                    ));
                }
            };

            let include_features = attr
                .iter()
                .find(|(k, _)| k == constants::INCLUDE_FEATURES_ATTRIBUTE_KEY)
                .map(|(_, v)| v.as_str().parse::<bool>());

            let include_features = match include_features {
                Some(Ok(v)) => Some(v),
                Some(Err(e)) => {
                    return Err(anyhow!(
                        "Failed to parse include_features attribute of node {:?} with error: {:?}",
                        current_node,
                        e
                    ));
                }
                None => None,
            };

            let source = attr
                .iter()
                .find(|(k, _)| k == constants::SOURCE_ATTRIBUTE_KEY)
                .map(|(_, v)| v.as_str().parse::<QuerySource>());

            let source = match source {
                Some(Ok(v)) => Some(v),
                Some(Err(e)) => {
                    return Err(anyhow!(
                        "Failed to parse source attribute of node {:?} with error: {:?}",
                        current_node,
                        e
                    ));
                }
                None => None,
            };

            let query = QuerySongsByArtist {
                artist_id,
                include_features,
                source,
                must_be_liked,
            };

            let url = playlist_already_exists.map(|(_, url)| url.clone());
            final_node_actions.push(Action {
                action_type: ActionType::QuerySongsByArtist(query),
                node: current_node.clone(),
                idx,
                for_node: current_node.clone(),
                playlist_url: url.clone(),
            });

            is_query_node = true;
        }
    }

    if let Some((_, url)) = playlist_already_exists {
        // We always query because we want the latest state of the playlist.
        final_node_actions.push(Action {
            action_type: ActionType::QuerySongs(Some(url.clone())),
            node: current_node.clone(),
            idx,
            for_node: current_node.clone(),
            playlist_url: Some(url.clone()),
        });

        if has_neighbors || is_query_node {
            final_node_actions.push(Action {
                action_type: ActionType::SaveChanges(Some(url.clone())),
                node: current_node.clone(),
                idx,
                for_node: current_node.clone(),
                playlist_url: Some(url.clone()),
            });
        }
    } else {
        if !playlists_created_memo.iter().any(|v| v == current_node) {
            playlists_created_memo.push(current_node.clone());

            actions.push(Action {
                action_type: ActionType::CreatePlaylist,
                node: current_node.clone(),
                idx,
                for_node: current_node.clone(),
                playlist_url: None,
            });

            final_node_actions.push(Action {
                action_type: ActionType::SaveChanges(None),
                node: current_node.clone(),
                idx,
                for_node: current_node.clone(),
                playlist_url: None,
            });
        }
    }

    for action in final_node_actions {
        actions.push(action);
    }

    return Ok(actions);
}

fn validate_graph(graph: &GraphAST) -> Result<(), anyhow::Error> {
    let mut nodes: Vec<String> = Vec::new();
    let mut nodes_from_edges: Vec<String> = Vec::new();

    graph.stmt.iter().for_each(|stmt| match stmt {
        Stmt::Edge(from, to, _) => {
            if !nodes_from_edges.contains(from) {
                nodes_from_edges.push(from.to_string());
            }
            if !nodes_from_edges.contains(to) {
                nodes_from_edges.push(to.to_string());
            }
        }
        Stmt::Node(node, _) => {
            nodes.push(node.to_string());
        }
        _ => {}
    });

    for node in nodes_from_edges {
        if !nodes.contains(&node) {
            let error = anyhow::anyhow!(
                "Node {:?} is used for an edge but not defined as node in the graph file. Please define it. It should look like this: {}",
                node, format!("{} [label={:?}];", node, "a playlist name of your choice"));

            return Err(error);
        }
    }

    return Ok(());
}

pub fn read_snapshot_file(id: u32, suffix: &str) -> Result<String, anyhow::Error> {
    let data = list_snapshot_content(id, suffix)?;

    let directory_path = format!("snapshots/{}/", id);
    if data.len() == 0 {
        return Err(anyhow::anyhow!(
            "No *.{}.gv file found in {} folder",
            suffix,
            &directory_path
        ));
    }

    if data.len() > 1 {
        return Err(anyhow::anyhow!(
            "More than one *.{}.gv file found in {} folder. Expected only one since mixify doesn't know which one to use.",
            suffix,
            &directory_path
        ));
    }

    let content = data.first().unwrap().as_ref().unwrap().clone();
    return Ok(content);
}

pub fn list_snapshot_content(
    id: u32,
    suffix: &str,
) -> Result<Vec<io::Result<String>>, anyhow::Error> {
    let full_suffix = format!("{}.gv", suffix);
    let data = list_snapshot_files(id, suffix)?
        .iter()
        .filter(|path| path.is_file() && path.to_str().unwrap().ends_with(full_suffix.as_str()))
        .map(|file_path| fs::read_to_string(file_path))
        .collect::<Vec<io::Result<String>>>();

    return Ok(data);
}

pub fn list_snapshot_files(
    id: u32,
    suffix: &str,
) -> Result<Vec<std::path::PathBuf>, anyhow::Error> {
    let directory_path = format!("snapshots/{}/", id);
    log::info!(
        "checking directory: {} for *.{}.gv snapshot",
        directory_path,
        suffix
    );

    let data = std::fs::read_dir(&directory_path)
        .or_error(format!(
            "failed to find snapshot folder: {}. Maybe it's another id?",
            directory_path
        ))?
        .map(|entry| entry.unwrap().path().canonicalize().unwrap())
        .collect::<Vec<std::path::PathBuf>>();

    return Ok(data);
}

fn get_playlist_url(nodes: &Vec<NodeData>, node: &String) -> Option<String> {
    let (_, attr) = nodes.iter().find(|(name, _)| *name == *node).unwrap();
    return attr
        .iter()
        .find(|(k, _)| k == constants::URL_ATTRIBUTE_KEY)
        .map(|(_, url)| url.clone());
}
