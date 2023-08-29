use super::args;

pub fn handle_apply_snapshot(cmd: &args::ApplyCommand) -> Result<(), anyhow::Error> {
    let path = format!("snapshots/{}/1_init.edit.gv", cmd.id);
    // let x = std::fs::read_dir(folder).unwrap().collect::<Vec<_>>();
    let content = std::fs::read_to_string(path).unwrap();
    let gv = graphviz_dot_parser::parse(&content).unwrap();

    println!("{:?}", gv.stmt);
    return Ok(());
}
