use std::path::{Path, PathBuf};

pub fn validate_and_get_directory_from_args() -> Result<PathBuf, String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        return Err("Usage: bonsai [path]".into());
    }
    if args[1] == "--help" {
        return Err("Usage: bonsai [path]".into());
    }
    let Ok(dir) = Path::new(&args[1]).canonicalize() else {
        return Err("please provide a valid directory path.".into());
    };
    if !dir.is_dir() {
        return Err("please provide a valid directory path.".into());
    }
    Ok(dir)
}
