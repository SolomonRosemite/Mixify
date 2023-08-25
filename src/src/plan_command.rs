use std::{fs, io, panic};

use graphviz_dot_parser::types::Stmt;

use crate::constants;

use super::args;

type EdgeData = (String, String, graphviz_dot_parser::types::Attributes);
type NodeData = (String, graphviz_dot_parser::types::Attributes);

#[derive(Debug)]
struct Action {
    action_type: ActionType,
    node: String,
    for_node: String,
    idx: usize,
}

#[derive(Debug)]
enum ActionType {
    CreateAndQueryPlaylist,
    QuerySongsPlaylist,
    CopySongs,
    RemoveSongs,
}

pub fn handle_plan_snapshot(cmd: &args::PlanCommand) {
    create_execution_plan(cmd.id);
    // let folder = format!("snapshots/{}", cmd.id);
    // let error_msg = format!("expected to find snapshot folder: {}", folder);
    //
    // let found_files: Vec<_> = std::fs::read_dir(&folder).expect(&error_msg).collect();
    //
    // found_files
    //     .iter()
    //     .for_each(|dir| println!("Found: {:?}", dir));
    //
    // println!("Plan snapshot: {}", cmd.id);
}

pub fn create_execution_plan(snapshot_id: u32) {
    // Read the snapshot file by id and read the current state file if it exists
    // Compare the two files and create a list of actions to perform
    // Write the list of actions to a file
    // Return the list of actions

    let directory_path = format!("snapshots/{}/", snapshot_id);
    let data = std::fs::read_dir(&directory_path)
        .expect(&format!(
            "expected to find snapshot folder: {}",
            directory_path
        ))
        .map(|entry| {
            let path = entry.unwrap().path().canonicalize().unwrap();
            println!("found path: {:?}", path);
            return path;
        })
        .filter(|path| path.is_file() && path.to_str().unwrap().ends_with("edit.gv"))
        .map(|file_path| fs::read_to_string(file_path))
        .collect::<Vec<io::Result<String>>>();

    if data.len() == 0 {
        panic!(
            "{}",
            format!("No *.edit.gv file found in {} folder", &directory_path)
        );
    }

    if data.len() > 1 {
        panic!(
            "{}",
            format!(
                "More than one *.edit.gv file found in {} folder. Expected only one since mixify doesn't know which one to use.",
                &directory_path
            )
        );
    }

    let content = data.first().unwrap().as_ref().unwrap();
    let gv = graphviz_dot_parser::parse(&content).unwrap();

    let mut nodes: Vec<NodeData> = Vec::new();
    let mut edges: Vec<EdgeData> = Vec::new();
    let mut root_nodes: Vec<String> = Vec::new();

    // NOTE: Important for this to work. All nodes must be defined in the graph. Otherwise it will panic.
    // In other words, edges that dont point to a node explicitly defined in the graph will cause a panic.
    // TODO: Technically, we can look what nodes are missing and add them to the graph or let the user know. (I believe)
    let result = panic::catch_unwind(|| gv.to_directed_graph());
    let graph = match result {
        Ok(g) => g.unwrap(),
        Err(_) => panic!(
            "Failed to create graph. There is a chance you forgot to define a node in the graph."
        ),
    };

    gv.stmt.iter().for_each(|stmt| match stmt {
        Stmt::Edge(from, to, attrs) => {
            edges.push((from.to_string(), to.to_string(), attrs.clone()));
        }
        Stmt::Node(node, attrs) => {
            let node_index = graph.node_indices().find(|i| graph[*i] == *node).unwrap();
            let number_of_outgoing_edges = graph
                .neighbors_directed(node_index, petgraph::Direction::Outgoing)
                .count();

            if number_of_outgoing_edges == 0 {
                root_nodes.push(node.to_string());
            }

            nodes.push((node.to_string(), attrs.clone()));
        }
        _ => {}
    });

    println!("root_nodes: {:?}", root_nodes);
    let mut all_actions: Vec<Vec<Action>> = Vec::new();

    for root in root_nodes {
        // TODO: Not sure about index 1. Since we are using a directed graph, we can have multiple root nodes.
        let res = create_node_execution_plan(1, &root, &nodes, &edges, &graph);
        all_actions.push(res);
    }

    for actions in all_actions {
        for action in actions {
            println!(
                "{:?} from {} for/to {}",
                action.action_type, action.node, action.for_node,
            );
        }
    }
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

    let action_type = match playlist_already_exists {
        true => ActionType::QuerySongsPlaylist,
        false => ActionType::CreateAndQueryPlaylist,
    };

    let action = Action {
        action_type,
        node: node.clone(),
        idx,
        for_node: node.clone(),
    };

    for action in final_node_actions {
        actions.push(action);
    }

    actions.push(action);
    return actions;
}
