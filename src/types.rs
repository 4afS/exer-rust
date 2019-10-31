pub struct Config {
    pub matches: Vec<String>,
    pub run_command: String,
}

impl Config {
    pub fn new(matches_str: Vec<&str>, run_command_str: &str) -> Self {
        Config {
            matches: matches_str
                .into_iter()
                .map(|match_| match_.to_string())
                .collect::<Vec<String>>(),
            run_command: run_command_str.to_string(),
        }
    }
}
