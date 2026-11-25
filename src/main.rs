mod utils;

use std::{path::PathBuf, process::exit};

fn main() {
    let root = match utils::validate_and_get_directory_from_args() {
        Ok(dir) => dir,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };
    let mut stack: Vec<(PathBuf, i32)> = vec![(root, 0)];
    while let Some((node, depth)) = stack.pop() {
        // pipe diagram symbols;  ─ ├ └ │
        let prefix: String;
        if depth == 0 {
            prefix = String::from(".");
        } else if stack.len() > 0 {
            let next_node = stack.last().unwrap();
            if next_node.1 == depth {
                prefix = String::from(&format!("{}├── ", "    ".repeat(depth as usize - 1)));
            } else {
                prefix = String::from(&format!("{}└── ", "    ".repeat(depth as usize - 1)));
            }
        } else {
            prefix = String::from(&format!("{}└── ", "    ".repeat(depth as usize - 1)));
        }

        println!(
            "{}{}",
            prefix,
            node.file_name().unwrap().to_str().unwrap() // TODO: kinda sketchy, find a better way
        );
        if node.is_dir() {
            let dir_entries = match node.read_dir() {
                Ok(entries) => entries,
                Err(err) => {
                    eprintln!("failed to directory entries, error: {}", err);
                    continue;
                }
            };
            for entry in dir_entries {
                match entry {
                    Ok(entry) => {
                        stack.push((entry.path(), depth + 1));
                    }
                    Err(err) => {
                        eprintln!("failed to read directory entry, error: {}", err);
                    }
                }
            }
        }
    }
}
