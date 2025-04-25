use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn code_block_indented1() {
    let doc = parse_markdown(MarkdownParserState::default(), "     a").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::CodeBlock(CodeBlock {
                kind: CodeBlockKind::Indented,
                literal: " a".to_owned()
            })]
        }
    );
}

#[test]
fn code_block_indented2() {
    let doc = parse_markdown(MarkdownParserState::default(), "     a\n    b").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::CodeBlock(CodeBlock {
                kind: CodeBlockKind::Indented,
                literal: " a\nb".to_owned()
            })]
        }
    );
}

#[test]
fn code_block_fenced1() {
    let doc = parse_markdown(MarkdownParserState::default(), "```\na\n```").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::CodeBlock(CodeBlock {
                kind: CodeBlockKind::Fenced { info: None },
                literal: "a".to_owned()
            })]
        }
    );
}

#[test]
fn code_block_fenced2() {
    let doc = parse_markdown(MarkdownParserState::default(), "  `````\na\n`````````").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::CodeBlock(CodeBlock {
                kind: CodeBlockKind::Fenced { info: None },
                literal: "a".to_owned()
            })]
        }
    );
}

#[test]
fn code_block_fenced3() {
    let doc = parse_markdown(MarkdownParserState::default(), "  ```\n    a\n      b\n```").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::CodeBlock(CodeBlock {
                kind: CodeBlockKind::Fenced { info: None },
                literal: "  a\n    b".to_owned()
            })]
        }
    );
}

#[test]
fn code_block_fenced4() {
    let doc = parse_markdown(MarkdownParserState::default(), "  ```rust\na\n```").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::CodeBlock(CodeBlock {
                kind: CodeBlockKind::Fenced {
                    info: Some("rust".to_owned())
                },
                literal: "a".to_owned()
            })]
        }
    );
}
