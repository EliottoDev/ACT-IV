use std::{env::var, fs};
use std::{io::Error, path::{Path, PathBuf}};
use clap::error;
use serde::{Deserialize, Serialize};
use tabled::{
    Table, Tabled,
    settings::*
};
use git2::{Commit, IndexAddOption, Repository, Signature, Tree};
use walkdir::WalkDir;
use chrono::Local;
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
        panic!("Error reading config file {}\nKind -> {}\nMessage -> {}", file_name, error.kind(), error);
    }

    let routine: Routine = toml::from_str(&routine_str.unwrap()).unwrap();

    Ok(routine)
}

pub fn print_routine(routine: &Routine) {
	#[derive(Tabled)]
	struct Data<'a> {
	    name: &'a str,
	    value: String,
	}

	let data = vec![
            Data {
		name: "Title",
		value: routine.base.title.clone(),
            },
            Data {
		name: "Path",
		value: routine.base.path.clone(),
            },
            Data {
		name: "Interval Timestamp",
		value: routine.base.interval.timestamp.to_string(),
            },
            Data {
		name: "Commit Interval",
		value: routine.base.interval.commit_interval.clone(),
            },
            Data {
		name: "Sync Method",
		value: routine.base.interval.sync_method.clone(),
            },
            Data {
		name: "Time Zone",
		value: routine.base.interval.time_zone.clone(),
            },
            Data {
		name: "Last Sync Time",
		value: routine.base.interval.last_sync_time.clone().unwrap_or("None".to_string()),
            },
            Data {
		name: "Encrypt",
		value: routine.crypt.encrypt.to_string(),
            },
            Data {
		name: "Password",
		value: routine.crypt.password.clone().unwrap_or("None".to_string()),
            },
            Data {
		name: "Password Eval",
		value: routine.crypt.password_eval.clone().unwrap_or("None".to_string()),
            },
            Data {
		name: "Compression",
		value: routine.press.compression.clone(),
            },
            Data {
		name: "Exclude",
		value: format!("{:?}", routine.advanced.exclude),
            },
            Data {
		name: "Sync On Startup",
		value: routine.advanced.sync_on_startup.to_string(),
            },
            Data {
		name: "Notify Topic",
		value: routine.ntfy.ntfy_topic.clone(),
            },
            Data {
		name: "Notification on Success",
		value: routine.ntfy.notification_on_success.to_string(),
            },
            Data {
		name: "Notification on Failure",
		value: routine.ntfy.notification_on_failure.to_string(),
            },
            Data {
		name: "Git Remote",
		value: routine.git.remote.clone(),
            },
            Data {
		name: "Git Branch",
		value: routine.git.branch.clone(),
            },
            Data {
		name: "Force Push",
		value: routine.git.force_push.to_string(),
            },
	];

	let table_config = Settings::default()
	    .with(Style::rounded());

	let table = Table::new(data)
	    .with(table_config)
	    .to_string();

	println!("{}", table);
}

fn walk_directory(path: &str) -> Vec<PathBuf> { //this is used in stage_routine
    let mut paths = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
	let entry_path = entry.path();
	let git_pattern = format!("{}.git", path);
	if entry_path != std::path::Path::new(path) {
	    if entry_path.starts_with(git_pattern) { continue; }
	    paths.push(entry_path.to_path_buf());
	}
    }
    paths
}

fn init_repo(repo: &Repository, publisher: &Signature, tree: &Tree<'_>) {
    let oid = repo.commit(
	Some("HEAD"),
	publisher,
	publisher,
	"ACT-IV Init",
	&tree,
	&[],
    ).expect("Failed to initalize repository");

    println!("initialized repo: {}", oid);
}

pub fn stage_routine(routine: &Routine, message: &Option<String>) {
    let msg:String;

    //optional commit message
    match message {
	Some(m) => { msg = m.to_string(); },
	//TODO for now just have the default commit, change later to be optional
	None => { msg = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(); }
    }

    println!("{}", msg);

    let path = &routine.base.path;
    let new_repo = git2::Repository::init(path).unwrap();
    let mut repo_index = new_repo.index().unwrap();

    let file_paths = walk_directory(path);
    let mut file_paths_string = String::new();
    //the reason we do this is so that later we can add an ignore files feature to the routine itself
    for file in file_paths {
	if let Some(file_str) = file.to_str() {
	    let file_str_stripped = file_str.strip_prefix(path).expect("File path doesn't start with base path");
            file_paths_string.push_str(file_str_stripped);
	    file_paths_string.push(' ');
        } else {
            eprintln!("Invalid UTF-8 path: {:?}", file);
        }
    }

    if let Err(e) = repo_index.add_all([file_paths_string].iter(), IndexAddOption::DEFAULT, None) {
        eprintln!("Failed to add files to index: {}", e);
    }

    if let Err(e) = repo_index.write() {
        eprintln!("Failed to write to index: {}", e);
    }

    /* todo!("Implement committing automatically on a new repository"); */
    //COMPLETE it now can initialize the repository and make an initial commit if it needs to*
    //*git2 can fail to create a commit if one hasn't already been made

    let committer = Signature::now("test", "test@example.com").expect("Failed to create signature");
    let tree_oid = repo_index.write_tree().expect("Failed to write index as tree");
    let tree = new_repo.find_tree(tree_oid).expect("Failed to find tree");

    let mut head = new_repo.head();
    let mut head_real;

    //If we don't do it this way rustc gets mad that the value of head can be uninitialized
    if let Err(err) = &head {
	init_repo(&new_repo, &committer, &tree);
	head_real = new_repo.head().unwrap();
    } else { head_real = head.unwrap() }

    let parent_commit = head_real.peel_to_commit().expect("Expected to peel to commit");

    let commit_oid = new_repo.commit(
	Some("HEAD"),
	&committer,
	&committer,
	&msg,
	&tree,
	&[&parent_commit],
    ).expect("Failed to create commit");

    println!("New commit created: {}", commit_oid);
}
