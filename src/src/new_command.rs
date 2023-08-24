use super::args;
use chrono::prelude::*;

pub fn handle_new_snapshot(cmd: &args::NewCommand) {
    std::fs::create_dir_all("snapshots/").unwrap();
    let found_dirs: Vec<_> = std::fs::read_dir("snapshots/").unwrap().collect();

    found_dirs
        .iter()
        .for_each(|dir| println!("Found: {:?}", dir));

    let latest_id = found_dirs
        .into_iter()
        .filter_map(|x| x.ok()?.file_name().into_string().ok()?.parse::<u32>().ok())
        .max();

    let new_id = match latest_id {
        Some(id) => {
            println!("Latest snapshot id: {}", id);
            id + 1
        }
        None => {
            println!("No snapshots found");
            1
        }
    };

    let now = Local::now();
    let file_name = format!("snapshots/{}/{}_{}.edit.gv", new_id, new_id, cmd.name);
    let snapshot_folder_name = format!("snapshots/{}/{}", new_id, new_id);
    let content = format!(
        "// Name: {}
// Created at: {}

digraph G {{
    Chill_lofi [URL=\"https://open.spotify.com/playlist/44xuOOjdOcWDeVsIthiEUG\"];
    More_Lofi [URL=\"https://open.spotify.com/playlist/6wrY4pcN1Q1yQV8fmmf4Dk\"];
    Lofi [label=\"all Lofi songs\"];
    
    Chill_lofi -> Lofi;
    More_Lofi -> Lofi;
    New_Lofi_that_might_not_be_good -> Test;
    Lofi -> Test;
}}
        ",
        cmd.name,
        now.format("%Y-%m-%d %H:%M:%S")
    );

    let file_err = format!("Failed to write to file: {}", file_name);
    let folder_err = format!("Failed to create snapshot folder: {}", snapshot_folder_name);

    std::fs::create_dir_all(format!("snapshots/{}", new_id)).expect(&folder_err);
    std::fs::write(&file_name, content).expect(&file_err);

    println!("Created snapshot: {}!", &file_name);
}
