use glob::glob;
use std::collections::HashMap;
use std::env;
use std::error::Error as StdError;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::PathBuf;

use proc_macro2::{Span, TokenStream};
use quote::TokenStreamExt;
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

    fn collect_markdown_files(&self) -> io::Result<Tree> {
        let root_dir = self.root_dir.as_ref().expect("should be initialized");

        let mut tree = Tree::default();
        for field in &self.input.fields {
            match field {
                Field::Include(ref f) => {
                    let pattern = f.value.value();
                    let entries =
                        glob(&root_dir.join(pattern).to_string_lossy()).map_err(io_error)?;
                    for entry in entries {
                        let abspath = entry.map_err(io_error)?;
                        let md_path = abspath
                            .strip_prefix(&root_dir)
                            .map_err(io_error)?
                            .to_owned();
                        tree.insert(MarkdownFile { md_path, abspath });
                    }
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

#[derive(Debug, Default)]
struct Tree {
    root: HashMap<OsString, Node>,
}

impl Tree {
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

    fn render(&self, tokens: &mut TokenStream) -> io::Result<()> {
        render_dir(&self.root, tokens)
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
        let mut inner = TokenStream::new();
        node.render(&mut inner)?;
        let module_name = Ident::new(
            &sanitize_file_path(&*segment.to_string_lossy()),
            Span::call_site(),
        );
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
