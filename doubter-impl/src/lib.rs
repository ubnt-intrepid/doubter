mod config;
mod extract;
mod render;
mod tree;
mod util;

// not a public API.
#[doc(hidden)]
pub mod private {
    pub use crate::config::Config;
    pub use crate::render::RenderContext;

    use proc_macro2::TokenStream;

    pub fn parse_config<T>(input: T) -> syn::parse::Result<Config>
    where
        T: Into<TokenStream>,
    {
        syn::parse2(input.into())
    }
}

pub mod public {
    use crate::render::RenderContext;
    use std::io;

    pub use crate::config::{Config, Mode};

    /// Generates a code from the given configuration.
    ///
    /// This function is typically used from the inside of `build.rs`,
    /// in order to avoid constraints on macro calls.
    pub fn generate_doc_tests<W>(config: Config, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        RenderContext::init(config)?.write(writer)
    }
}
