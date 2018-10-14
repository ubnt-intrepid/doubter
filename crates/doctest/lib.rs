#[macro_use]
extern crate doubter;

generate_doc_tests! {
    include = "foo.md",
    include = "doc/**/*.md",
    include = "../docs/**/*.md",
}
