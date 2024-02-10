use std::env;
use std::env::current_dir;
use std::fs;
use std::io::Error;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().skip(1).collect();

    let base_dir = if args.len() > 0 {
        std::path::PathBuf::from(&args[0])
    } else {
        current_dir()?
    };

    if !base_dir.is_dir() {
        eprintln!("{} is not a directory", base_dir.to_str().unwrap());
        return Ok(());
    }
    _ = find_git_repositories(base_dir);
    Ok(())
}

fn find_git_repositories(dir: std::path::PathBuf) -> Result<(), Error> {
    if !dir.is_dir() {
        return Ok(());
    }
    if is_git_dir(&dir.join(".git")) {
        println!("{}", dir.display());
    }
    if is_git_dir(&dir) && !is_named_git(&dir) {
        println!("{}", dir.display());
    }

    let entries = fs::read_dir(&dir)?;
    for entry in entries {
        let path = entry?.path();
        if path.is_dir() {
            _ = find_git_repositories(path);
        }
    }
    Ok(())
}

fn is_git_dir(path: &std::path::PathBuf) -> bool {
    let git_files = ["HEAD", "config", "description", "hooks", "objects", "refs"];

    for file in git_files.iter() {
        let git_path = path.join(file);
        if !git_path.exists() {
            return false;
        }
    }

    true
}

fn is_named_git(path: &std::path::PathBuf) -> bool {
    path.file_name().unwrap().to_str().unwrap() == ".git"
}
