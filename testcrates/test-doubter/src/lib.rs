#[macro_use]
extern crate doubter;

doubter! {
    file = "doc/the_answer.md",
    file = "doc/with-hyphen.md",
    file = "doc/with.dots.md",
    file = "doc/with whitespace.md",
    file = "doc/with%nonascii&chars.md",
    file = "README.md",
}
