use std::env::current_dir;
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
    #[clap(about = "Gets ", long_about = None, name = "info")]
    Info { routine:String },
    #[clap(about = "", long_about = None, name = "throw")]
    Throw {},
    #[clap(about = "", long_about = None, name = "catch")]
    Catch,
    #[clap(about = "", long_about = None, name = "wind")]
    Wind,
}

pub(crate) fn info(routine_path: &String) {
    let mut relative_path = current_dir().unwrap();
    let absolute_path = Path::new(&routine_path);
    if absolute_path.exists() && absolute_path.is_file() {
	let routine_toml = library::routine::read_routine(routine_path).unwrap(); //routine is a &String already
	println!("Routine {} exists", routine_toml.base.title); //print the title of the routine
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
	println!("Routine {} exists", routine_toml.base.title); //print the title of the routine
        return;
    }

    println!("Routine not found");
}

pub(crate) fn throw() {
}

pub(crate) fn catch() {
    todo!();
}

pub(crate) fn wind() {
    todo!();
}
