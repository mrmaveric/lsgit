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
    find_git_repositories(current_dir);
}

fn find_git_repositories(dir: std::path::PathBuf) {
    let entries = fs::read_dir(dir);
    if entries.is_err() {
        return;
    }
    let entries = entries.unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            if path.join(".git").is_dir() {
                println!("{}", path.display());
            }
            find_git_repositories(path);
        }
    }
}
