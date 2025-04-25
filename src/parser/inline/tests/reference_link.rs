use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn reference_link1() {
    let doc = parse_markdown(MarkdownParserState::default(), "[text][label]").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::LinkReference(
                LinkReference {
                    label: "label".to_owned(),
                    text: "text".to_owned()
                }
            )])],
        }
    );
}

#[test]
fn reference_link2() {
    let doc = parse_markdown(MarkdownParserState::default(), "[text][]").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::LinkReference(
                LinkReference {
                    label: "text".to_owned(),
                    text: "text".to_owned()
                }
            )])],
        }
    );
}

#[test]
fn reference_link3() {
    let doc = parse_markdown(MarkdownParserState::default(), "[text]").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::LinkReference(
                LinkReference {
                    label: "text".to_owned(),
                    text: "text".to_owned()
                }
            )])],
        }
    );
}
