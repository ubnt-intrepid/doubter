use proc_macro2::Span;
use syn::parse::{Error as ParseError, Parse, ParseStream, Result as ParseResult};
use syn::punctuated::Punctuated;
use syn::{Ident, LitStr};

fn parse_error(message: impl ::std::fmt::Display) -> ParseError {
    ParseError::new(Span::call_site(), message)
}

#[derive(Debug)]
pub struct IncludeField {
    pub ident: Ident,
    pub eq: Token![=],
    pub value: LitStr,
}

#[derive(Debug)]
pub enum Field {
    Include(IncludeField),
}

impl Parse for Field {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let ident: Ident = input.parse()?;
        let eq = input.parse()?;
        let value = input.parse()?;

        if ident == "include" {
            Ok(Field::Include(IncludeField { ident, eq, value }))
        } else {
            Err(parse_error(format!("invalid key: {}", ident)))
        }
    }
}

#[derive(Debug)]
pub struct Input {
    pub fields: Punctuated<Field, Token![,]>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let fields = Punctuated::<Field, Token![,]>::parse_terminated(input)?;
        Ok(Input { fields })
    }
}
