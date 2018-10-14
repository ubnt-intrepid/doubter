# `doubter`

[![crates.io](https://img.shields.io/crates/v/doubter.svg)](https://crates.io/crates/doubter)
[![Docs.rs](https://docs.rs/doubter/badge.svg)](https://docs.rs/doubter)
[![Build Status](https://travis-ci.org/ubnt-intrepid/doubter.svg?branch=master)](https://travis-ci.org/ubnt-intrepid/doubter)

A helper crate for testing Rust code blocks in Markdown.

## Status
Experimental (see also [the roadmap issue](https://github.com/ubnt-intrepid/doubter/issues/2))

## Usage

At first, create a new crate for testing Markdown files.
This crate must be separated from all published crates.

Next, add the dependencies for `doubter` to `Cargo.toml`.
If some external crates are used in some code blocks, specify it as `[dev-dependencies]`:

```toml
[dependencies]
doubter = "0.0.6"

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

The macro `doubter!()` takes a comma-separated list of fields.
The following field keys are currently supported:

* `include` - string literal  
  A glob pattern that points to the path to the Markdown file(s) to be tested.
  Required to be a relative path from cargo's manifest directory.
* `mode` - string literal, optional  
  The mode to convert Markdown files to doctest.
  Supported values are as follows:
  - `"raw"` (default) : embeds the Markdown files in Rust source *as it is*.
  - `"extract"` : extracts code blocks and emit as doctest *per blocks*.
* `use_external_doc` - string/bool literal, optional
  Specify whether to use `#[doc(include = "...")]` to embed Markdown files.
  When this filed is enabled, the value of `mode` is forced to `"raw"`.

## Implementation Details

This crate emulates the behavior of unstable feature `#[doc(include = "...")]`
by using the procedural macros, and runs code blocks by embedding the the Markdown files in Rust source code.

For example, the macro call

```rust
doubter! {
    include = "README.md",
}
```

is roughly expanded to the following code:

```rust
pub mod doctests {
    pub mod readme_md {
        // ...
        // The content of README.md converted to doc comment
        // ...
    }
}
```

## Alternatives

* [`skeptic`](https://github.com/budziq/rust-skeptic)
* [`docmatic`](https://github.com/assert-rs/docmatic)

## License
[MIT license](LICENSE)
