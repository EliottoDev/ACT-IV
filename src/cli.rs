use std::path::Path;
use clap::{Parser, Subcommand};
use crate::library;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    #[clap(about = "Print info about a routine", long_about = None, name = "info")]
    Info { routine:String },
    #[clap(about = "Prepare a routine's Git repository for throwing", long_about = None, name = "stage")]
    Stage { routine:String, message:Option<String> },
    #[clap(about = "Push a routine's Git repository to it's specified remote", long_about = None, name = "throw")]
    Throw {},
    #[clap(about = "Pull a routine's Git repository from it's specified remote", long_about = None, name = "catch")]
    Catch,
    #[clap(about = "Revert a routine's Git repository to a previous commit", long_about = None, name = "wind")]
    Wind,
}

//Print info about a routine
pub(crate) fn info(routine_path: &String) {
    let mut relative_path = std::env::current_dir().unwrap();
    let absolute_path = Path::new(&routine_path);
    
    if absolute_path.exists() && absolute_path.is_file() {
	let routine_toml = library::routine::read_routine(routine_path).unwrap(); //routine is a &String already
	library::routine::print_routine(&routine_toml);
        return;
    }

    for directory in routine_path.split("/") {
        if directory == "." { continue; }
        if directory == ".." {
            relative_path.pop();
            continue;
        }
        relative_path.push(directory);
    }

    if relative_path.exists() && relative_path.is_file() {
	let routine_toml = library::routine::read_routine(routine_path).unwrap(); //routine is a &String already
	library::routine::print_routine(&routine_toml);
        return;
    }

    println!("Routine not found");
}
//create the repo, commit changes, and stop there
pub(crate) fn stage(routine_path: &String, message: &Option<String>) {
    let routine_toml = library::routine::read_routine(routine_path).unwrap(); //stage_routine takes a &Routine, so we need to get that

    if let Some(_m) = message {
	library::routine::stage_routine(&routine_toml, message);
    } else { 
	library::routine::stage_routine(&routine_toml, &None);
    }
}

pub(crate) fn throw() {
    todo!()
}

pub(crate) fn catch() {
    todo!();
}

pub(crate) fn wind() {
    todo!();
}
