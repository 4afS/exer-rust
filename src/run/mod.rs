extern crate glob;

use std::fs;
use std::process::Command;

enum Error {
    GitCommandNotFound,
    DotGitNotFound,
}

pub fn run() {
    match get_project_root_path() {
        Ok(path) => {
            let entries = fs::read_dir(&path.trim_end()).unwrap();
            for entry in entries {
                match entry {
                    Ok(file) => println!("{:?}", file),
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        Err(Error::GitCommandNotFound) => println!("git command not found."),
        Err(Error::DotGitNotFound) => println!(".git directory not found."),
    }
}

fn get_project_root_path() -> Result<String, Error> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("gi rev-parse --show-toplevel")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8(output.stdout).unwrap())
            } else {
                Err(Error::DotGitNotFound)
            }
        }
        Err(_) => Err(Error::GitCommandNotFound),
    }
}
