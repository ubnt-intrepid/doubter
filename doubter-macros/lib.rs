extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate proc_macro_hack;
extern crate doubter_impl as imp;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

proc_macro_item_impl! {
    pub fn doubter_impl(input: &str) -> String {
        let input = input.parse().expect("failed to input to TokenStream");
        inner(input).to_string()
    }
}

#[proc_macro]
pub fn generate_doc_tests(input: TokenStream) -> TokenStream {
    inner(input.into()).into()
}

fn inner(input: TokenStream2) -> TokenStream2 {
    let config = imp::config::Config::from_tokens(input).unwrap_or_else(|e| {
        panic!("failed to parse the input: {}", e);
    });

    let renderer = imp::renderer(config).unwrap_or_else(|e| {
        panic!("error during initializing render context: {}", e);
    });

    let mut tokens = TokenStream2::new();
    renderer.render(&mut tokens).unwrap_or_else(|e| {
        panic!("error during generating doc comments: {}", e);
    });

    tokens
}
