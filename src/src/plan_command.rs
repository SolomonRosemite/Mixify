use core::panic;
use std::fs;

use super::args;

pub fn handle_plan_snapshot(cmd: &args::PlanCommand) {
    let folder = format!("snapshots/{}", cmd.id);
    let error_msg = format!("expected to find snapshot folder: {}", folder);

    let found_files: Vec<_> = std::fs::read_dir(&folder).expect(&error_msg).collect();

    found_files
        .iter()
        .for_each(|dir| println!("Found: {:?}", dir));

    println!("Plan snapshot: {}", cmd.id);
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

    let directory_path = format!("snapshots/{}", snapshot_id);

    let x: Vec<_> = fs::read_dir(directory_path)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path().canonicalize())
        .filter_map(Result::ok)
        .filter(|path| path.is_file() && path.ends_with("edit.gv"))
        .map(|file_path| {
            return fs::read_to_string(file_path)
                .map(|file_contents| println!("File contents: {}", file_contents));
        })
        .collect();

    let snapshot_file = format!("snapshots/{}/snapshot.json", cmd.id);
    let error_msg = format!("expected to find snapshot file: {}", snapshot_file);

    let snapshot_file = std::fs::read_to_string(&snapshot_file).expect(&error_msg);

    let current_state_file = format!("snapshots/{}/current_state.json", cmd.id);

    let current_state_file = std::fs::read_to_string(&current_state_file);

    let current_state_file = match current_state_file {
        Ok(file) => Some(file),
        Err(_) => None,
    };
}
