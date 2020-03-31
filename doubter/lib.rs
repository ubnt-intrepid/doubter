//! A helper crate for testing Rust code blocks in Markdown.

#![doc(html_root_url = "https://docs.rs/doubter/0.2.0-dev")]

#[doc(inline)]
pub use doubter_impl::public::{generate_doc_tests, Config, Mode};

#[doc(hidden)]
pub use doubter_macros::*;

/// A macro generating an item from the specified Markdown files.
#[macro_export(local_inner_macros)]
macro_rules! generate_doc_tests {
    ($($t:tt)*) => ( generate_doc_tests_impl!{ $($t)* } );
}
