use clap::Parser;
use cli::{Args, Commands};
use commands::{catch, info, stage, sync, throw, wind};
use crate::commands::init;

mod cli;
mod commands;
mod library;
mod tests;

fn main() {
    // Parse command-line arguments
    let args = Args::parse();

    // Match the parsed command and delegate the execution
    match &args.command {
        Commands::Info { routine } => info::info(routine),
        Commands::Stage { routine, message } => stage::stage(routine, message),
        Commands::Sync => sync::sync(),
        Commands::Throw {} => throw::throw(),
        Commands::Catch => catch::catch(),
        Commands::Wind => wind::wind(),
        Commands::Init { name } => init::init(name),
    }
}
