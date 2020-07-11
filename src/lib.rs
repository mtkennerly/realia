extern crate proc_macro;

mod attr;
mod expr;

use crate::attr::Then;
use crate::expr::Expr;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, ItemFn, Result};

macro_rules! res {
    ($name:ident) => {
        #[proc_macro_attribute]
        pub fn $name(args: TokenStream, input: TokenStream) -> TokenStream {
            cfg(stringify!($name), args, input)
        }
    };
}

res!(not);
res!(any);
res!(all);
res!(crate_available);
res!(crate_equals);
res!(crate_since);
res!(crate_before);
res!(crate_from_registry);
res!(env);
res!(env_equals);
res!(command);

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
