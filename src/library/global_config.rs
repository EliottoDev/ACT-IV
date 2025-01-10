use clap::Error;
use serde::{Serialize, Deserialize};
use serde_json;
use tabled::grid::config;
use std::{path::{Path, PathBuf}, env, fs::{self, create_dir, File, OpenOptions}, io::{self, Write}};
use super::routine::walk_directory;

/* ACT-IV maintains a library of active routines, located in $XDGCONFIGDIR/ACT-IV/, where each routine is identified by a name.

EX: The file `act-notes.toml` located in `$XDGCONFIGDIR/ACT-IV/` will have a `name` field set to "notes."
This allows the routine to be referenced by its name, "notes," rather than by its full file path.

Benefits:
- Commands like `act [stage|throw|catch|wind|etc]` can use the name "notes" instead of the full file path.
- EX: `act throw notes` instead of `act throw /home/me/.config/ACT-IV/act-notes.toml`. */

const CONFIG_FILE_NAME: &str = "ACT-IV.toml";
const LIBRARY_FILE_NAME: &str = "ACT-LIBRARY.toml";

/// Find the config directory
fn get_config_dir() -> Result<PathBuf, std::io::Error> {
    let home_dir = match std::env::var("HOME") {
	Ok(value) => { PathBuf::from(value) }
	Err(e) => { panic!("Error finding $HOME env-var: {}", e) }
    };
    
    let config_dir = home_dir.join(".config").join("ACT-IV");

    match std::fs::create_dir(&config_dir) {
	Ok(value) => { /*Create the directory if it doesn't exist*/ }
	Err(e) => {
	    if e.kind() == std::io::ErrorKind::AlreadyExists {
		/*We know it exists*/
            } else {
		eprintln!("Failed to create config directory: {}", e);
            }
	}
    }

    Ok(config_dir)
}

/// Write to the global config file for ACT-IV
pub(crate) fn write_config() -> io::Result<()> {
    


    todo!();
    // let config_file_path = 

    // let mut file = OpenOptions::new()
    //     .read(true)
    //     .write(true)
    //     .create(true)
        // .open(config_file_path)?;

    // write!(file, "Hello, world!")?;
    
    // Ok(())
}

/// Write to the routine library for ACT-IV
pub(crate) fn write_library() {
    let config_dir = match get_config_dir() {
	Ok(path) => { path },
	Err(e) => { panic!("Error getting config directory: {}", e) }
    };

    let files = super::routine::walk_directory(config_dir.to_str().unwrap());
    println!("{:?}", files);
}

/// Read from the global config file for ACT-IV
pub(crate) fn read_config() {
    todo!();
}

/// Read from the routine library for ACT-IV
pub(crate) fn read_library() {
    todo!();
}
