use failure::Fallible;
use proc_macro2::TokenStream;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

use proc_macro2::Span;
use quote::TokenStreamExt;
use syn::Ident;

use config::Config;
use tree::{Dir, MarkdownFile, Tree, Visitor};

#[derive(Debug)]
pub struct RenderContext {
    config: Config,
    root_dir: PathBuf,
    tree: Tree,
}

impl RenderContext {
    pub(crate) fn init(config: Config) -> Fallible<RenderContext> {
        let root_dir = env::var_os("CARGO_MANIFEST_DIR")
            .map(PathBuf::from)
            .ok_or_else(|| {
                format_err!("the environment variable `CARGO_MANIFEST_DIR` is not set")
            })?;

        let mut tree = Tree::default();
        for pattern in &config.includes {
            tree.register_md_files(pattern, &root_dir)?;
        }

        Ok(RenderContext {
            config,
            root_dir,
            tree,
        })
    }

    pub fn render(&self, tokens: &mut TokenStream) -> io::Result<()> {
        let mut inner = TokenStream::new();
        Renderer { tokens: &mut inner }.visit_dir(&self.tree.root)?;

        tokens.append_all(quote!(
            pub mod doctests {
                #inner
            }
        ));

        Ok(())
    }
}

#[derive(Debug)]
struct Renderer<'a> {
    tokens: &'a mut TokenStream,
}

impl<'a> Visitor for Renderer<'a> {
    type Error = io::Error;

    fn visit_dir(&mut self, dir: &Dir) -> io::Result<()> {
        for (segment, node) in dir.iter() {
            let module_name = match segment {
                s if s == ".." => Ident::new("__PARENT__", Span::call_site()),
                segment => Ident::new(&sanitize::sanitize(segment), Span::call_site()),
            };

            let mut inner = TokenStream::new();
            Renderer { tokens: &mut inner }.visit_node(node)?;

            self.tokens.append_all(quote! {
                pub mod #module_name {
                    #inner
                }
            });
        }
        Ok(())
    }

    fn visit_file(&mut self, file: &MarkdownFile) -> io::Result<()> {
        if cfg!(feature = "external-doc") {
            let path = file.path.to_string_lossy();
            self.tokens.append_all(quote!(#![doc(include = #path)]));
            Ok(())
        } else {
            let content = fs::read_to_string(&file.path)?;
            self.tokens.append_all(quote!(#![doc = #content]));
            Ok(())
        }
    }
}

mod sanitize {
    use std::ffi::OsStr;

    pub fn sanitize<S>(s: S) -> String
    where
        S: AsRef<OsStr>,
    {
        s.as_ref()
            .to_string_lossy()
            .to_ascii_lowercase()
            .replace(|c: char| !c.is_ascii() || !c.is_alphanumeric(), "_")
    }

    #[test]
    fn test_sanitize() {
        assert_eq!(sanitize("foo.md"), "foo_md");
        assert_eq!(sanitize("_foo.md"), "_foo_md");
        assert_eq!(sanitize("with whitespace.md"), "with_whitespace_md");
        assert_eq!(sanitize("with-hyphen.md"), "with_hyphen_md");
        assert_eq!(sanitize("with%non&ascii.md"), "with_non_ascii_md");
    }
}
