use std::env;
use std::env::current_dir;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
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

fn find_git_repositories(dir: std::path::PathBuf) -> Result<(), Box<dyn Error>> {
    if !dir.is_dir() {
        return Ok(());
    }
    if is_git_dir(&dir.join(".git")) || (is_git_dir(&dir) && !is_named_git(&dir)) {
        println!("{}", dir.display());
    }

    for entry in fs::read_dir(&dir)? {
        let path = entry?.path();
        if path.is_dir() {
            _ = find_git_repositories(path);
        }
    }
    Ok(())
}

fn is_git_dir(path: &std::path::PathBuf) -> bool {
    let git_files = ["HEAD", "config", "description", "hooks", "objects", "refs"];

    git_files
        .iter()
        .filter(|file| path.join(file).exists())
        .copied()
        .collect::<Vec<&str>>()
        .len()
        == 6
}

fn is_named_git(path: &std::path::PathBuf) -> bool {
    if let Some(p) = path.file_name() {
        if let Some(ps) = p.to_str() {
            return ps == ".git";
        }
    }
    false
}
