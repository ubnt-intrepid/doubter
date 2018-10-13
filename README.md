# `doubter`

[![crates.io](https://img.shields.io/crates/v/doubter.svg)](https://crates.io/crates/doubter)
[![Docs.rs](https://docs.rs/doubter/badge.svg)](https://docs.rs/doubter)
[![Build Status](https://travis-ci.org/ubnt-intrepid/doubter.svg?branch=master)](https://travis-ci.org/ubnt-intrepid/doubter)

A helper crate for testing Rust code blocks in Markdown.

This crate works similarly to `#![doc(include = "...")]`, but it does not require
the nightly Rust toolchain.

## Usage

At first, create a new crate for testing Markdown files.
This crate must be separated from all published crates.

Next, add the dependencies for `doubter` to `Cargo.toml`.
If some external crates are used in some code blocks, specify it as `[dev-dependencies]`:

```toml
[dependencies]
doubter = "0.0.5"

[dev-dependencies]
# put here additional dependencies used in code blocks.
rand = "*"
# ...
```

Finally modify `src/lib.rs` to specify the path to target Markdown files.
All paths specified here must be the relative path from Cargo's manifest directory.

```rust
#[macro_use]
extern crate doubter;

doubter! {
    include = "README.md",
    include = "docs/**/*.md",
}
```

## Status
Experimental (see also [the roadmap issue](https://github.com/ubnt-intrepid/doubter/issues/2))

## Alternatives

* [`skeptic`](https://github.com/budziq/rust-skeptic)
* [`docmatic`](https://github.com/assert-rs/docmatic)

## License
[MIT license](LICENSE)
