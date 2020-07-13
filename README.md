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
* Dependencies (accounts for target-specific ones, but not optional ones currently):
  * `#[realia::dep("your-crate", "foo")]`
    * Checks if your crate uses any version of the `foo` crate.
  * `#[realia::dep("your-crate", "foo", "1.2.3")]`
    * Checks if your crate uses the `foo` crate with exactly version 1.2.3.
  * `#[realia::dep_since("your-crate", "foo", "1.2.3")]`
    * Checks if your crate uses the `foo` crate with version 1.2.3 or newer.
  * `#[realia::dep_before("your-crate", "foo", "1.2.3")]`
    * Checks if your crate uses the `foo` crate with a version before 1.2.3.
  * `#[realia::dep_from_registry("your-crate", "foo")]`
    * Checks if your crate uses the `foo` crate from the registry (as opposed to
      being a `git` or `path` dependency). This is useful if you have
      [publishing fallbacks](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#multiple-locations).

The above can be refined or augmented by these additional attributes:

* `#[realia::not(env("FOO"))]`
  * Inverts the condition.
* `#[realia::any(env("FOO"), env("bar"))]`
  * Checks if any of the conditions are met.
* `#[realia::all(env("FOO"), env("bar"))]`
  * Checks if all of the conditions are met.
* `#[realia::attr(env("FOO"), some_attr)]`
  * Applies `#[some_attr]` if the condition is met.
    You can also specify `const` this way.

## Triggering build on changed conditions
If you use the `env` or `cmd` attributes,you'll need to include a `build.rs`
in your project with any environment variables you check.

```rust
fn main() {
    // Necessary when using #[realia::env("FOO")]
    println!("cargo:rerun-if-env-changed=FOO");

    // Necessary when using #[realia::cmd(...)]
    println!("cargo:rerun-if-env-changed=PATH");
}
```
