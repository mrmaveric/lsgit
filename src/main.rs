use std::env;
use std::env::current_dir;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let current_dir = if args.len() > 1 {
        std::path::PathBuf::from(&args[1])
    } else {
        current_dir().unwrap()
    };

    if !current_dir.is_dir() {
        eprintln!("{} is not a directory", current_dir.to_str().unwrap());
        return;
    }

    if is_git_dir(&current_dir.join(".git")) {
        println!("{}", current_dir.display());
    }

    if is_git_dir(&current_dir) && !is_named_git(&current_dir) {
        println!("{}", current_dir.display());
    }

    find_git_repositories(current_dir);
}

fn find_git_repositories(dir: std::path::PathBuf) {
    let entries = fs::read_dir(&dir);
    if entries.is_err() {
        eprintln!("Error reading dir {}: {:?}", dir.to_str().unwrap(), entries);
        return;
    }
    let entries = entries.unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            if is_git_dir(&path.join(".git")) {
                println!("{}", path.display());
            }
            if is_git_dir(&path) && !is_named_git(&path) {
                println!("{}", path.display());
            }
            find_git_repositories(path);
        }
    }
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
