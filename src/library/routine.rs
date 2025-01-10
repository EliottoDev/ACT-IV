use chrono::Local;
use git2::{IndexAddOption, Repository, Signature, Tree};
use serde::{Deserialize, Serialize};
use std::{fs, io::Error, path::PathBuf};
use std::fs::File;
use tabled::{settings::*, Table, Tabled};
use tempfile::tempdir;
use toml;
use walkdir::WalkDir;

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
    pub commit_interval: String, // Example: "1d" for daily commit
    pub sync_method: String,     // Example: "thorough" or "delta"
    pub time_zone: String,
    pub last_sync_time: Option<String>, // Optional timestamp of the last sync
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Encrypt {
    pub encrypt: bool,
    pub password: Option<String>,      // Optional plain text password
    pub password_eval: Option<String>, // Command evaluation for more secure password
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Compress {
    pub compression: String, // Compression method, e.g., "gz", "xz", etc.
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Advanced {
    pub exclude: Vec<String>,  // List of excluded files or directories
    pub sync_on_startup: bool, // Whether to sync on startup
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ntfy {
    pub ntfy_topic: String,            // Notification topic
    pub notification_on_success: bool, // Notify on successful sync
    pub notification_on_failure: bool, // Notify on failed sync
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Git {
    pub remote: String,   // Git remote URL
    pub branch: String,   // Git branch name
    pub force_push: bool, // Whether to force push the changes
}

/// Walks through the directory and returns a list of paths, excluding `.git` directories
pub(crate) fn walk_directory(path: &str) -> Vec<PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok) // Filter out errors
        .filter(|entry| {
            let entry_path = entry.path();
            // Skip the .git directory and the base directory itself
            !entry_path.starts_with(&format!("{}.git", path))
                && entry_path != std::path::Path::new(path)
        })
        .map(|entry| entry.into_path())
        .collect()
}

/// Initializes the repository with an initial commit
fn init_repo(repo: &Repository, publisher: &Signature, tree: &Tree<'_>) -> Result<(), git2::Error> {
    let oid = repo.commit(
        Some("HEAD"), // Target reference
        publisher,    // Author and committer
        publisher,
        "ACT-IV Init", // Commit message
        &tree,         // The tree (file changes)
        &[],           // No parents (this is the first commit)
    )?;
    println!("Initialized repo: {}", oid);
    Ok(())
}

impl Routine {
    /// Reads the routine configuration from a TOML file
    pub fn read(file_name: &str) -> Result<Self, Error> {
        // Read the file content as a string
        let routine_str = fs::read_to_string(file_name)?;

        // Parse the TOML string into the `Routine` struct
        let routine: Routine = toml::from_str(&routine_str).unwrap();
        Ok(routine)
    }

    /// Stages the changes, commits them, and handles the Git repository
    pub fn stage(&self, message: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        // Determine the commit message, use current timestamp if not provided
        let msg =
            message.clone().unwrap_or_else(|| Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        println!("{}", msg);

        // Initialize the Git repository
        let path = &self.base.path;

	//Check if the repository exists, if not, create a new one
        let repo = match Repository::open(path) {
            Ok(repo) => repo,
            Err(_) => {
		println!("No repository exists, initializing.");
		Repository::init(path).expect("Failed to create repository")
            },
        };
	
        let mut repo_index = repo.index()?;

        // Get all file paths in the directory (excluding .git directories)
        let file_paths = walk_directory(path);
        let file_paths_string: String = file_paths
            .iter()
            .filter_map(|file| file.to_str())
            .filter_map(|file_str| file_str.strip_prefix(path)) // Strip the base path
            .collect::<Vec<_>>()
            .join(" "); // Join all paths with a space

        // Add the files to the Git index
        repo_index.add_all([&file_paths_string], IndexAddOption::DEFAULT, None)?;
        repo_index.write()?; // Write the changes to the index

        // Create a commit author signature
	/*TODO allow the user to change this*/
        let committer = Signature::now("test", "test@example.com")?;
        let tree_oid = repo_index.write_tree()?; // Write the index as a tree
        let tree = repo.find_tree(tree_oid)?;

        // If there is no HEAD, initialize the repository with the first commit
        let head_real = match repo.head() {
            Ok(head) => head,
            Err(_) => {
                init_repo(&repo, &committer, &tree)?; // Initialize the repo if no HEAD exists
                repo.head()?
            },
        };

        // Get the parent commit (the current latest commit)
        let parent_commit = head_real.peel_to_commit()?;

        // Create a new commit with the changes
        let commit_oid = repo.commit(
            Some("HEAD"),
            &committer,
            &committer,
            &msg,              // Commit message
            &tree,             // The changes (tree)
            &[&parent_commit], // The parent commit
        )?;

        println!("New commit created: {}", commit_oid);

        Ok(())
    }

    /// Prints the routine configuration as a table
    pub fn print(&self) {
        #[derive(Tabled)] // This struct will be used for displaying the data in a table
        struct Data<'a> {
            name: &'a str,
            value: String,
        }

        // Collect all the configuration data into a vector of Data structs
        let data = vec![
            Data { name: "Title", value: self.base.title.clone() },
            Data { name: "Path", value: self.base.path.clone() },
            Data { name: "Interval Timestamp", value: self.base.interval.timestamp.to_string() },
            Data { name: "Commit Interval", value: self.base.interval.commit_interval.clone() },
            Data { name: "Sync Method", value: self.base.interval.sync_method.clone() },
            Data { name: "Time Zone", value: self.base.interval.time_zone.clone() },
            Data {
                name: "Last Sync Time",
                value: self.base.interval.last_sync_time.clone().unwrap_or("None".to_string()),
            },
            Data { name: "Encrypt", value: self.crypt.encrypt.to_string() },
            Data {
                name: "Password",
                value: self.crypt.password.clone().unwrap_or("None".to_string()),
            },
            Data {
                name: "Password Eval",
                value: self.crypt.password_eval.clone().unwrap_or("None".to_string()),
            },
            Data { name: "Compression", value: self.press.compression.clone() },
            Data { name: "Exclude", value: format!("{:?}", self.advanced.exclude) },
            Data { name: "Sync On Startup", value: self.advanced.sync_on_startup.to_string() },
            Data { name: "Notify Topic", value: self.ntfy.ntfy_topic.clone() },
            Data {
                name: "Notification on Success",
                value: self.ntfy.notification_on_success.to_string(),
            },
            Data {
                name: "Notification on Failure",
                value: self.ntfy.notification_on_failure.to_string(),
            },
            Data { name: "Git Remote", value: self.git.remote.clone() },
            Data { name: "Git Branch", value: self.git.branch.clone() },
            Data { name: "Force Push", value: self.git.force_push.to_string() },
        ];

        // Configure the table styling and display it
	/*TODO Option to disable the pretty table printing*/
        let table = Table::new(data)
            .with(Settings::default().with(Style::rounded())) // Apply rounded style
            .to_string();

        println!("{}", table); // Print the table to the console
    }
}
