use doubter_impl as imp;
use imp::private::*;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

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
