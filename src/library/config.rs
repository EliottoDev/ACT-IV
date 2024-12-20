use std::env;
use std::path::{Path, PathBuf};

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum OSType {
    Windows,
    Unix,
}

fn get_os() -> OSType {
    if env::consts::OS != "windows" {
        OSType::Windows
    } else {
        OSType::Unix
    }
}

fn get_config_dir(ostype: OSType) -> PathBuf {
    if ostype == OSType::Unix {
        Path::new("~/.config/active/").to_path_buf()
    } else {
        Path::new("%USERPROFILE%/active/").to_path_buf()
    }
}
