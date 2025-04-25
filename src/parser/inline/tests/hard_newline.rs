use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn hard_newline1() {
    let doc = parse_markdown(MarkdownParserState::default(), "line1\\\nline2").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![
                Inline::Text("line1".to_string()),
                Inline::LineBreak,
                Inline::Text("line2".to_string())
            ])],
        }
    );
}

#[test]
fn hard_newline2() {
    let doc = parse_markdown(MarkdownParserState::default(), "line1  \nline2").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![
                Inline::Text("line1".to_string()),
                Inline::LineBreak,
                Inline::Text("line2".to_string())
            ])],
        }
    );
}
