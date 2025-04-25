use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn html_entity1() {
    let doc = parse_markdown(MarkdownParserState::default(), "&amp;").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Text("&".to_string())]),]
        }
    );
}

#[test]
fn html_entity2() {
    let doc = parse_markdown(MarkdownParserState::default(), "&#32;").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Text(" ".to_string())]),]
        }
    );
}

#[test]
fn html_entity3() {
    let doc = parse_markdown(MarkdownParserState::default(), "&#x20;").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Text(" ".to_string())]),]
        }
    );
}

#[test]
fn html_entity4() {
    let doc = parse_markdown(MarkdownParserState::default(), "&unknownchar;").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Text(
                "&unknownchar;".to_string()
            )]),]
        }
    );
}
