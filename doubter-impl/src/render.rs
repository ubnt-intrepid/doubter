use failure::Fallible;
use proc_macro2::TokenStream;
use std::env;
use std::io;
use std::path::PathBuf;

use config::Config;
use tree::Tree;

#[derive(Debug)]
pub struct Context<'a> {
    config: &'a Config,
    root_dir: PathBuf,
    _priv: (),
}

impl<'a> Context<'a> {
    pub(crate) fn init(config: &'a Config) -> Fallible<Context<'a>> {
        let root_dir = env::var_os("CARGO_MANIFEST_DIR")
            .map(PathBuf::from)
            .ok_or_else(|| {
                format_err!("the environment variable `CARGO_MANIFEST_DIR` is not set")
            })?;

        Ok(Context {
            config,
            root_dir,
            _priv: (),
        })
    }

    fn collect_markdown_files(&self) -> io::Result<Tree> {
        let mut tree = Tree::default();
        for pattern in &self.config.includes {
            tree.register_md_files(pattern, &self.root_dir)?;
        }
        Ok(tree)
    }

    pub fn run(&mut self) -> io::Result<TokenStream> {
        let file_tree = self.collect_markdown_files()?;

        let mut tokens = TokenStream::new();
        file_tree.render(&mut tokens)?;
        Ok(tokens)
    }
}
