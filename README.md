# `doubter`

[![crates.io](https://img.shields.io/crates/v/doubter.svg)](https://crates.io/crates/doubter)
[![Docs.rs](https://docs.rs/doubter)](https://docs.rs/doubter)
[![Build Status](https://travis-ci.org/ubnt-intrepid/doubter.svg?branch=master)](https://travis-ci.org/ubnt-intrepid/doubter)

A helper crate for testing Rust code blocks in Markdown.

# Status
Experimental

# Usage

```rust
#[macro_use]
extern crate doubter;

doubter! {
    file = "doc/print_foo.md",
    file = "README.md",
}
```

The macro `doubter!()` cannot be called twice in the same scope.

(See also [`dtolnay/proc-macro-hack#2`](https://github.com/dtolnay/proc-macro-hack/issues/2)).

# Tasks
- [ ] validate the input Markdown files
  - add support for custom info
- [ ] add `pattern = "..."` or support glob pattern in `file = "..."`
- [ ] add `root_dir = "..."`
- [ ] add `include = [...]` and `exclude = [...]`

# License
[MIT license](LICENSE)
