use crate::plan_command;

use super::args;

pub fn handle_apply_snapshot(cmd: &args::ApplyCommand) -> Result<(), anyhow::Error> {
    let gv = plan_command::read_snapshot(cmd.id, "edit")?;
    let res = plan_command::create_execution_plan(&gv)?;

    return Ok(());
}
