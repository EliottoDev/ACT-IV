mod lib;

use clap::{Parser, Subcommand};

// CLI args
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // TODO: Add subcommands
    #[clap(about = "Gets project act file info", long_about = None, name = "info")]
    Info,

}

fn main() {
    let args = Args::parse();

    // TODO: Implement logic
    match &args.command {
        None => {}
        _ => {}
    }
}
