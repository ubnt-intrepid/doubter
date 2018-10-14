use std::fmt;
use std::str::FromStr;

use proc_macro2::Span;
use syn;
use syn::parse;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, Lit};

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    Raw,
    Extract,
}

#[derive(Debug)]
pub struct Config {
    pub includes: Vec<String>,
    pub mode: Option<Mode>,
    pub use_external_doc: bool,
}

impl FromStr for Config {
    type Err = parse::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        syn::parse_str(s)
    }
}

impl Parse for Config {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let fields = Punctuated::<Field, Token![,]>::parse_terminated(input)?;

        let mut includes = vec![];
        let mut mode = None;
        let mut use_external_doc = false;

        for field in fields {
            match &*field.ident.to_string() {
                "include" => match field.value {
                    Lit::Str(s) => includes.push(s.value()),
                    _ => return Err(parse_error("unsupported literal type in 'include' field.")),
                },
                "mode" => match field.value {
                    Lit::Str(s) => match s.value().trim() {
                        "raw" => mode = Some(Mode::Raw),
                        "extract" => mode = Some(Mode::Extract),
                        s => return Err(parse_error(format!("invalid mode: {:?}", s))),
                    },
                    _ => return Err(parse_error("unsupported literal type in 'mode' field.")),
                },
                "use_external_doc" => match field.value {
                    Lit::Str(s) => match s.value().trim() {
                        "on" | "true" | "yes" => use_external_doc = true,
                        "off" | "false" | "no" => use_external_doc = false,
                        s => {
                            return Err(parse_error(format!(
                                "invalid value in `use_external_doc`: {}",
                                s
                            )))
                        }
                    },
                    Lit::Bool(b) => use_external_doc = b.value,
                    _ => {
                        return Err(parse_error(
                            "unsupported literal type in 'use_external_doc' field.",
                        ))
                    }
                },
                s => return Err(parse_error(format!("invalid key: {:?}", s))),
            }
        }

        Ok(Config {
            includes,
            mode,
            use_external_doc,
        })
    }
}

fn parse_error<D>(message: D) -> parse::Error
where
    D: fmt::Display,
{
    parse::Error::new(Span::call_site(), message)
}

#[derive(Debug)]
struct Field {
    ident: Ident,
    eq: Token![=],
    value: Lit,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        Ok(Field {
            ident: input.parse()?,
            eq: input.parse()?,
            value: input.parse()?,
        })
    }
}
