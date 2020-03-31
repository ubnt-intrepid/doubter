# `doubter`

[![crates.io](https://img.shields.io/crates/v/doubter.svg)](https://crates.io/crates/doubter)
[![Docs.rs](https://docs.rs/doubter/badge.svg)](https://docs.rs/doubter)
[![Master Doc](https://img.shields.io/badge/docs-master-blue.svg)](https://ubnt-intrepid.github.io/doubter)
[![Rustc Version](https://img.shields.io/badge/rustc-1.22.1+-lightgray.svg)](https://www.rust-lang.org)

Test Rust code blocks in your Markdown files.

## Overview

This crate executes the code blocks in Markdown files by embedding them into
the Rust source and builds as a crate.
The advantage of this approach is that the dependency resolution are done by `cargo`.
This means that that dependency problems associated with updating the Rust toolchain do not occur.

## Getting Started

`doubter` embeds the target Markdown files into the Rust code as the *public* doc comments.
Therefore, it is necessary to create a new crate for testing code blocks separately from the published crates.
This crate(s) are usually registered in `[workspace.members]`.

### Using Procedural Macros

Add the dependencies for `doubter` to `Cargo.toml`.
If some external crates are required in code blocks, specify them as the members of `[dev-dependencies]`:

```toml
[dependencies]
doubter = "0.1.0"

[dev-dependencies]
rand = "0.5"
```

Then, modify `src/lib.rs` to specify the path to target Markdown files.

```rust
#[macro_use]
extern crate doubter;

generate_doc_tests! {
    include = "README.md",
    include = "docs/**/*.md",
}
```

The macro `generate_doc_tests!(...)` takes a comma-separated list of fields.
The following field keys are currently supported:

* `include` - string  
  A glob pattern that points to the path to the Markdown file(s) to be tested.
  Required to be a relative path from cargo's manifest directory.
* `mode` - string, optional  
  The mode to convert Markdown files to doctest.
  Supported values are as follows:
  - `"raw"` (default) : embeds the Markdown files in Rust source *as it is*.
  - `"extract"` : extracts code blocks and emit as doctest *per blocks*.
* `use_external_doc` - string or boolean, optional  
  Specify whether to use `#[doc(include = "...")]` to embed Markdown files.
  When this filed is enabled, the value of `mode` is forced to `"raw"`.

>
> Currently, the implementation of function style procedural macro is using `proc-macro-hack`.
> The definition of procedural macro via custom Derive has some restrictions and the `use`-style
> import does not work as expected.
> You can switch the implementation to the Macros 1.2 by disabling the feature flag `hack`
> (this feature flag is enabled by default):
>
> ```toml
> [dependencies.doubter]
> version = "0.1.0"
> default-features = false
> ```
>

### Using Custom Build Script (a.k.a `build.rs`)

There are some restrictions on the use of procedural macros
(for example, literals passed to the field cannot be calculated by using another macros).
`doubter` provides a low level API for generating test codes from `build.rs`.

At first, moves the dependency on `doubter` to `[build-dependencies]`:

```diff
-[dependencies]
+[build-dependencies]
doubter = "0.1.0"
```

The code for generating test cases in `build.rs` looks like as follows:

```rust
extern crate doubter;

fn main() {
    let config = doubter::Config {
        includes: vec![...],
        mode: None,
        use_external_doc: false,
    };

    let out_path = std::env::var_os("OUT_DIR")
        .map(std::path::PathBuf::from)
        .unwrap()
        .join("doubter-tests.rs");

    let mut file = std::fs::OpenOptions::new()
        .write(true).create(true).truncate(true)
        .open(out_path)
        .unwrap();

    doubter::generate_doc_tests(config, &mut file).unwrap();
}
```

Finally, includes the generated source into `lib.rs` as follows:

```rust
include!(concat!(env!("OUT_DIR"), "/doubter-tests.rs"));
```

## Examples

See the test crates [inside of `crates/`](https://github.com/ubnt-intrepid/doubter/tree/master/crates).

## Alternatives

* [`doc-comment`](https://github.com/GuillaumeGomez/doc-comment)

## License
`doubter` is released under the [MIT license](LICENSE).
