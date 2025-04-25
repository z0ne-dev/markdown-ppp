use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn footnote_reference1() {
    let doc = parse_markdown(MarkdownParserState::default(), "[^label]").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::FootnoteReference(
                "label".to_string()
            )])],
        }
    );
}
