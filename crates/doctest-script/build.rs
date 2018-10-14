extern crate doubter;
use doubter::{Config, Mode};

fn main() {
    let config = Config {
        includes: vec!["README".into()],
        mode: Some(Mode::Extract),
        use_external_doc: false,
    };

    doubter::generate_doc_tests(config).unwrap();
}
