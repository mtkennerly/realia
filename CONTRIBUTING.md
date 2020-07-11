## Development
Rust 1.44.0 or newer is recommended.

* Run tests:
  * `REALIA_EMPTY='' REALIA_FULL=1 cargo test`
* Linting:
  * `cargo fmt`
  * `cargo clippy -- -D warnings`
* Activate pre-commit hooks (requires Python):
  ```
  pip install --user pre-commit
  pre-commit install
  ```
