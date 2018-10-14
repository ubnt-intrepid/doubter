extern crate bytecount;
extern crate glob;
extern crate proc_macro;
extern crate proc_macro2;
extern crate pulldown_cmark;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

pub mod config;
pub mod extract;
pub mod render;
pub mod tree;

use config::Config;
use render::RenderContext;
use std::fs;
use std::io;
use std::path::Path;

pub fn renderer(config: Config) -> io::Result<RenderContext> {
    RenderContext::init(config)
}

pub fn generate_doc_tests(config: Config, out: impl AsRef<Path>) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(out)?;
    RenderContext::init(config)?.write(&mut file)
}
