use core::panic;
use std::{
    fs::{self, DirEntry, FileType},
    os::linux::fs,
    path::PathBuf,
};

use super::db_file_exists;

#[derive(Debug)]
enum MigrationType {
    UP,
    DOWN,
}

#[derive(Debug)]
struct Migration {
    id: i16,
    name: String,
    migration_type: MigrationType,
    content: String,
}

fn is_migration_file_name_valid(name: &str) -> bool {
    ["up", "down"].contains(&name)
}

fn is_migration_file_ext_valid(extention: &str) -> bool {
    ["sql"].contains(&extention)
}

fn read_migration_folder(path: PathBuf) -> Option<(Migration, Migration)> {
    let folder_name = path.display().to_string();
    let (id, name) = folder_name.split_once('-').unwrap_or_else(|| ("", ""));
    let id: i16 = id.parse().unwrap_or(-1);
    let name = name.to_string();
    if id == -1 && name == "" {
        return None;
    }

    let mut up_file_content = String::new();
    let mut down_file_content = String::new();

    for entry in path.read_dir().expect("could not read folder") {
        if let Ok(entry) = entry {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    continue;
                }

                let file_full_name = entry.path().display().to_string();
                let (file_name, extention) = file_full_name.split_once('.').unwrap();

                if !is_migration_file_ext_valid(extention) {
                    continue;
                }
                if !is_migration_file_name_valid(file_name) {
                    continue;
                }

                match file_name {
                    "up" => {
                        up_file_content = fs::read_to_string(entry.path()).unwrap();
                    }
                    "down" => {
                        down_file_content = fs::read_to_string(entry.path()).unwrap();
                    }
                }
            }
        }
    }

    let up = Migration {
        id,
        name: name.clone(),
        migration_type: MigrationType::UP,
        content: up_file_content.to_owned(),
    };
    let down = Migration {
        id,
        name: name.clone(),
        migration_type: MigrationType::DOWN,
        content: down_file_content.to_owned(),
    };
    Some((up, down))
}
