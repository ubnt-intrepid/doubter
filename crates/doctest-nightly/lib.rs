#![feature(external_doc)]

#[macro_use]
extern crate doubter;

generate_doc_tests! {
    include = "foo.md",
    use_external_doc = true,
}
