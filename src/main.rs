use std::fs;

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    find_git_repositories(current_dir);
}

fn find_git_repositories(dir: std::path::PathBuf) {
    for entry in fs::read_dir(dir).unwrap() {
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
