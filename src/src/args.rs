use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(
    author = "SolomonRosemite™",
    about = "A tool that allows you to combine different spotify playlists that sync continuously.",
    version = "0.0.1"
)]
pub struct MixifyArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create new snapshot file to edit
    #[command(arg_required_else_help = true)]
    New(NewCommand),

    /// View the changes and playlist updates that will be applied
    #[command(arg_required_else_help = true)]
    Plan(PlanCommand),

    /// Apply playlist updates to Spotify
    #[command(arg_required_else_help = true)]
    Apply(ApplyCommand),
}

#[derive(Debug, Args)]
pub struct NewCommand {
    /// The name of the snapshot
    pub name: String,
}

#[derive(Debug, Args)]
pub struct PlanCommand {
    /// The id of the snapshot, if not provided, the latest snapshot will be used
    pub id: u32,
}

#[derive(Debug, Args)]
pub struct ApplyCommand {
    /// The id of the snapshot, if not provided, the latest snapshot will be used
    pub id: u32,
}
