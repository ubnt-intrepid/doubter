use std::env;
use std::fs;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use proc_macro2::{Span, TokenStream};
use syn::Ident;

use parsing::{Input, KeyValue};

#[derive(Debug)]
pub struct Context<'a> {
    input: &'a Input,
    _priv: (),
}

impl<'a> Context<'a> {
    pub fn new(input: &'a Input) -> Context<'a> {
        Context { input, _priv: () }
    }

    fn markdown_files<'c>(&'c self) -> impl Iterator<Item = MarkdownFile<'a>> + 'c {
        self.input.files.iter().map(|input| {
            assert_eq!(input.key, "file");
            MarkdownFile { input }
        })
    }

    fn root_dir(&self) -> PathBuf {
        env::var("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap()
    }

    pub fn run(&mut self) -> TokenStream {
        let root_dir = self.root_dir();

        let items = self
            .markdown_files()
            .map(|file| {
                let ident = file.ident();
                let lines = file
                    .read_lines(&root_dir)
                    .expect("failed to read a markdown file");
                quote!(
                    #(#[doc = #lines])*
                    #[allow(dead_code)]
                    pub const #ident : () = ();
                )
            }).collect::<Vec<_>>();

        quote!(
            #(#items)*
        )
    }
}

#[derive(Debug)]
struct MarkdownFile<'a> {
    input: &'a KeyValue,
}

impl<'a> MarkdownFile<'a> {
    fn ident(&self) -> Ident {
        let escaped_name = self.input.value.value().replace('/', "_").replace('.', "_");
        Ident::new(
            &format!("doubter_doctest_{}", escaped_name),
            Span::call_site(),
        )
    }

    fn read_lines(&self, root_dir: impl AsRef<Path>) -> io::Result<Vec<String>> {
        let doc_path = root_dir.as_ref().join(self.input.value.value());
        let file = fs::OpenOptions::new().read(true).open(doc_path)?;
        BufReader::new(file).lines().collect()
    }
}
