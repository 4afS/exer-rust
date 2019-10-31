use super::types::Config;

pub fn get_matches(files: Vec<&str>, configs: Vec<Config>) -> Option<Config> {
    for config in configs {
        if elems(&files, &config.matches) {
            return Some(config);
        }
    }
    None
}

fn elems(l: &Vec<&str>, es: &Vec<String>) -> bool {
    for e in es {
        if l.contains(&e.as_str()) {
            return true;
        }
    }
    false
}
