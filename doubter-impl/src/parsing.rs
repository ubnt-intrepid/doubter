use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::punctuated::Punctuated;
use syn::{Ident, LitStr};

#[derive(Debug)]
pub struct Field {
    pub key: Ident,
    pub eq: Token![=],
    pub value: LitStr,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let key = input.parse()?;
        let eq = input.parse()?;
        let value = input.parse()?;
        Ok(Field { key, eq, value })
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
