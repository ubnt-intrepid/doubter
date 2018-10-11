use pulldown_cmark::{Event, Parser, Tag};
use std::mem;

#[derive(Debug, PartialEq)]
pub struct CodeBlock {
    pub line_number: usize,
    pub block_info: String,
    pub content: String,
}

pub fn extract_code_blocks(input: &str) -> Vec<CodeBlock> {
    let mut parser = Parser::new(input).enumerate();

    let mut block_info = None;
    let mut in_codeblock = false;
    let mut content = String::new();
    let mut started_line_num = 0;

    let mut blocks = vec![];
    while let Some((line_num, event)) = parser.next() {
        match event {
            Event::Start(Tag::CodeBlock(info)) => {
                assert!(block_info.is_none());
                assert!(!in_codeblock);
                assert!(content.is_empty());

                block_info = Some(info.into_owned());
                in_codeblock = true;
                started_line_num = line_num;
            }
            Event::End(Tag::CodeBlock(info)) => {
                assert_eq!(block_info.as_ref().map(|s| s.as_str()), Some(&*info));
                assert!(in_codeblock);
                in_codeblock = false;
                blocks.push(CodeBlock {
                    line_number: started_line_num,
                    block_info: block_info.take().unwrap(),
                    content: mem::replace(&mut content, String::new()),
                });
            }
            Event::Text(ref line) if in_codeblock => content.push_str(&**line),
            _ => {}
        }
    }

    blocks
}
