pub fn delete_newline(str: &str) -> String {
    str.chars().filter(|c| c != &'\n' || c != &'\r').collect::<String>()
}
