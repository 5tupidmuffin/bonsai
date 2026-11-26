use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

pub fn validate_and_get_directory_from_args() -> Result<PathBuf, (String, i32)> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        return Err(("Usage: bonsai [path]".into(), 1));
    }
    if args[1] == "--help" {
        return Err(("Usage: bonsai [path]".into(), 0));
    }
    let Ok(dir) = Path::new(&args[1]).canonicalize() else {
        return Err(("please provide a valid directory path.".into(), 1));
    };
    if !dir.is_dir() {
        return Err(("please provide a valid directory path.".into(), 1));
    }
    Ok(dir)
}

pub fn create_prefix(
    current_depth: i32,
    stack: &mut Vec<(PathBuf, i32)>,
    depth_map: &mut BTreeMap<i32, bool>,
) -> String {
    // pipe diagram symbols;  ─ ├ └ │
    let mut prefix = String::new();
    if current_depth == 0 {
        prefix = String::from(".");
        return prefix;
    }
    depth_map.insert(current_depth, true);
    let pipe_symbol = if stack.len() > 0 && stack.last().unwrap().1 == current_depth {
        "├─── "
    } else {
        depth_map.remove(&current_depth);
        "└─── "
    };
    for i in 1..current_depth {
        if depth_map.contains_key(&i) {
            prefix += "│    "
        } else {
            prefix += "     "
        }
    }
    prefix += pipe_symbol;
    prefix
}
