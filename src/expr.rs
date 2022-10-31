use crate::dep::{get_dep, get_deps};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    LitStr, Token,
};
use version_compare::{CompOp, VersionCompare};

pub enum Expr {
    Not(Box<Expr>),
    Any(Vec<Expr>),
    All(Vec<Expr>),
    EnvExists {
        name: String,
    },
    EnvEquals {
        name: String,
        value: String,
    },
    CmdExists {
        name: String,
    },
    DepExists {
        anchor: String,
        name: String,
    },
    DepEquals {
        anchor: String,
        name: String,
        version: String,
    },
    DepSince {
        anchor: String,
        name: String,
        version: String,
    },
    DepBefore {
        anchor: String,
        name: String,
        version: String,
    },
    DepFromRegistry {
        anchor: String,
        name: String,
    },
}

impl Expr {
    pub fn eval(&self) -> bool {
        use self::Expr::*;

        match self {
            Not(expr) => !expr.eval(),
            Any(exprs) => exprs.iter().any(|e| e.eval()),
            All(exprs) => exprs.iter().all(|e| e.eval()),
            EnvExists { name } => std::env::var(name).is_ok(),
            EnvEquals { name, value } => match std::env::var(name) {
                Ok(x) => &x == value,
                _ => false,
            },
            CmdExists { name } => which::which(name).is_ok(),
            DepExists { anchor, name } => get_dep(anchor, name).is_ok(),
            DepEquals { anchor, name, version } => get_deps(anchor).unwrap().iter().any(|dep| {
                &dep.name == name && VersionCompare::compare_to(&dep.version, version, &CompOp::Eq).unwrap()
            }),
            DepSince { anchor, name, version } => get_deps(anchor).unwrap().iter().any(|dep| {
                &dep.name == name && VersionCompare::compare_to(&dep.version, version, &CompOp::Ge).unwrap()
            }),
            DepBefore { anchor, name, version } => get_deps(anchor).unwrap().iter().any(|dep| {
                &dep.name == name && VersionCompare::compare_to(&dep.version, version, &CompOp::Lt).unwrap()
            }),
            DepFromRegistry { anchor, name } => get_deps(anchor).unwrap().iter().any(|dep| {
                &dep.name == name
                    && match &dep.source {
                        Some(source) => source.starts_with("registry+"),
                        _ => false,
                    }
            }),
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
    syn::custom_keyword!(dep);
    syn::custom_keyword!(dep_since);
    syn::custom_keyword!(dep_before);
    syn::custom_keyword!(dep_from_registry);
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
        } else if lookahead.peek(keyword::dep) {
            Self::parse_dep(input)
        } else if lookahead.peek(keyword::dep_since) {
            Self::parse_dep_since(input)
        } else if lookahead.peek(keyword::dep_before) {
            Self::parse_dep_before(input)
        } else if lookahead.peek(keyword::dep_from_registry) {
            Self::parse_dep_from_registry(input)
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
                Ok(Expr::EnvEquals {
                    name: arg1.value(),
                    value: arg2.value(),
                })
            }
            Err(_) => Ok(Expr::EnvExists { name: arg1.value() }),
        }
    }

    fn parse_cmd(input: ParseStream) -> Result<Self> {
        input.parse::<keyword::cmd>()?;

        let paren;
        parenthesized!(paren in input);
        let arg1: LitStr = paren.parse()?;

        Ok(Expr::CmdExists { name: arg1.value() })
    }

    fn parse_dep(input: ParseStream) -> Result<Self> {
        input.parse::<keyword::dep>()?;

        let paren;
        parenthesized!(paren in input);
        let arg1: LitStr = paren.parse()?;
        paren.parse::<Token![,]>()?;
        let arg2: LitStr = paren.parse()?;
        match paren.parse::<Token![,]>() {
            Ok(_) => {
                let arg3: LitStr = paren.parse()?;
                Ok(Expr::DepEquals {
                    anchor: arg1.value(),
                    name: arg2.value(),
                    version: arg3.value(),
                })
            }
            Err(_) => Ok(Expr::DepExists {
                anchor: arg1.value(),
                name: arg2.value(),
            }),
        }
    }

    fn parse_dep_since(input: ParseStream) -> Result<Self> {
        input.parse::<keyword::dep_since>()?;

        let paren;
        parenthesized!(paren in input);
        let arg1: LitStr = paren.parse()?;
        paren.parse::<Token![,]>()?;
        let arg2: LitStr = paren.parse()?;
        paren.parse::<Token![,]>()?;
        let arg3: LitStr = paren.parse()?;
        Ok(Expr::DepSince {
            anchor: arg1.value(),
            name: arg2.value(),
            version: arg3.value(),
        })
    }

    fn parse_dep_before(input: ParseStream) -> Result<Self> {
        input.parse::<keyword::dep_before>()?;

        let paren;
        parenthesized!(paren in input);
        let arg1: LitStr = paren.parse()?;
        paren.parse::<Token![,]>()?;
        let arg2: LitStr = paren.parse()?;
        paren.parse::<Token![,]>()?;
        let arg3: LitStr = paren.parse()?;
        Ok(Expr::DepBefore {
            anchor: arg1.value(),
            name: arg2.value(),
            version: arg3.value(),
        })
    }

    fn parse_dep_from_registry(input: ParseStream) -> Result<Self> {
        input.parse::<keyword::dep_from_registry>()?;

        let paren;
        parenthesized!(paren in input);
        let arg1: LitStr = paren.parse()?;
        paren.parse::<Token![,]>()?;
        let arg2: LitStr = paren.parse()?;
        Ok(Expr::DepFromRegistry {
            anchor: arg1.value(),
            name: arg2.value(),
        })
    }
}
