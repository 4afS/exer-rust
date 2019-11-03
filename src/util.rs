use super::types::Config;

pub fn get_matches(files: Vec<&str>, configs: Vec<Config>) -> Option<Config> {
    for config in configs {
        if contains_any(&files, &config.matches) {
            return Some(config);
        }
    }
    None
}

fn contains_any(searched: &Vec<&str>, elements: &Vec<String>) -> bool {
    for element in elements {
        if searched.contains(&element.as_str()) {
            return true;
            }
    }
    false
}
