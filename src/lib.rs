//! Realia provides attribute macros for conditional compilation,
//! analogous to `#[cfg(...)]` and `#[cfg_attr(...)]`.

#![allow(clippy::needless_doctest_main)]

extern crate proc_macro;

mod attr;
mod dep;
mod expr;
mod metadata;

use crate::attr::Then;
use crate::expr::Expr;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, ItemFn, Result};

/// Checks whether an environment variable is defined and optionally
/// what value it has.
///
/// If you use this attribute, your project should include a `build.rs` that
/// triggers a rebuild when any environment variables of interest change:
///
/// ```
/// fn main() {
///     // Necessary when using #[realia::env("FOO")]
///     println!("cargo:rerun-if-env-changed=FOO");
/// }
/// ```
///
/// # Example
/// ```
/// #[realia::env("CI")]
/// fn example() {
///     println!("CI is defined");
/// }
/// ```
///
/// ```
/// #[realia::env("CI", "true")]
/// fn example() {
///     println!("CI is set to true");
/// }
/// ```
#[proc_macro_attribute]
pub fn env(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("env", args, input)
}

/// Checks whether an executable exists on the `PATH`.
///
/// If you use this attribute, your project should include a `build.rs` that
/// triggers a rebuild when the `PATH` environment variable changes:
///
/// ```
/// fn main() {
///     println!("cargo:rerun-if-env-changed=PATH");
/// }
/// ```
///
/// # Example
/// ```
/// #[realia::cmd("git")]
/// fn example() {
///     println!("Git is installed and available");
/// }
/// ```
#[proc_macro_attribute]
pub fn cmd(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("cmd", args, input)
}

/// Checks whether a crate has a certain dependency and optionally if that
/// dependency is a certain version.
///
/// The first argument is an "anchor crate", which should generally be your crate.
/// Since this functionality is implemented by checking the output of
/// `cargo metadata --manifest-path $CARGO_MANIFEST_DIR/Cargo.toml`,
/// the anchor is used to ensure consistent results when `$CARGO_MANIFEST_DIR`
/// changes (e.g., when building your crate directly vs when building a downstream crate).
///
/// This accounts for target-specific dependencies, but currently ignores any
/// optional dependencies enabled by features.
///
/// # Example
/// ```
/// #[realia::dep("realia", "syn")]
/// fn example() {
///     println!("Realia depends on Syn");
/// }
/// ```
///
/// ```
/// #[realia::dep("realia", "syn", "1.0.34")]
/// fn example() {
///     println!("Realia uses Syn 1.0.34 exactly");
/// }
/// ```
#[proc_macro_attribute]
pub fn dep(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("dep", args, input)
}

/// Checks whether a crate has a certain dependency at or above a certain version.
///
/// The first argument is an "anchor crate", which should generally be your crate.
/// Since this functionality is implemented by checking the output of
/// `cargo metadata --manifest-path $CARGO_MANIFEST_DIR/Cargo.toml`,
/// the anchor is used to ensure consistent results when `$CARGO_MANIFEST_DIR`
/// changes (e.g., when building your crate directly vs when building a downstream crate).
///
/// This accounts for target-specific dependencies, but currently ignores any
/// optional dependencies enabled by features.
///
/// # Example
/// ```
/// #[realia::dep_since("realia", "syn", "1.0.34")]
/// fn example() {
///     println!("Realia uses Syn 1.0.34 or newer");
/// }
/// ```
#[proc_macro_attribute]
pub fn dep_since(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("dep_since", args, input)
}

/// Checks whether a crate has a certain dependency below a certain version.
///
/// The first argument is an "anchor crate", which should generally be your crate.
/// Since this functionality is implemented by checking the output of
/// `cargo metadata --manifest-path $CARGO_MANIFEST_DIR/Cargo.toml`,
/// the anchor is used to ensure consistent results when `$CARGO_MANIFEST_DIR`
/// changes (e.g., when building your crate directly vs when building a downstream crate).
///
/// This accounts for target-specific dependencies, but currently ignores any
/// optional dependencies enabled by features.
///
/// # Example
/// ```
/// #[realia::dep_before("realia", "syn", "1.0.34")]
/// fn example() {
///     println!("Realia uses Syn 1.0.33 or older");
/// }
/// ```
#[proc_macro_attribute]
pub fn dep_before(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("dep_before", args, input)
}

/// Checks whether a crate has a certain dependency installed from the registry
/// (as opposed to being a Git dependency or a path dependency). This is useful
/// if you have non-registry dependencies with a
/// [registry fallback for publishing](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#multiple-locations).
///
/// The first argument is an "anchor crate", which should generally be your crate.
/// Since this functionality is implemented by checking the output of
/// `cargo metadata --manifest-path $CARGO_MANIFEST_DIR/Cargo.toml`,
/// the anchor is used to ensure consistent results when `$CARGO_MANIFEST_DIR`
/// changes (e.g., when building your crate directly vs when building a downstream crate).
///
/// This accounts for target-specific dependencies, but currently ignores any
/// optional dependencies enabled by features.
///
/// # Example
/// ```
/// #[realia::dep_from_registry("realia", "syn")]
/// fn example() {
///     println!("Realia uses Syn from the registry");
/// }
/// ```
#[proc_macro_attribute]
pub fn dep_from_registry(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("dep_from_registry", args, input)
}

/// Inverts another condition.
///
/// # Example
/// ```
/// #[realia::not(cmd("git"))]
/// fn example() {
///     println!("Git is not installed");
/// }
/// ```
#[proc_macro_attribute]
pub fn not(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("not", args, input)
}

/// Checks if at least one of multiple conditions is met.
///
/// # Example
/// ```
/// #[realia::any(cmd("git"), cmd("hg"))]
/// fn example() {
///     println!("Some version control is available");
/// }
/// ```
#[proc_macro_attribute]
pub fn any(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("any", args, input)
}

/// Checks if multiple conditions are met.
///
/// # Example
/// ```
/// #[realia::all(cmd("git"), env("GIT_AUTHOR_NAME"))]
/// fn example() {
///     println!("Git is available and GIT_AUTHOR_NAME is defined");
/// }
/// ```
#[proc_macro_attribute]
pub fn all(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("all", args, input)
}

fn cfg(top: &str, args: TokenStream, input: TokenStream) -> TokenStream {
    match try_cfg(top, args, input) {
        Ok(tokens) => tokens,
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}

fn try_cfg(top: &str, args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let args = TokenStream2::from(args);
    let top = Ident::new(top, Span::call_site());

    let mut full_args = quote!(#top);
    if !args.is_empty() {
        full_args.extend(quote!((#args)));
    }

    let expr: Expr = syn::parse2(full_args)?;

    if expr.eval() {
        Ok(input)
    } else {
        Ok(TokenStream::new())
    }
}

/// Applies an attribute when the condition is met.
/// You can also specify `const` this way.
///
/// # Example
/// ```
/// #[test]
/// #[realia::attr(not(cmd("git")), ignore)]
/// fn some_test_that_requires_git() {}
/// ```
///
/// ```
/// #[realia::attr(env("USE_CONST_FN"), const)]
/// fn this_becomes_const() {}
/// ```
#[proc_macro_attribute]
pub fn attr(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as attr::Args);

    match try_attr(args, input) {
        Ok(tokens) => tokens,
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}

fn try_attr(args: attr::Args, input: TokenStream) -> Result<TokenStream> {
    if !args.condition.eval() {
        return Ok(input);
    }

    match args.then {
        Then::Const(const_token) => {
            let mut input: ItemFn = syn::parse(input)?;
            input.sig.constness = Some(const_token);
            Ok(TokenStream::from(quote!(#input)))
        }
        Then::Attribute(then) => {
            let input = TokenStream2::from(input);
            Ok(TokenStream::from(quote! {
                #[cfg_attr(all(), #then)]
                #input
            }))
        }
    }
}
