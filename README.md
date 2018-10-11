# `doubter`

[![crates.io](https://img.shields.io/crates/v/doubter.svg)](https://crates.io/crates/doubter)
[![Docs.rs](https://docs.rs/doubter/badge.svg)](https://docs.rs/doubter)
[![Build Status](https://travis-ci.org/ubnt-intrepid/doubter.svg?branch=master)](https://travis-ci.org/ubnt-intrepid/doubter)

A helper crate for testing Rust code blocks in Markdown.

This crate works similarly to `#![doc(include = "...")]`, but it does not require
the nightly Rust toolchain.

# Status
Experimental (see also [the roadmap issue](https://github.com/ubnt-intrepid/doubter/issues/2))

# Usage

```rust
#[macro_use]
extern crate doubter;

doubter! {
    file = "doc/print_foo.md",
    file = "README.md",
}
```

Note that the macro `doubter!()` cannot be called twice in the same scope.

# License
[MIT license](LICENSE)
