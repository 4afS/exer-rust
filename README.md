# exer

A run tool that identifies the used build tool.

## Usage

- Run **at the root directory of the project!**
  ```sh
  run
  ```
## Supported Languages and build-tools

| Language | Build-tool | Matches | Command |
|:--|:--|:--|:--|
| D | Dub | dub.json, dub.sdl | `dub run` |
| Elm | - | elm.json | `elm reactor` |
| Go | - | go.mod | `go run` |
| Haskell | Stack | stack.yaml | `stack run` |
| Java | gradle | build.gradle | `gradle run` |
| Kotlin | gradle | build.gradle | `gradle run` |
| Rust | Cargo | Cargo.toml, Cargo.lock | `cargo run` |
| Scala | sbt | build.sbt | `sbt run` |
| PureScript | spago | spago.dhall, .spago | `spago run` |

## Installation
- With Rust's package manager cargo, you can install `exer` via:
  ```sh
  git clone https://github.com/4afs/exer ~/.exer &&\
  cd ~/.exer &&\
  cargo build --release
  ```

- Add the following to your .bashrc
  ```bash
  run() {
      local result=$($HOME/.exer/target/release/exer run)
      if [[ $result =~ "Error:" ]]; then
        echo $result
      else
        sh -c "$result"
      fi
    }
  ```
