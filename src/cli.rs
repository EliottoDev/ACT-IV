use clap::{Parser, Subcommand};

/// Represents the command-line arguments structure.
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub(crate) struct Args {
    /// Subcommand to execute based on the user input.
    #[clap(subcommand)]
    pub(crate) command: Commands,
}

/// Enum representing the available commands in the CLI.
#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// Prints detailed information about a routine.
    #[clap(about = "Display information about a routine.", long_about = None, name = "info")]
    Info {
        /// The path to the routine to retrieve information from.
        routine: String,
    },

    /// Prepares a routine's Git repository for further actions (e.g., pushing).
    #[clap(about = "Prepare a routine's Git repository for subsequent actions.", long_about = None, name = "stage")]
    Stage {
        /// Path to the routine repository to be staged.
        routine: String,

        /// Optional commit message; if none is provided, current timestamp is used.
        message: Option<String>,
    },

    /// Creates a new routine file under the name, "name", defaulting to current dir name
    #[clap(about = "Init and register a new routine.", long_about = None, name = "init")]
    Init {
        name: Option<String>
    },

    /// Sync the routine library and routine folder
    #[clap(about = "Sync the routine library with the routine folder", long_about = None, name = "sync")]
    Sync,

    /// Pushes a routine's Git repository to the specified remote.
    #[clap(about = "Push a routine's Git repository to its remote.", long_about = None, name = "throw")]
    Throw {},

    /// Pulls a routine's Git repository from the specified remote.
    #[clap(about = "Pull a routine's Git repository from its remote.", long_about = None, name = "catch")]
    Catch,

    /// Reverts a routine's Git repository to a previous commit.
    #[clap(about = "Revert a routine's Git repository to a previous commit.", long_about = None, name = "wind")]
    Wind,
}
