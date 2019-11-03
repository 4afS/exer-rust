extern crate glob;

use exer::config::generate_config;
use exer::types::Config;
use exer::util::get_matches;
use std::process::Command;

enum Error {
    GitCommandNotFound,
    DotGitNotFound,
}

pub fn run() {
    match get_project_root_path() {
        Ok(path) => {
            let entries = glob::glob(&format!("{}{}", path.trim_end(), "/*")).unwrap();
            let file_paths: Vec<_> = entries
                .into_iter()
                .filter(|entry| entry.is_ok())
                .map(|entry| entry.unwrap())
                .collect();
            let file_names: Vec<&str> = file_paths
                .iter()
                .filter_map(|x| x.file_name().and_then(|file_name| file_name.to_str()))
                .collect();
            let configs: Vec<Config> = generate_config();
            match get_matches(file_names, configs) {
                Some(config) => println!("{}", config.run_command),
                None => println!("Error: project not found."),
            }
        }
        Err(Error::GitCommandNotFound) => println!("Error: git command not found."),
        Err(Error::DotGitNotFound) => println!("Error: .git directory not found."),
    }
}

fn get_project_root_path() -> Result<String, Error> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("git rev-parse --show-toplevel")
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
