#![feature(external_doc)]

#[macro_use]
extern crate doubter;

doubter! {
    include = "README.md",
    use_external_doc = true,
}
