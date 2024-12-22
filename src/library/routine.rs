use std::fs;
use std::io::Error;
use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Routine {
    pub base: Base,
    pub crypt: Encrypt,
    pub press: Compress,
    pub advanced: Advanced,
    pub ntfy: Ntfy,
    pub git: Git,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Base {
    pub title: String,
    pub path: String,
    pub interval: BaseInterval,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseInterval {
    pub timestamp: bool,
    pub commit_interval: String,  // e.g., "1d"
    pub sync_method: String,      // e.g., "thorough" or "delta"
    pub time_zone: String,
    pub last_sync_time: Option<String>,  // optional timestamp
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Encrypt {
    pub encrypt: bool,
    pub password: Option<String>,     // optional plain text password
    pub password_eval: Option<String>, // more secure: command evaluation
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Compress {
    pub compression: String,  // e.g., "gz", "xz", etc.
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Advanced {
    pub exclude: Vec<String>,   // list of excluded files and directories
    pub sync_on_startup: bool,  // whether to sync on startup
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ntfy {
    pub ntfy_topic: String,            // notification topic
    pub notification_on_success: bool, // notify on success
    pub notification_on_failure: bool, // notify on failure
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Git {
    pub remote: String,    // Git remote URL
    pub branch: String,    // Git branch name
    pub force_push: bool,  // whether to force push
}

pub fn read_routine(file_name: &String) -> Result<Routine, Error> {
    let routine_str = fs::read_to_string(file_name);

    if let Err(error) = routine_str {
        panic!("Error reading config file {}\nKind -> {}\nMessage: {}", file_name, error.kind(), error);
    }

    let routine: Routine = toml::from_str(&routine_str.unwrap()).unwrap();

    Ok(routine)
}

pub fn print_routine(file_name: &String) {
    println!("{:?}", read_routine(&file_name).unwrap());
}
