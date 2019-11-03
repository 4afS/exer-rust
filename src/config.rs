use super::types::Config;

pub fn generate_config() -> Vec<Config> {
    vec![
        // Rust
        Config::new(vec!["Cargo.toml", "Cargo.lock"], "cargo run"),

        // Haskell
        Config::new(vec!["stack.yaml"], "stack run"),

        // PureScript
        Config::new(vec!["spago.dhall", ".spago"], "spago run"),

        // Elm
        Config::new(vec!["elm.json"], "elm reactor"),

        // Java and Kotlin
        Config::new(vec!["build.gradle"], "gradle run"),

        // Scala
        Config::new(vec!["build.sbt"], "sbt run"),

        // Golang
        Config::new(vec!["go.mod"], "go run"),
        
        // Dlang
        Config::new(vec!["dub.json", "dub.sdl"], "dub run"),
    ]
}
