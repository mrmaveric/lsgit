//! lsgit is a command line application that lists git resositories recursivly.
//! lsgit can be called without any arguements and will start it's search in the current directory.
//!
//! If called with a valid path as it's first command line arguement, it will start it's search
//! in the path provided.
use std::collections::VecDeque;
use std::env;
use std::env::current_dir;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn main() {
    let depth_first = env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .contains(&String::from("-d"));
    let args: Vec<String> = env::args()
        .filter(|arg| arg != &String::from("-d"))
        .collect();

    let base_dir = if args.len() > 0 {
        std::path::PathBuf::from(&args[0])
    } else {
        if let Ok(cd) = current_dir() {
            cd
        } else {
            std::path::PathBuf::from("/")
        }
    };

    if !base_dir.is_dir() {
        eprintln!(
            "{} is not a directory",
            if let Some(e) = base_dir.to_str() {
                e
            } else {
                ""
            }
        );
        return;
    }

    if depth_first {
        _ = find_git_repositories(base_dir);
    } else {
        _ = find_git_repositories_breadth_first(base_dir);
    }
}

/// Recursive function that prints the current directories path if it is a git repo
/// then calls it's self on all sub-directories.
fn find_git_repositories(dir: std::path::PathBuf) -> Result<(), Box<dyn Error>> {
    if (is_git_dir(&dir) || is_git_dir(&dir.join(".git"))) && !is_named_git(&dir) {
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

/// Function that prints the current directories path if it is a git repo
/// then calls it's self on all sub-directories.
fn find_git_repositories_breadth_first(dir: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut queue = VecDeque::new();
    queue.push_back(dir);

    while let Some(dir) = queue.pop_front() {
        if (is_git_dir(&dir) || is_git_dir(&dir.join(".git"))) && !is_named_git(&dir) {
            println!("{}", dir.display());
        }

        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries {
                let path = entry?.path();
                if path.is_dir() {
                    queue.push_back(path);
                }
            }
        }
    }

    Ok(())
}

/// Looks for common files found in the root of a git directory. Returns true if all
/// files in the list are found
fn is_git_dir(path: &std::path::PathBuf) -> bool {
    let git_files = ["HEAD", "config", "description", "hooks", "objects", "refs"];

    git_files
        .iter()
        .filter(|file| path.join(file).exists())
        .copied()
        .collect::<Vec<&str>>()
        .len()
        == git_files.len()
}

/// Checks if the directory provided is named ".git"
fn is_named_git(path: &std::path::PathBuf) -> bool {
    if let Some(p) = path.file_name() {
        if let Some(ps) = p.to_str() {
            return ps == ".git";
        }
    }
    false
}
