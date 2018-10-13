#![feature(external_doc)]

#[macro_use]
extern crate doubter;

doubter! {
    mode = "external-doc",
    include = "README.md",
}
