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
                if cfg!(feature = "external-doc") {
                    file.render_with_external_doc()
                } else {
                    file.render()
                }
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
    fn render(&self) -> io::Result<TokenStream> {
        let constant_name = self.constant_name();
        let path = self.resolve_path()?;
        let content = fs::read_to_string(path)?;
        Ok(quote!(
            #[doc = #content]
            pub const #constant_name : () = ();
        ))
    }

    fn render_with_external_doc(&self) -> io::Result<TokenStream> {
        let constant_name = self.constant_name();
        let path = self.resolve_path()?.display().to_string();
        Ok(quote!(
            #[doc(include = #path)]
            pub const #constant_name : () = ();
        ))
    }

    fn constant_name(&self) -> Ident {
        let sanitized = sanitize_file_path(&self.input.value.value());
        Ident::new(&sanitized, Span::call_site())
    }

    fn resolve_path(&self) -> io::Result<PathBuf> {
        self.root_dir.join(self.input.value.value()).canonicalize()
    }
}

fn sanitize_file_path(s: &str) -> String {
    s.to_ascii_lowercase()
        .replace(|c: char| !c.is_ascii() || !c.is_alphanumeric(), "_")
        .split('_')
        .fold(String::new(), |mut acc, s| {
            if !s.is_empty() {
                if !acc.is_empty() {
                    acc += "_";
                }
                acc += s;
            }
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::sanitize_file_path;

    #[test]
    fn test_sanitize_file_path() {
        assert_eq!(sanitize_file_path("foo.md"), "foo_md");
        assert_eq!(sanitize_file_path("_foo.md"), "foo_md");
        assert_eq!(sanitize_file_path("../../foo.md"), "foo_md");
        assert_eq!(sanitize_file_path("/path/to/file.md"), "path_to_file_md");
        assert_eq!(sanitize_file_path("with whitespace"), "with_whitespace");
        assert_eq!(sanitize_file_path("with-hyphen"), "with_hyphen");
        assert_eq!(sanitize_file_path("with%non&ascii"), "with_non_ascii");
    }
}
