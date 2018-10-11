#[macro_use]
extern crate proc_macro_hack;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use std::env;
use std::fs;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use proc_macro2::Span;
use syn::punctuated::Punctuated;

#[derive(Debug)]
struct KeyValue {
    key: syn::Ident,
    eq: Token![=],
    value: syn::LitStr,
}

impl syn::parse::Parse for KeyValue {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let key = input.parse()?;
        let eq = input.parse()?;
        let value = input.parse()?;
        Ok(KeyValue { key, eq, value })
    }
}

struct Input {
    files: Punctuated<KeyValue, Token![,]>,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let files = Punctuated::<KeyValue, Token![,]>::parse_terminated(input)?;
        Ok(Input { files })
    }
}

proc_macro_item_impl! {
    pub fn doubter_impl(input: &str) -> String {
        let input: Input = syn::parse_str(input).unwrap();

        let mut items = vec![];
        for file in input.files {
            assert_eq!(file.key, "file");

            let escaped_name = file.value.value().replace('/', "_").replace('.', "_");
            let ident = syn::Ident::new(
                &format!("doubter_doctest_{}", escaped_name),
                Span::call_site(),
            );

            let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap();
            let doc_path = cargo_manifest_dir.join(file.value.value());

            let file = fs::OpenOptions::new().read(true).open(doc_path).unwrap();
            let file = BufReader::new(file);
            let lines: Vec<String> = file.lines().collect::<io::Result<Vec<String>>>().unwrap();

            items.push(quote!(
                #(#[doc = #lines])*
                #[allow(dead_code)]
                pub const #ident : () = ();
            ));
        }

        quote!(
            #(#items)*
        ).to_string()
    }
}
