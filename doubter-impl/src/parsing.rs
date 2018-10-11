use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::punctuated::Punctuated;
use syn::{Ident, LitStr};

#[derive(Debug)]
pub struct KeyValue {
    pub key: Ident,
    pub eq: Token![=],
    pub value: LitStr,
}

impl Parse for KeyValue {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let key = input.parse()?;
        let eq = input.parse()?;
        let value = input.parse()?;
        Ok(KeyValue { key, eq, value })
    }
}

#[derive(Debug)]
pub struct Input {
    pub files: Punctuated<KeyValue, Token![,]>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let files = Punctuated::<KeyValue, Token![,]>::parse_terminated(input)?;
        Ok(Input { files })
    }
}
