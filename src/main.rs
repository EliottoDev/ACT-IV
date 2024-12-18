use clap::{Parser, Subcommand};

// CLI args
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    // Subcommand delegator
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // TODO: Add subcommands
}

fn main() {
    let args = Args::parse();

    // TODO: Implement logic
    match &args.command {
        None => {}
        _ => {}
    }
}
