use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn strikethrough1() {
    let doc = parse_markdown(MarkdownParserState::default(), "~~text~~").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Strikethrough(vec![
                Inline::Text("text".to_string())
            ])])],
        }
    );
}

#[test]
fn strikethrough2() {
    let doc = parse_markdown(MarkdownParserState::default(), "~~text~~~").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Strikethrough(vec![
                Inline::Text("text~".to_string())
            ])])],
        }
    );
}

#[test]
fn strikethrough3() {
    let doc = parse_markdown(MarkdownParserState::default(), "~~~text~~~").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![
                Inline::Text("~".to_string()),
                Inline::Strikethrough(vec![Inline::Text("text~".to_string())])
            ])],
        }
    );
}
