# `doubter`

A helper crate for testing Rust code blocks in Markdown.

## Usage

```toml
[dependencies]
doubter = { git = "https://github.com/ubnt-intrepid/doubter.git" }

[dev-dependencies]
...
```

```rust
#[macro_use]
extern crate doubter;

doubter! {
    file = "doc/print_foo.md",
    file = "README.md",
}
```

The macro `doctest!()` cannot be called twice in the same scope
(See also [`dtolnay/proc-macro-hack#2`](https://github.com/dtolnay/proc-macro-hack/issues/2)).

## Status
WIP

- [ ] validate the input Markdown files
- [ ] add `pattern = "..."`
- [ ] add `root_dir = "..."`
- [ ] add `include = [...]`
