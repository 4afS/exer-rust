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
| Rust | Cargo | Cargo.toml, Cargo.lock | `cargo run` |
| Scala | sbt | build.sbt | `sbt run` |
| Haskell | Stack | stack.yaml, package.yaml | `stack run` |
| PureScript | spago | spago.dhall, .spago | `spago run` |
| Elm | elm | elm.json | `elm reactor` |

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
