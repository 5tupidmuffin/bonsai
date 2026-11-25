use std::{
    path::{Path, PathBuf},
    process::exit,
};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: bonsai [path]");
        exit(1);
    }
    if args[1] == "--help" {
        println!("Usage: bonsai [path]");
        exit(0);
    }
    let Ok(dir) = Path::new(&args[1]).canonicalize() else {
        eprintln!("please provide a valid directory path.");
        exit(1);
    };
    if !dir.is_dir() {
        eprintln!("please provide a valid directory path.");
        exit(1);
    }

    let root = dir;
    let mut stack: Vec<(PathBuf, i32)> = vec![(root, 0)];
    while let Some((node, depth)) = stack.pop() {
        // pipe diagram symbols;  ─ ├ └ │
        let mut prefix = String::new();
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
