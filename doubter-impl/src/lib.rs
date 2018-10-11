#[macro_use]
extern crate proc_macro_hack;
extern crate proc_macro;
extern crate proc_macro2;
extern crate pulldown_cmark;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

mod generate;
mod markdown;
mod parsing;

use generate::Context;
use parsing::Input;

proc_macro_item_impl! {
    pub fn doubter_impl(input: &str) -> String {
        let input: Input = syn::parse_str(input).unwrap();
        Context::new(&input).run().to_string()
    }
}
