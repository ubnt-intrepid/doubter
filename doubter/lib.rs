//! A helper crate for testing Rust code blocks in Markdown.

#![doc(html_root_url = "https://docs.rs/doubter/0.1.0")]

extern crate doubter_impl as imp;
#[allow(unused_imports)]
#[macro_use]
extern crate doubter_macros as macros;

#[cfg(feature = "hack")]
#[macro_use]
extern crate proc_macro_hack;

#[doc(inline)]
pub use imp::public::{generate_doc_tests, Config, Mode};
#[doc(hidden)]
pub use macros::*;

#[cfg(feature = "hack")]
proc_macro_item_decl! {
    /// A macro generating an item from the specified Markdown files.
    generate_doc_tests! => generate_doc_tests_impl
}

/// A macro generating an item from the specified Markdown files.
#[cfg(not(feature = "hack"))]
#[macro_export(local_inner_macros)]
macro_rules! generate_doc_tests {
    ($($t:tt)*) => ( generate_doc_tests_impl!{ $($t)* } );
}
