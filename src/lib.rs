//! A helper crate for testing Rust code blocks in Markdown.

#![doc(html_root_url = "https://docs.rs/doubter/0.0.2")]

#[macro_use]
extern crate proc_macro_hack;

#[allow(unused_imports)]
#[macro_use]
extern crate doubter_impl;
#[doc(hidden)]
pub use doubter_impl::*;

proc_macro_item_decl! {
    doubter! => doubter_impl
}
