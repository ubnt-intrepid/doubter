use glob::glob;
use std::env;
use std::error::Error as StdError;
use std::fs;
use std::io;
use std::path::PathBuf;

use proc_macro2::{Span, TokenStream};
use syn::Ident;

use parsing::{Field, Input};

fn io_error(cause: impl Into<Box<StdError + Send + Sync + 'static>>) -> io::Error {
    io::Error::new(io::ErrorKind::Other, cause)
}

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

    fn markdown_files(&self) -> io::Result<Vec<MarkdownFile>> {
        let root_dir = self.root_dir.as_ref().expect("should be initialized");

        let mut files = vec![];

        for field in &self.input.fields {
            match field {
                Field::File(ref f) => {
                    let md_path = f.value.value().into();
                    let abspath = root_dir.join(&md_path).canonicalize()?;
                    files.push(MarkdownFile { md_path, abspath });
                }
                Field::Include(ref f) => {
                    let pattern = f.value.value();
                    let entries = glob(&pattern).map_err(io_error)?;
                    for entry in entries {
                        let md_path = entry.map_err(io_error)?;
                        let abspath = root_dir.join(&md_path).canonicalize()?;
                        files.push(MarkdownFile { md_path, abspath });
                    }
                }
            }
        }

        Ok(files)
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
            .markdown_files()?
            .into_iter()
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
struct MarkdownFile {
    md_path: PathBuf,
    abspath: PathBuf,
}

impl MarkdownFile {
    fn render(&self) -> io::Result<TokenStream> {
        let constant_name = self.constant_name();
        let content = fs::read_to_string(&self.abspath)?;
        Ok(quote!(
            #[doc = #content]
            pub const #constant_name : () = ();
        ))
    }

    fn render_with_external_doc(&self) -> io::Result<TokenStream> {
        let constant_name = self.constant_name();
        let path = self.abspath.to_string_lossy();
        Ok(quote!(
            #[doc(include = #path)]
            pub const #constant_name : () = ();
        ))
    }

    fn constant_name(&self) -> Ident {
        let sanitized = sanitize_file_path(&self.md_path.to_string_lossy());
        Ident::new(&sanitized, Span::call_site())
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
