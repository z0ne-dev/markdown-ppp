use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn thematic_break() {
    let doc = parse_markdown(MarkdownParserState::default(), "---").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::ThematicBreak]
        }
    );
}
