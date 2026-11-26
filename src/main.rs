mod utils;

use std::{
    collections::HashMap,
    io::{self, BufWriter, Write},
    path::PathBuf,
    process::exit,
};

fn main() {
    let root = match utils::validate_and_get_directory_from_args() {
        Ok(dir) => dir,
        Err((err, exit_code)) => {
            eprintln!("{}", err);
            exit(exit_code);
        }
    };

    let stdout = io::stdout();
    let mut output = BufWriter::new(stdout.lock());

    // DFS with depth tracking
    let mut stack: Vec<(PathBuf, i32)> = vec![(root, 0)];
    let mut depth_map: HashMap<i32, bool> = HashMap::new();
    while let Some((node, depth)) = stack.pop() {
        if let Err(err) = writeln!(
            output,
            "{}{}",
            utils::create_prefix(depth, &mut stack, &mut depth_map),
            node.file_name()
                .unwrap()
                .to_str()
                .unwrap_or("[FAILED TO READ PATH/FILE]")
        ) {
            eprintln!("failed to write to buffer, error: {}", err);
            exit(1);
        };

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
    if let Err(err) = output.flush() {
        eprintln!("failed to print tree, error = {}", err);
        exit(1);
    }
}
