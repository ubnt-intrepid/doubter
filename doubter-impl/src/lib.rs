#[macro_use]
extern crate proc_macro_hack;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

mod generate;
mod parsing;

use generate::Context;
use parsing::Input;

proc_macro_item_impl! {
    pub fn doubter_impl(input: &str) -> String {
        let input: Input = syn::parse_str(input).unwrap();
        let output = Context::new(&input).run().expect("error during generating doc comments");
        output.to_string()
    }
}
