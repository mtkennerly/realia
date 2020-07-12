use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parenthesized, LitStr, Token};

pub enum Expr {
    Not(Box<Expr>),
    Any(Vec<Expr>),
    All(Vec<Expr>),
    EnvExists(String),
    EnvEquals(String, String),
    CmdExists(String),
}

impl Expr {
    pub fn eval(&self) -> bool {
        use self::Expr::*;

        match self {
            Not(expr) => !expr.eval(),
            Any(exprs) => exprs.iter().any(|e| e.eval()),
            All(exprs) => exprs.iter().all(|e| e.eval()),
            EnvExists(name) => std::env::var(name).is_ok(),
            EnvEquals(name, value) => match std::env::var(name) {
                Ok(x) => &x == value,
                _ => false,
            },
            CmdExists(name) => which::which(name).is_ok(),
        }
    }
}

type Exprs = Punctuated<Expr, Token![,]>;

mod keyword {
    syn::custom_keyword!(not);
    syn::custom_keyword!(any);
    syn::custom_keyword!(all);
    syn::custom_keyword!(env);
    syn::custom_keyword!(cmd);
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
        } else if lookahead.peek(keyword::env) {
            Self::parse_env(input)
        } else if lookahead.peek(keyword::cmd) {
            Self::parse_cmd(input)
        } else {
            Err(lookahead.error())
        }
    }
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

    fn parse_env(input: ParseStream) -> Result<Self> {
        input.parse::<keyword::env>()?;

        let paren;
        parenthesized!(paren in input);
        let arg1: LitStr = paren.parse()?;
        match paren.parse::<Token![,]>() {
            Ok(_) => {
                let arg2: LitStr = paren.parse()?;
                Ok(Expr::EnvEquals(arg1.value(), arg2.value()))
            }
            Err(_) => Ok(Expr::EnvExists(arg1.value())),
        }
    }

    fn parse_cmd(input: ParseStream) -> Result<Self> {
        input.parse::<keyword::cmd>()?;

        let paren;
        parenthesized!(paren in input);
        let arg1: LitStr = paren.parse()?;

        Ok(Expr::CmdExists(arg1.value()))
    }
}
