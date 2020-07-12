## Development
Rust 1.44.0 or newer is recommended.

For testing, you'll need to set the environment variables defined in `.envrc`.

* Run tests:
  * `cargo test`
* Linting:
  * `cargo fmt`
  * `cargo clippy --tests -- -D warnings`
* Activate pre-commit hooks (requires Python):
  ```
  pip install --user pre-commit
  pre-commit install
  ```
