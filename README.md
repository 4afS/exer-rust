# exer
A run tool that identifies the used build tool.

## Usage
- Add the following to your .bashrc
  ```bash
  run() {
      local result=$(exer run)
      if [[ $result =~ "Error:" ]]; then
        echo $result
      else
        sh -c "$result"
      fi
    }
  ```

- run a project!!!
  ```sh
  run
  ```
## Configuration

| Language | Matches | Command |
|:--:|:--:|:--:|
| Rust | Cargo.toml, Cargo.lock | `cargo run` |
| Scala | build.sbt | `sbt run` |
| Haskell | stack.yaml, package.yaml | `stack run` |
| PureScript | spago.dhall, .spago | `spago run` |
| Elm | elm.json | `elm reactor` |
