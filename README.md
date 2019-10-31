# exer
A run tool that identifies the used build tool.

# Usage
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
