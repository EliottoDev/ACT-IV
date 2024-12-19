/**
* # File Description
*
* In each Git repository where the active tool is intended to function,
* there must be a configuration file named .act. This file serves as the
* central point for defining how active operates within the directory, ensuring
* that its behavior is tailored to the specific requirements of the repository.
*
* The .act file, referred to as the "act-git" file, adheres to a standard YAML
* format. This structure is similar to what is commonly used in tools like docker-compose
* or gitlab-ci. Its primary purpose is to provide a clear, modular, and flexible
* way to specify the behavior and interactions of files or processes managed by active.
*
* A critical rule is that any file referenced within the .act configuration must have a
* name starting with the prefix ACT-. This naming convention is mandatory to ensure consistency
* and avoid conflicts. However, when referring to these files inside the .act file itself,
* their names should be specified without the "ACT-" prefix. This approach enhances modularity
* and scalability, enabling the use of multiple files to handle distinct aspects of a single
* task type.
*
* By adopting this design, users can organize their workflows more effectively,
* allowing for the decomposition of complex tasks into smaller, manageable configurations.
* The modular nature of this setup makes it easy to adapt to evolving needs or extend the system
* to accommodate additional tasks without disrupting existing configurations.
*
*/
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

struct ConfigDirectory {
    name: String,
    count: usize,
    files: Vec<PathBuf>,
}

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

fn get_dir_info(dir: fs::DirEntry) -> ConfigDirectory {
    todo!();
}

impl ConfigDirectory {
    fn get_config() -> Self {
        todo!();
    }
}
