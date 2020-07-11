use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parenthesized, LitStr, Token};
use version_compare::{CompOp, VersionCompare};

include!(concat!(env!("OUT_DIR"), "/crate_info.rs"));

pub enum Expr {
    Not(Box<Expr>),
    Any(Vec<Expr>),
    All(Vec<Expr>),
    CrateAvailable(String),
    CrateEquals(String, String),
    CrateSince(String, String),
    CrateBefore(String, String),
    CrateFromRegistry(String),
    Env(String),
    EnvEquals(String, String),
    Command(String),
}

impl Expr {
    pub fn eval(&self) -> bool {
        use self::Expr::*;

        match self {
            Not(expr) => !expr.eval(),
            Any(exprs) => exprs.iter().any(|e| e.eval()),
            All(exprs) => exprs.iter().all(|e| e.eval()),
            CrateAvailable(name) => CRATES.iter().any(|x| x.name == name),
            CrateEquals(name, version) => CRATES
                .iter()
                .any(|x| x.name == name && VersionCompare::compare_to(&x.version, &version, &CompOp::Eq).unwrap()),
            CrateSince(name, version) => CRATES
                .iter()
                .any(|x| x.name == name && VersionCompare::compare_to(&x.version, &version, &CompOp::Ge).unwrap()),
            CrateBefore(name, version) => CRATES
                .iter()
                .any(|x| x.name == name && VersionCompare::compare_to(&x.version, &version, &CompOp::Lt).unwrap()),
            CrateFromRegistry(name) => CRATES.iter().any(|x| {
                x.name == name
                    && match x.source {
                        Some(source) => source.starts_with("registry+"),
                        _ => false,
                    }
            }),
            Env(name) => std::env::var(name).is_ok(),
            EnvEquals(name, value) => match std::env::var(name) {
                Ok(x) => &x == value,
                _ => false,
            },
            Command(name) => which::which(name).is_ok(),
        }
    }
}

type Exprs = Punctuated<Expr, Token![,]>;

mod keyword {
    syn::custom_keyword!(not);
    syn::custom_keyword!(any);
    syn::custom_keyword!(all);
    syn::custom_keyword!(crate_available);
    syn::custom_keyword!(crate_equals);
    syn::custom_keyword!(crate_since);
    syn::custom_keyword!(crate_before);
    syn::custom_keyword!(crate_from_registry);
    syn::custom_keyword!(env);
    syn::custom_keyword!(env_equals);
    syn::custom_keyword!(command);
}

impl Parse for Expr {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(keyword::not) {
            Self::parse_not(input)
        } else if lookahead.peek(keyword::any) {
            Self::parse_any(input)
        } else if lookahead.peek(keyword::all) {
            Self::parse_all(input)
        } else if lookahead.peek(keyword::crate_available) {
            Self::parse_crate_available(input)
        } else if lookahead.peek(keyword::crate_equals) {
            Self::parse_crate_equals(input)
        } else if lookahead.peek(keyword::crate_since) {
            Self::parse_crate_since(input)
        } else if lookahead.peek(keyword::crate_before) {
            Self::parse_crate_before(input)
        } else if lookahead.peek(keyword::crate_from_registry) {
            Self::parse_crate_from_registry(input)
        } else if lookahead.peek(keyword::env) {
            Self::parse_env(input)
        } else if lookahead.peek(keyword::env_equals) {
            Self::parse_env_equals(input)
        } else if lookahead.peek(keyword::command) {
            Self::parse_command(input)
        } else {
            Err(lookahead.error())
        }
    }
}

macro_rules! def_parser_1_string {
    ($fn_name:ident, $name:ident, $output:expr) => {
        fn $fn_name(input: ParseStream) -> Result<Self> {
            input.parse::<keyword::$name>()?;

            let paren;
            parenthesized!(paren in input);
            let arg1: LitStr = paren.parse()?;
            paren.parse::<Option<Token![,]>>()?;

            Ok($output(arg1.value()))
        }
    };
}

macro_rules! def_parser_2_strings {
    ($fn_name:ident, $name:ident, $output:expr) => {
        fn $fn_name(input: ParseStream) -> Result<Self> {
            input.parse::<keyword::$name>()?;

            let paren;
            parenthesized!(paren in input);
            let arg1: LitStr = paren.parse()?;
            paren.parse::<Token![,]>()?;
            let arg2: LitStr = paren.parse()?;

            Ok($output(arg1.value(), arg2.value()))
        }
    };
}

impl Expr {
    fn parse_not(input: ParseStream) -> Result<Self> {
        input.parse::<keyword::not>()?;

        let paren;
        parenthesized!(paren in input);
        let expr: Expr = paren.parse()?;
        paren.parse::<Option<Token![,]>>()?;

        Ok(Expr::Not(Box::new(expr)))
    }

    fn parse_any(input: ParseStream) -> Result<Self> {
        input.parse::<keyword::any>()?;

        let paren;
        parenthesized!(paren in input);
        let exprs: Exprs = paren.parse_terminated(Expr::parse)?;

        Ok(Expr::Any(exprs.into_iter().collect()))
    }

    fn parse_all(input: ParseStream) -> Result<Self> {
        input.parse::<keyword::all>()?;

        let paren;
        parenthesized!(paren in input);
        let exprs: Exprs = paren.parse_terminated(Expr::parse)?;

        Ok(Expr::All(exprs.into_iter().collect()))
    }

    def_parser_1_string!(parse_crate_available, crate_available, Expr::CrateAvailable);
    def_parser_2_strings!(parse_crate_equals, crate_equals, Expr::CrateEquals);
    def_parser_2_strings!(parse_crate_since, crate_since, Expr::CrateSince);
    def_parser_2_strings!(parse_crate_before, crate_before, Expr::CrateBefore);
    def_parser_1_string!(parse_crate_from_registry, crate_from_registry, Expr::CrateFromRegistry);
    def_parser_1_string!(parse_env, env, Expr::Env);
    def_parser_2_strings!(parse_env_equals, env_equals, Expr::EnvEquals);
    def_parser_1_string!(parse_command, command, Expr::Command);
}
