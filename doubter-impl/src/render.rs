use proc_macro2::TokenStream;
use std::env;
use std::io;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use proc_macro2::Span;
use quote::TokenStreamExt;
use syn::Ident;

use config::{Config, Mode};
use extract::extract_code_blocks;
use tree::{Dir, MarkdownFile, Node, Tree};
use util::{io_error, read_to_string};

#[derive(Debug)]
pub struct RenderContext {
    config: Config,
    root_dir: PathBuf,
    tree: Tree,
}

impl RenderContext {
    pub fn init(config: Config) -> io::Result<RenderContext> {
        let root_dir = env::var_os("CARGO_MANIFEST_DIR")
            .map(PathBuf::from)
            .ok_or_else(|| io_error("the environment variable `CARGO_MANIFEST_DIR` is not set"))?;

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

    fn mode(&self) -> Mode {
        if self.config.use_external_doc {
            Mode::Raw
        } else {
            self.config.mode.unwrap_or_else(|| Mode::Raw)
        }
    }

    pub fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        let mut tokens = TokenStream::new();
        self.render(&mut tokens)?;

        let mut writer = BufWriter::new(writer);
        writer.write_all(tokens.to_string().as_bytes())
    }

    pub fn render(&self, tokens: &mut TokenStream) -> io::Result<()> {
        (Renderer {
            context: self,
            tokens,
        })
        .render_tree(&self.tree)
    }
}

#[derive(Debug)]
struct Renderer<'a> {
    context: &'a RenderContext,
    tokens: &'a mut TokenStream,
}

impl<'a> Renderer<'a> {
    fn with_tokens<F>(&self, f: F) -> io::Result<TokenStream>
    where
        F: FnOnce(&mut Renderer) -> io::Result<()>,
    {
        let mut tokens = TokenStream::new();
        f(&mut Renderer {
            tokens: &mut tokens,
            context: &*self.context,
        })?;
        Ok(tokens)
    }

    fn render_tree(&mut self, tree: &Tree) -> io::Result<()> {
        let inner = self.with_tokens(|r| r.render_dir(&tree.root))?;
        self.tokens.append_all(quote!(
            pub mod doctests {
                #inner
            }
        ));

        Ok(())
    }

    fn render_node(&mut self, node: &Node) -> io::Result<()> {
        match *node {
            Node::Dir(ref dir) => self.render_dir(dir),
            Node::File(ref file) => self.render_file(file),
        }
    }

    fn render_dir(&mut self, dir: &Dir) -> io::Result<()> {
        for (segment, node) in dir.iter() {
            let module_name = match segment {
                s if s == ".." => Ident::new("__PARENT__", Span::call_site()),
                segment => Ident::new(&sanitize::sanitize(segment), Span::call_site()),
            };

            let inner = self.with_tokens(|r| r.render_node(node))?;
            self.tokens.append_all(quote! {
                pub mod #module_name {
                    #inner
                }
            });
        }
        Ok(())
    }

    fn render_file(&mut self, file: &MarkdownFile) -> io::Result<()> {
        match self.context.mode() {
            Mode::Raw => {
                if self.context.config.use_external_doc {
                    let path = file.path.to_string_lossy();
                    self.tokens.append_all(quote!(#![doc(include = #path)]));
                } else {
                    let content = read_to_string(&file.path)?;
                    self.tokens.append_all(quote!(#![doc = #content]));
                }
            }
            Mode::Extract => {
                let content = read_to_string(&file.path)?;
                let blocks = extract_code_blocks(&content);

                for block in blocks {
                    let header = format!("```{}", block.info);
                    let content = &block.content;
                    let const_name = Ident::new(&format!("line_{}", block.line), Span::call_site());
                    self.tokens.append_all(quote! {
                        #[doc = #header]
                        #(#[doc = #content])*
                        #[doc = "```"]
                        #[allow(non_upper_case_globals)]
                        pub const #const_name: () = ();
                    });
                }
            }
        }
        Ok(())
    }
}

mod sanitize {
    #[allow(unused_imports, deprecated)]
    use std::ascii::AsciiExt;
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
