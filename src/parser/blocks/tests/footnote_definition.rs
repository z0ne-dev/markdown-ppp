use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn footnote_definition1() {
    let doc = parse_markdown(MarkdownParserState::default(), "[^foo]: definition").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::FootnoteDefinition(FootnoteDefinition {
                label: "foo".to_owned(),
                blocks: vec![Block::Paragraph(vec![Inline::Text(
                    "definition".to_owned()
                )])]
            })]
        }
    );
}

#[test]
fn footnote_definition2() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "[^foo]: line1
    line2
",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::FootnoteDefinition(FootnoteDefinition {
                label: "foo".to_owned(),
                blocks: vec![Block::Paragraph(vec![Inline::Text(
                    "line1\nline2".to_owned()
                ),])]
            })]
        }
    );
}
