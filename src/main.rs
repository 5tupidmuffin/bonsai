mod utils;

use std::{collections::BTreeMap, path::PathBuf, process::exit};

fn create_prefix(
    current_node: &PathBuf,
    current_depth: i32,
    stack: &mut Vec<(PathBuf, i32)>,
    depth_map: &mut BTreeMap<i32, bool>,
) -> String {
    let mut prefix = String::new();
    if current_depth == 0 {
        prefix = String::from(".");
        return prefix;
    }
    depth_map.insert(current_depth, true);
    let pipe_symbol = if stack.len() > 0 && stack.last().unwrap().1 == current_depth {
        "├"
    } else {
        depth_map.remove(&current_depth);
        "└"
    };
    for i in 0..current_depth {
        if depth_map.contains_key(&i) {
            prefix += "│"
        } else {
            prefix += "   "
        }
    }
    prefix += pipe_symbol;
    prefix
}

fn main() {
    let root = match utils::validate_and_get_directory_from_args() {
        Ok(dir) => dir,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };
    let mut stack: Vec<(PathBuf, i32)> = vec![(root, 0)];
    let mut depth_map = BTreeMap::new();
    while let Some((node, depth)) = stack.pop() {
        // pipe diagram symbols;  ─ ├ └ │
        println!(
            "{}{}",
            create_prefix(&node, depth, &mut stack, &mut depth_map),
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
