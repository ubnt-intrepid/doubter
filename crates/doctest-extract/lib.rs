#[macro_use]
extern crate doubter;

generate_doc_tests! {
    mode = "extract",
    include = "foo.md",
}
