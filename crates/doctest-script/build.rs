extern crate doubter;

fn main() {
    doubter::generate_doc_tests(&["README.md"]).unwrap();
}
