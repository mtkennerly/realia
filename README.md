# Realia

This crate provides macros for conditional compilation based on various checks,
such as crate versions and environment variables. These macros are analogous to
`#[cfg(...)]` and `#[cfg_attr(...)]`.

Realia is inspired by and heavily based on [rustversion](https://crates.io/crates/rustversion).

## Attributes
Primary:

* Crate info, based on the content of Cargo.lock:
  * `#[realia::crate_available("foo")]`
    * Checks if any version of the crate is installed.
  * `#[realia::crate_equals("foo", "1.2.3")]`
    * Checks if the `foo` crate is installed with exactly version 1.2.3.
  * `#[realia::crate_since("foo", "1.2.3")]`
    * Checks if the `foo` crate is installed with version 1.2.3 or newer.
  * `#[realia::crate_before("foo", "1.2.3")]`
    * Checks if the `foo` crate is installed with a version before 1.2.3.
  * `#[realia::crate_from_registry("foo")]`
    * Checks if the `foo` crate is installed from the registry (as opposed to
      being a `git` or `path` dependency).
* Environment variables:
  * `#[realia::env("foo")]`
    * Checks if the `foo` environment variable exists.
  * `#[realia::env_equals("foo", "bar")]`
    * Checks if the `foo` environment variable has the value `bar`.
* Executables:
  * `#[realia::command("foo")]`
    * Checks if the executable `foo` exists in the PATH.

The above can be refined or augmented by these additional attributes:

* `#[realia::not(crate_available("foo"))]`
  * Inverts the condition.
* `#[realia::any(crate_available("foo"), crate_available("bar"))]`
  * Checks if any of the conditions are met.
* `#[realia::all(crate_available("foo"), crate_available("bar"))]`
  * Checks if all of the conditions are met.
* `#[realia::attr(crate_available("foo"), some_attr)]`
  * Applies `#[some_attr]` if the condition is met.
    You can also specify `const` this way.
