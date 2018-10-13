use std::env;
use std::io;
use std::path::PathBuf;

use proc_macro2::TokenStream;

use parsing::{Field, Input};
use tree::Tree;

#[derive(Debug)]
pub struct Context<'a> {
    input: &'a Input,
    root_dir: Option<PathBuf>,
    _priv: (),
}

impl<'a> Context<'a> {
    pub(crate) fn new(input: &'a Input) -> Context<'a> {
        Context {
            input,
            root_dir: None,
            _priv: (),
        }
    }

    fn collect_markdown_files(&self) -> io::Result<Tree> {
        let root_dir = self.root_dir.as_ref().expect("should be initialized");

        let mut tree = Tree::default();

        for field in &self.input.fields {
            match field {
                Field::Include(ref f) => {
                    let pattern = f.value.value();
                    tree.register_md_files(pattern, root_dir)?;
                }
            }
        }

        Ok(tree)
    }

    fn init_root_dir(&mut self) -> io::Result<()> {
        if self.root_dir.is_some() {
            return Ok(());
        }

        let manifest_dir = env::var("CARGO_MANIFEST_DIR")
            .map(PathBuf::from)
            .map_err(|kind| io::Error::new(io::ErrorKind::Other, kind))?;

        self.root_dir = Some(manifest_dir);
        Ok(())
    }

    pub fn run(&mut self) -> io::Result<TokenStream> {
        self.init_root_dir()?;

        let file_tree = self.collect_markdown_files()?;

        let mut tokens = TokenStream::new();
        file_tree.render(&mut tokens)?;
        Ok(tokens)
    }
}
