mod args;
mod new_command;
mod plan_command;

use args::MixifyArgs;
use clap::Parser;

fn main() {
    let args = MixifyArgs::parse();

    match args.entity_type {
        args::EntityType::New(cmd) => new_command::handle_new_snapshot(&cmd),
        args::EntityType::Plan(cmd) => plan_command::handle_plan_snapshot(&cmd),
        args::EntityType::Apply(_) => todo!(),
    };
}
