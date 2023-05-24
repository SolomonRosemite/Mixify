mod args;

use args::MixifyArgs;
use clap::Parser;

fn main() {
    let args = MixifyArgs::parse();
    println!("{:?}", args);
}
