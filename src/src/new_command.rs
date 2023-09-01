use crate::plan_command;

use super::args;
use chrono::prelude::*;

pub fn handle_new_snapshot(cmd: &args::NewCommand) -> Result<(), anyhow::Error> {
    let (content, id) = get_latest_snapshot_or_default(cmd.name.clone())?;

    let file_name = format!("snapshots/{}/{}_{}.edit.gv", id, id, cmd.name);
    let snapshot_folder_name = format!("snapshots/{}/{}", id, id);

    let file_err = format!("Failed to write to file: {}", file_name);
    let folder_err = format!("Failed to create snapshot folder: {}", snapshot_folder_name);

    std::fs::create_dir_all(format!("snapshots/{}", id)).expect(&folder_err);
    std::fs::write(&file_name, content).expect(&file_err);

    println!("Created snapshot: {}!", &file_name);
    return Ok(());
}

fn get_latest_snapshot_or_default(name: String) -> Result<(String, u32), anyhow::Error> {
    std::fs::create_dir_all("snapshots/").unwrap();
    let found_dirs: Vec<_> = std::fs::read_dir("snapshots/").unwrap().collect();

    found_dirs
        .iter()
        .for_each(|dir| println!("Found: {:?}", dir));

    let latest_id = found_dirs
        .into_iter()
        .filter_map(|x| x.ok()?.file_name().into_string().ok()?.parse::<u32>().ok())
        .max();

    let content = match latest_id {
        Some(id) => {
            let c = plan_command::read_snapshot_file(id, "post.apply")?;
            let now = Local::now();
            let content = format!(
                "// Name: {}
// Created at: {}
// -------------------------------
{}",
                name,
                now.format("%Y-%m-%d %H:%M:%S"),
                c,
            );
            return Ok((content, id + 1));
        }
        None => (default_snapshot(name)?, 1),
    };

    return Ok(content);
}

fn default_snapshot(name: String) -> Result<String, anyhow::Error> {
    let now = Local::now();
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
        name,
        now.format("%Y-%m-%d %H:%M:%S")
    );

    return Ok(content);
}
