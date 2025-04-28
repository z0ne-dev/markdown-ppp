use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserConfig, MarkdownParserState};

#[test]
fn heading_v1() {
    let doc = parse_markdown(MarkdownParserState::default(), "## a").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Heading(Heading {
                kind: HeadingKind::Atx(2),
                content: vec![Inline::Text("a".to_owned())]
            })]
        }
    );

    let doc = parse_markdown(MarkdownParserState::default(), "##a").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Text("##a".to_string())])]
        }
    );

    let config = MarkdownParserConfig::default().with_allow_no_space_in_headings();
    let doc = parse_markdown(MarkdownParserState::with_config(config), "##a").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Heading(Heading {
                kind: HeadingKind::Atx(2),
                content: vec![Inline::Text("a".to_owned())]
            })]
        }
    );
}

#[test]
fn heading_v2() {
    let doc = parse_markdown(MarkdownParserState::default(), "a\n==").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Heading(Heading {
                kind: HeadingKind::Setext(SetextHeading::Level1),
                content: vec![Inline::Text("a".to_owned())]
            })]
        }
    );

    let doc = parse_markdown(MarkdownParserState::default(), "a\n--").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Heading(Heading {
                kind: HeadingKind::Setext(SetextHeading::Level2),
                content: vec![Inline::Text("a".to_owned())]
            })]
        }
    );
}
