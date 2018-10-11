use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use proc_macro2::{Span, TokenStream};
use syn::Ident;

use parsing::{Input, KeyValue};

#[derive(Debug)]
pub struct Context<'a> {
    input: &'a Input,
    root_dir: Option<PathBuf>,
    _priv: (),
}

impl<'a> Context<'a> {
    pub fn new(input: &'a Input) -> Context<'a> {
        Context {
            input,
            root_dir: None,
            _priv: (),
        }
    }

    fn markdown_files<'c>(&'c self) -> impl Iterator<Item = MarkdownFile<'a, 'c>> + 'c {
        let root_dir = self.root_dir.as_ref().expect("should be initialized");
        self.input
            .files
            .iter()
            .filter(|input| input.key == "file")
            .map(move |input| MarkdownFile { input, root_dir })
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

        let items = self
            .markdown_files()
            .map(|file| {
                let constant_name = Ident::new(&file.escaped_file_name(), Span::call_site());
                let content = file.read_content()?;
                Ok(quote!(
                    #[doc = #content]
                    pub const #constant_name : () = ();
                ))
            }).collect::<io::Result<Vec<_>>>()?;

        Ok(quote!(
            #(#items)*
        ))
    }
}

#[derive(Debug)]
struct MarkdownFile<'a, 'c> {
    input: &'a KeyValue,
    root_dir: &'c Path,
}

impl<'a, 'c> MarkdownFile<'a, 'c> {
    fn escaped_file_name(&self) -> String {
        self.input.value.value().replace('/', "_").replace('.', "_")
    }

    fn read_content(&self) -> io::Result<String> {
        let doc_path = self.root_dir.join(self.input.value.value());
        fs::read_to_string(&doc_path)
    }
}
