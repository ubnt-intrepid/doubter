use glob;
use std::collections::HashMap;
use std::ffi::OsString;
use std::io;
use std::iter;
use std::path::{Path, PathBuf};

use util::io_error;

#[derive(Debug, Default)]
pub struct Tree {
    pub root: Dir,
}

impl Tree {
    pub fn register_md_files<P, R>(&mut self, pattern: P, root_dir: R) -> io::Result<()>
    where
        P: AsRef<Path>,
        R: AsRef<Path>,
    {
        let pattern = pattern.as_ref();
        let root_dir = root_dir.as_ref();

        let entries = glob::glob(&root_dir.join(pattern).to_string_lossy()).map_err(io_error)?;

        for entry in entries {
            let path = entry.map_err(io_error)?;
            let normalized = normalize_path::normalize_path(&path, &root_dir)?;
            self.insert(normalized, MarkdownFile { path });
        }

        Ok(())
    }

    fn insert(&mut self, path: impl AsRef<Path>, file: MarkdownFile) {
        let mut current_dir = &mut self.root;
        let mut iter = path.as_ref().iter().peekable();

        while let Some(segment) = iter.next() {
            if iter.peek().is_none() {
                current_dir.insert_file(segment.to_owned(), file);
                break;
            } else {
                current_dir = { current_dir }.insert_subdir(segment.to_owned());
            }
        }
    }
}

#[derive(Debug)]
pub enum Node {
    Dir(Dir),
    File(MarkdownFile),
}

#[derive(Debug, Default)]
pub struct Dir {
    inner: HashMap<OsString, Node>,
}

impl Dir {
    fn insert_subdir(&mut self, name: OsString) -> &mut Self {
        match { self }.inner.entry(name).or_insert_with(|| {
            Node::Dir(Dir {
                inner: Default::default(),
            })
        }) {
            Node::Dir(ref mut dir) => dir,
            Node::File(..) => unreachable!(),
        }
    }

    fn insert_file(&mut self, name: OsString, file: MarkdownFile) {
        self.inner.insert(name, Node::File(file));
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a OsString, &'a Node)> + 'a {
        self.inner.iter()
    }
}

#[derive(Debug)]
pub struct MarkdownFile {
    pub path: PathBuf,
}

mod normalize_path {
    use super::*;

    pub fn normalize_path<P, R>(path: P, root_dir: R) -> io::Result<PathBuf>
    where
        P: AsRef<Path>,
        R: AsRef<Path>,
    {
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
