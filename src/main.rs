use clap::Parser;
mod cli;
mod library;

fn main() {
    let args = cli::Args::parse();
    use cli::Commands;

    match &args.command {
        Commands::Info { routine } => cli::info(routine),
        Commands::Stage { routine, message } => cli::stage(routine, message),
        Commands::Throw {} => cli::throw(),
        Commands::Catch => cli::catch(),
        Commands::Wind => cli::wind()
    }
}
