# `testcrate`

## Example

```rust
#[macro_use]
extern crate serde;
extern crate serde_json;

#[derive(Debug, Deserialize, PartialEq)]
struct Param {
    name: String,
    age: u32,
}

fn main() {
    let input = r#"{ "name": "Alice", "age": 14 }"#;
    assert_eq!(
        serde_json::from_str(input).ok(),
        Some(Param { name: "Alice".into(), age: 14, })
    );
}
```
