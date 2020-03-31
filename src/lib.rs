//! A helper crate for testing Rust code blocks in Markdown.

#![doc(html_root_url = "https://docs.rs/doubter/0.2.0-dev")]

mod config;
mod extract;
mod render;
mod tree;
mod util;

use crate::config::Config;
use crate::render::RenderContext;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro]
pub fn generate_doc_tests(input: TokenStream) -> TokenStream {
    let config = parse_config(input).expect("failed to parse input as Config");
    inner(config).into()
}

fn parse_config<T>(input: T) -> syn::parse::Result<Config>
where
    T: Into<TokenStream2>,
{
    syn::parse2(input.into())
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
