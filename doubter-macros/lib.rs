extern crate doubter_impl as imp;
extern crate proc_macro;
extern crate proc_macro2;

#[cfg(feature = "proc-macro-hack")]
#[macro_use]
extern crate proc_macro_hack;

#[cfg(not(feature = "proc-macro-hack"))]
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use imp::private::*;

#[cfg(feature = "proc-macro-hack")]
proc_macro_item_impl! {
    pub fn generate_doc_tests_impl(input: &str) -> String {
        let config = input.parse().expect("failed to parse input as Config");
        inner(config).to_string()
    }
}

#[cfg(not(feature = "proc-macro-hack"))]
#[proc_macro]
pub fn generate_doc_tests_impl(input: TokenStream) -> TokenStream {
    let config = parse_config(input).expect("failed to parse input as Config");
    inner(config).into()
}

fn inner(config: Config) -> TokenStream2 {
    let renderer = RenderContext::init(config).unwrap_or_else(|e| {
        panic!("error during initializing render context: {}", e);
    });

    let mut tokens = TokenStream2::new();
    renderer.render(&mut tokens).unwrap_or_else(|e| {
        panic!("error during generating doc comments: {}", e);
    });

    tokens
}
