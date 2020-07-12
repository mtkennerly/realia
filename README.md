# Realia

This crate provides macros for conditional compilation based on various checks.
These macros are analogous to `#[cfg(...)]` and `#[cfg_attr(...)]`.

Realia is inspired by and heavily based on [rustversion](https://crates.io/crates/rustversion).

## Attributes
Primary:

* Environment variables:
  * `#[realia::env("FOO")]`
    * Checks if the `FOO` environment variable exists.
  * `#[realia::env("FOO", "bar")]`
    * Checks if the `FOO` environment variable has the value `bar`.
* Executables:
  * `#[realia::cmd("foo")]`
    * Checks if the executable `foo` exists in the `PATH` environment variable.

The above can be refined or augmented by these additional attributes:

* `#[realia::not(env("foo"))]`
  * Inverts the condition.
* `#[realia::any(env("foo"), env("bar"))]`
  * Checks if any of the conditions are met.
* `#[realia::all(env("foo"), env("bar"))]`
  * Checks if all of the conditions are met.
* `#[realia::attr(env("foo"), some_attr)]`
  * Applies `#[some_attr]` if the condition is met.
    You can also specify `const` this way.

## Triggering build on changed conditions
To trigger builds when the conditions change, you'll need to include a
`build.rs` in your project with the environment variables you check.

```rust
fn main() {
    // Necessary when using #[realia::env("FOO")]
    println!("cargo:rerun-if-env-changed=FOO");

    // Necessary when using #[realia::cmd(...)]
    println!("cargo:rerun-if-env-changed=PATH");
}
```
