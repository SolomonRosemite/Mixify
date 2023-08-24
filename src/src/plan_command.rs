use core::panic;
// use petgraph::Graph;
use std::{collections::HashMap, fs};

use graphviz_dot_parser::types::Stmt;

use super::args;

enum Action {
    CreatePlaylist,
    CopySongs,
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

    // let folder = format!("snapshots/{}/", snapshot_id);
    // let error_msg = format!("expected to find snapshot folder: {}", folder);

    // let found_files: Vec<_> = std::fs::read_dir(&folder).expect(&error_msg).collect();

    // found_files
    //     .iter()
    //     .for_each(|dir| println!("Found: {:?}", dir));

    // let file = found_files
    //     .iter()
    //     .filter(|dir| {
    //         dir.expect("")
    //             .file_name()
    //             .to_str()
    //             .unwrap()
    //             .contains("edit.gv")
    //     })
    //     .map(|dir| dir.unwrap().file_name().to_str().unwrap())
    //     .collect::<Vec<&str>>()
    //     .first();

    // match file {
    //     Some(file) => println!("Found: {}", file),
    //     None => {
    //         let err = format!("No edit.gv file found in {}", folder);
    //         panic!("{}", err);
    //     }
    // }

    // let directory_path = format!("snapshots/{}", snapshot_id);
    //
    // let x: Vec<_> = fs::read_dir(directory_path)
    //     .unwrap()
    //     // .filter_map(Result::ok)
    //     .map(|entry| entry.unwrap().path().canonicalize())
    //     .filter_map(Result::ok)
    //     .filter(|path| path.is_file() && path.ends_with("edit.gv"))
    //     .map(|file_path| {
    //         return fs::read_to_string(file_path)
    //             .map(|file_contents| println!("File contents: {}", file_contents));
    //     })
    //     .collect();

    // println!("x: {:?}", x);

    // let snapshot_file = format!("snapshots/{}/snapshot.json", cmd.id);
    // let error_msg = format!("expected to find snapshot file: {}", snapshot_file);

    // let snapshot_file = std::fs::read_to_string(&snapshot_file).expect(&error_msg);

    // let current_state_file = format!("snapshots/{}/current_state.json", cmd.id);

    // let current_state_file = std::fs::read_to_string(&current_state_file);

    // let current_state_file = match current_state_file {
    //     Ok(file) => Some(file),
    //     Err(_) => None,
    // };

    // ----------------------------------------

    let snapshot_file = format!("snapshots/1/1_init.edit.gv");
    let content = fs::read_to_string(&snapshot_file).expect("expected to find snapshot file");

    let gv = graphviz_dot_parser::parse(&content).unwrap();

    let mut nodes: Vec<(String, String, graphviz_dot_parser::types::Attributes)> = Vec::new();
    let mut edges: Vec<(String, String, graphviz_dot_parser::types::Attributes)> = Vec::new();
    let mut root_nodes: Vec<String> = Vec::new();
    let mut can_not_be_root_node: Vec<String> = Vec::new();

    gv.stmt.iter().for_each(|x| match x {
        Stmt::Edge(from, to, attrs) => {
            can_not_be_root_node.push(from.to_string());
            edges.push((from.to_string(), to.to_string(), attrs.clone()));
        }
        _ => {}
    });

    gv.stmt.iter().for_each(|x| match x {
        Stmt::Edge(from, to, attrs) => {
            if !can_not_be_root_node.contains(to) && !root_nodes.contains(to) {
                root_nodes.push(to.to_string());
            }
            nodes.push((from.to_string(), to.to_string(), attrs.clone()));
        }
        _ => {}
    });

    println!("root_nodes: {:?}", root_nodes);
    let actions: Vec<Vec<Action>> = Vec::new();

    // TODO: Important for this to work. All nodes must be defined in the graph. Otherwise it will panic.
    // In other words, edges that dont point to a node explicitly defined in the graph will cause a panic.
    let graph = gv.to_directed_graph().unwrap();

    // let e = edges;

    for root in root_nodes {
        let root_item_index = graph.node_indices().find(|x| graph[*x] == root).unwrap();
        let nei = graph.neighbors_directed(root_item_index, petgraph::Direction::Incoming);

        // nodes.into_iter().for_each(|edge| {
        //     let (x,y,z) = edge;
        // });
        //
        // for n in nei {
        //     println!("n: {:?}", graph[n]);
        // }
    }

    println!("done");
}
