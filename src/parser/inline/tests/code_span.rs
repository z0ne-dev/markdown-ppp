use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn code_span1() {
    let doc = parse_markdown(MarkdownParserState::default(), "`foo`").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Code("foo".to_string())])],
        }
    );
}

#[test]
fn code_span2() {
    let doc = parse_markdown(MarkdownParserState::default(), "`` foo ` bar ``").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Code(
                "foo ` bar".to_string()
            )])],
        }
    );
}

#[test]
fn code_span3() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "``
foo
bar  
baz
``",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Code(
                "foo bar   baz".to_string()
            )])],
        }
    );
}
