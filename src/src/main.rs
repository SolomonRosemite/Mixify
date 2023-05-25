mod args;
mod new_command;

use args::MixifyArgs;
use clap::Parser;

fn main() {
    let args = MixifyArgs::parse();

    match args.entity_type {
        args::EntityType::New(cmd) => new_command::handle_new_snapshot(&cmd),
        args::EntityType::Plan(_) => todo!(),
        args::EntityType::Apply(_) => todo!(),
    };
}
