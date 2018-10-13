use glob;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io;
use std::iter;
use std::path::{Path, PathBuf};

use proc_macro2::{Span, TokenStream};
use quote::TokenStreamExt;
use syn::Ident;

fn io_error(cause: impl Into<Box<StdError + Send + Sync + 'static>>) -> io::Error {
    io::Error::new(io::ErrorKind::Other, cause)
}

#[derive(Debug, Default)]
pub(crate) struct Tree {
    root: HashMap<OsString, Node>,
}

impl Tree {
    pub(crate) fn register_md_files(
        &mut self,
        pattern: impl AsRef<Path>,
        root_dir: impl AsRef<Path>,
    ) -> io::Result<()> {
        let pattern = pattern.as_ref();
        let root_dir = root_dir.as_ref();

        let entries = glob::glob(&root_dir.join(pattern).to_string_lossy()).map_err(io_error)?;

        for entry in entries {
            let abspath = entry.map_err(io_error)?;
            let md_path = normalize_path(&abspath, &root_dir)?;
            self.insert(MarkdownFile { md_path, abspath });
        }

        Ok(())
    }

    fn insert(&mut self, md_file: MarkdownFile) {
        let mut cursor = &mut self.root;

        let file_name = {
            let mut iter = md_file.md_path.iter().peekable();
            loop {
                if let Some(segment) = iter.next() {
                    if iter.peek().is_none() {
                        break Some(segment.to_owned());
                    } else {
                        cursor = match { cursor }
                            .entry(segment.to_owned())
                            .or_insert_with(|| Node::Dir(Default::default()))
                        {
                            Node::Dir(ref mut map) => map,
                            Node::File(..) => unreachable!(),
                        };
                    }
                } else {
                    break None;
                }
            }
        };

        cursor.insert(file_name.unwrap(), Node::File(md_file));
    }

    pub(crate) fn render(&self, tokens: &mut TokenStream) -> io::Result<()> {
        let mut inner = TokenStream::new();
        render_dir(&self.root, &mut inner)?;
        tokens.append_all(quote!(
            pub mod doctests {
                #inner
            }
        ));
        Ok(())
    }
}

#[derive(Debug)]
enum Node {
    Dir(HashMap<OsString, Node>),
    File(MarkdownFile),
}

impl Node {
    fn render(&self, tokens: &mut TokenStream) -> io::Result<()> {
        match self {
            Node::Dir(ref dir) => render_dir(dir, tokens),
            Node::File(ref md_file) => render_file(md_file, tokens),
        }
    }
}

fn render_dir(dir: &HashMap<OsString, Node>, tokens: &mut TokenStream) -> io::Result<()> {
    for (segment, node) in dir {
        let module_name = match segment {
            s if s == ".." => Ident::new("__PARENT__", Span::call_site()),
            segment => Ident::new(&sanitize(segment), Span::call_site()),
        };

        let mut inner = TokenStream::new();
        node.render(&mut inner)?;

        tokens.append_all(quote! {
            pub mod #module_name {
                #inner
            }
        });
    }
    Ok(())
}

fn render_file(md_file: &MarkdownFile, tokens: &mut TokenStream) -> io::Result<()> {
    if cfg!(feature = "external-doc") {
        md_file.render_with_external_doc(tokens)
    } else {
        md_file.render(tokens)
    }
}

#[derive(Debug)]
struct MarkdownFile {
    md_path: PathBuf,
    abspath: PathBuf,
}

impl MarkdownFile {
    fn render(&self, tokens: &mut TokenStream) -> io::Result<()> {
        let content = fs::read_to_string(&self.abspath)?;
        tokens.append_all(quote!(#![doc = #content]));
        Ok(())
    }

    fn render_with_external_doc(&self, tokens: &mut TokenStream) -> io::Result<()> {
        let path = self.abspath.to_string_lossy();
        tokens.append_all(quote!(#![doc(include = #path)]));
        Ok(())
    }
}

fn sanitize(s: impl AsRef<OsStr>) -> String {
    s.as_ref()
        .to_string_lossy()
        .to_ascii_lowercase()
        .replace(|c: char| !c.is_ascii() || !c.is_alphanumeric(), "_")
}

fn normalize_path(path: impl AsRef<Path>, root_dir: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();
    let mut base = root_dir.as_ref();
    let mut num_parents = 0;
    loop {
        match path.strip_prefix(base) {
            Ok(stripped) => {
                if num_parents > 0 {
                    return Ok(iter::repeat(Path::new(".."))
                        .take(num_parents)
                        .chain(Some(stripped))
                        .collect::<PathBuf>());
                } else {
                    return Ok(stripped.to_owned());
                }
            }
            Err(..) => match base.parent() {
                Some(p) => {
                    num_parents += 1;
                    base = p
                }
                None => return Err(io_error("")),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{normalize_path, sanitize};

    #[test]
    fn test_sanitize() {
        assert_eq!(sanitize("foo.md"), "foo_md");
        assert_eq!(sanitize("_foo.md"), "_foo_md");
        assert_eq!(sanitize("with whitespace.md"), "with_whitespace_md");
        assert_eq!(sanitize("with-hyphen.md"), "with_hyphen_md");
        assert_eq!(sanitize("with%non&ascii.md"), "with_non_ascii_md");
    }

    #[test]
    fn test_normalize_path() {
        assert_eq!(
            normalize_path("/path/to/a/b.md", "/path/to")
                .unwrap()
                .to_string_lossy(),
            "a/b.md"
        );
        assert_eq!(
            normalize_path("/path/to/a/b.md", "/path/to/c")
                .unwrap()
                .to_string_lossy(),
            "../a/b.md"
        );
        assert_eq!(
            normalize_path("/path/to/a/b.md", "/foo/bar")
                .unwrap()
                .to_string_lossy(),
            "../../path/to/a/b.md"
        );
    }
}
