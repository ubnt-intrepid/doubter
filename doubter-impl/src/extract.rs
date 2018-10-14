use bytecount;
use pulldown_cmark as cmark;
use std::borrow::Cow;

#[derive(Debug, PartialEq)]
pub struct CodeBlock<'a> {
    pub line: usize,
    pub info: Cow<'a, str>,
    pub content: Vec<Cow<'a, str>>,
}

pub fn extract_code_blocks(input: &str) -> Vec<CodeBlock> {
    let mut parser = cmark::Parser::new(input);

    let mut blocks = vec![];
    let mut block = None;

    while let Some(event) = parser.next() {
        match event {
            cmark::Event::Start(cmark::Tag::CodeBlock(info)) => {
                assert!(block.is_none());

                let offset = parser.get_offset();
                let line = bytecount::count(&input.as_bytes()[..offset], b'\n');

                block = Some(CodeBlock {
                    line,
                    info,
                    content: vec![],
                });
            }
            cmark::Event::Text(line) => {
                if let Some(ref mut block) = block {
                    block.content.push(line);
                }
            }
            cmark::Event::End(cmark::Tag::CodeBlock(info)) => {
                let block = block.take().expect("unexpected condition");
                assert_eq!(block.info, info);
                blocks.push(block);
            }
            _ => {}
        }
    }

    blocks
}

#[cfg(test)]
mod tests {
    use super::{extract_code_blocks, CodeBlock};

    #[test]
    fn test_no_blocks() {
        const INPUT: &str = "\
aaa
bbb
ccc";
        assert_eq!(extract_code_blocks(INPUT), vec![]);
    }

    #[test]
    fn test_single_block() {
        const INPUT: &str = "\
header.

```rust
fn main() {}
```

footer.";
        assert_eq!(
            extract_code_blocks(INPUT),
            vec![CodeBlock {
                line: 3,
                info: "rust".into(),
                content: vec!["fn main() {}\n".into(),],
            }]
        );
    }
}
