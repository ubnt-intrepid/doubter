#[macro_use]
extern crate failure;
extern crate glob;
#[macro_use]
extern crate proc_macro_hack;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

mod config;
mod render;
mod tree;

use config::Config;
use render::Context;

proc_macro_item_impl! {
    pub fn doubter_impl(input: &str) -> String {
        let config: Config = input.parse().unwrap_or_else(|e| {
            panic!("failed to parse the input: {}", e);
        });

        let mut ctx = Context::init(&config).unwrap_or_else(|e| {
            panic!("failed to initialize the render context: {}", e);
        });

        let output = ctx.run().unwrap_or_else(|e| {
            panic!("error during generating doc comments: {}", e);
        });

        output.to_string()
    }
}
