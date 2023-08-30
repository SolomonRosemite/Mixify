use std::{fs, io};

use anyhow::{anyhow, Ok};
use graphviz_dot_parser::types::{GraphAST, Stmt};
use url::Url;

use crate::{constants, traits::ResultExtension};

use super::args;

type EdgeData = (String, String, graphviz_dot_parser::types::Attributes);
type NodeData = (String, graphviz_dot_parser::types::Attributes);

#[derive(Debug)]
pub struct Action {
    action_type: ActionType,
    node: String,
    for_node: String,
    idx: usize,
}

#[derive(Debug)]
enum ActionType {
    CreatePlaylist,
    QuerySongsPlaylist,
    CopySongs,
    RemoveSongs,
}

pub fn handle_plan_snapshot(cmd: &args::PlanCommand) -> Result<(), anyhow::Error> {
    let gv = read_snapshot(cmd.id, "edit")?;
    let res = create_execution_plan(&gv)?;

    for actions in res {
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

    return Ok(());
}

pub fn create_execution_plan(gv: &GraphAST) -> Result<Vec<Vec<Action>>, anyhow::Error> {
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

            // All base nodes should have a spotify url attribute.
            if number_of_incoming_edges == 0 && number_of_outgoing_edges > 0 {
                let attr = attrs
                    .iter()
                    .find(|(k, _)| k == constants::URL_ATTRIBUTE_KEY);

                if attr.is_none() {
                    error = Some(anyhow!(
                        "Node {:?} is a base node and should have a spotify url attribute",
                        node
                    ));
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
    let res = create_node_execution_plan(1, &root, &nodes, &edges, &graph);
    all_actions.push(res);

    return Ok(all_actions);
}

fn create_node_execution_plan(
    idx: usize,
    node: &String,
    nodes: &Vec<NodeData>,
    edges: &Vec<EdgeData>,
    graph: &petgraph::Graph<String, ()>,
) -> Vec<Action> {
    let mut actions: Vec<Action> = Vec::new();

    let node_index = graph.node_indices().find(|i| graph[*i] == *node).unwrap();
    let nei = graph.neighbors_directed(node_index, petgraph::Direction::Incoming);
    let names = nei.map(|i| graph[i].clone()).collect::<Vec<String>>();

    let mut edges_with_subtraction: Vec<&EdgeData> = Vec::new();

    let mut final_node_actions: Vec<Action> = Vec::new();
    for n in names {
        let subtraction_edge = edges.iter().find(|(from, to, attr)| {
            *from == *n
                && *to == *node
                && attr
                    .iter()
                    .any(|(k, v)| k == constants::SUBTRACT_ATTRIBUTE_KEY && v == "true")
        });

        // Edges with subtraction are being done at tht end.
        if let Some(v) = subtraction_edge {
            edges_with_subtraction.push(v);
            continue;
        }

        let r = create_node_execution_plan(idx + 1, &n, nodes, edges, graph);
        for action in r {
            actions.push(action);
        }

        let action = Action {
            action_type: ActionType::CopySongs,
            node: n,
            idx,
            for_node: node.clone(),
        };
        final_node_actions.push(action);
    }

    for (n, _, _) in edges_with_subtraction {
        let r = create_node_execution_plan(idx + 1, &n, nodes, edges, graph);
        for action in r {
            actions.push(action);
        }

        let action = Action {
            action_type: ActionType::RemoveSongs,
            node: n.clone(),
            idx,
            for_node: node.clone(),
        };
        final_node_actions.push(action);
    }

    let (_, attr) = nodes.iter().find(|(name, _)| *name == *node).unwrap();
    let playlist_already_exists = attr.iter().any(|(k, _)| k == constants::URL_ATTRIBUTE_KEY);

    let action = Action {
        action_type: ActionType::QuerySongsPlaylist,
        node: node.clone(),
        idx,
        for_node: node.clone(),
    };

    if !playlist_already_exists {
        actions.push(Action {
            action_type: ActionType::CreatePlaylist,
            node: node.clone(),
            idx,
            for_node: node.clone(),
        });
    }

    for action in final_node_actions {
        actions.push(action);
    }

    actions.push(action);
    return actions;
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

pub fn read_snapshot(id: u32, suffix: &str) -> Result<GraphAST, anyhow::Error> {
    let directory_path = format!("snapshots/{}/", id);
    log::info!(
        "checking directory: {} for *.{}.gv snapshot",
        directory_path,
        suffix
    );

    let full_suffix = format!("{}.gv", suffix);
    let data = std::fs::read_dir(&directory_path)
        .or_error(format!(
            "failed to find snapshot folder: {}. Maybe it's another id?",
            directory_path
        ))?
        .map(|entry| {
            let path = entry.unwrap().path().canonicalize().unwrap();
            log::info!("found: {}", directory_path);
            return path;
        })
        .filter(|path| path.is_file() && path.to_str().unwrap().ends_with(full_suffix.as_str()))
        .map(|file_path| fs::read_to_string(file_path))
        .collect::<Vec<io::Result<String>>>();

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

    let content = data.first().unwrap().as_ref().unwrap();
    let gv =
        graphviz_dot_parser::parse(&content).or_error(String::from("failed to parse graph"))?;

    return Ok(gv);
}
