extern crate doubter;

use std::env;
use std::fs;
use std::path::PathBuf;

use doubter::{Config, Mode};

fn main() {
    let config = Config {
        includes: vec!["foo.md".into()],
        mode: Some(Mode::Extract),
        use_external_doc: false,
    };

    let out_path = env::var_os("OUT_DIR")
        .map(PathBuf::from)
        .unwrap()
        .join("doubter-tests.rs");

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(out_path)
        .unwrap();

    doubter::generate_doc_tests(config, &mut file).unwrap();
}
