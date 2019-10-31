use super::types::Config;

pub fn generate_config() -> Vec<Config> {
    vec![
        Config::new(vec!["Cargo.toml", "Cargo.lock"], "cargo run"),
        Config::new(vec!["stack.yaml", "package.yaml"], "stack run"),
        Config::new(vec!["spago.dhall", ".spago"], "spago run"),
    ]
}
