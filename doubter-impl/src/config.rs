#[derive(Debug)]
pub struct Config {
    pub includes: Vec<String>,
    _priv: (),
}

mod parsing {
    use super::Config;

    use proc_macro2::Span;
    use std::str::FromStr;
    use syn;
    use syn::parse;
    use syn::parse::{Parse, ParseStream};
    use syn::punctuated::Punctuated;
    use syn::{Ident, LitStr};

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

            for field in fields {
                if field.ident == "include" {
                    includes.push(field.value.value());
                } else {
                    return Err(parse_error(format!("invalid key: {}", field.ident)));
                }
            }

            Ok(Config {
                includes,
                _priv: (),
            })
        }
    }

    #[derive(Debug)]
    struct Input {
        fields: Punctuated<Field, Token![,]>,
    }

    impl Parse for Input {
        fn parse(input: ParseStream) -> parse::Result<Self> {
            let fields = Punctuated::<Field, Token![,]>::parse_terminated(input)?;
            Ok(Input { fields })
        }
    }

    fn parse_error(message: impl ::std::fmt::Display) -> parse::Error {
        parse::Error::new(Span::call_site(), message)
    }

    #[derive(Debug)]
    struct Field {
        ident: Ident,
        eq: Token![=],
        value: LitStr,
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

}
