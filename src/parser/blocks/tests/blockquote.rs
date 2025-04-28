use crate::ast::*;
use crate::parser::config::{ElementBehavior, MarkdownParserConfig};
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn blockquote1() {
    let doc = parse_markdown(MarkdownParserState::default(), "> a\n>\n>> b").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::BlockQuote(vec![
                Block::Paragraph(vec![Inline::Text("a".to_owned())]),
                Block::BlockQuote(vec![Block::Paragraph(vec![Inline::Text("b".to_owned())])])
            ])]
        }
    );
}

#[test]
fn blockquote2() {
    let doc = parse_markdown(MarkdownParserState::default(), ">> a\n>>\n>> b").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::BlockQuote(vec![Block::BlockQuote(vec![
                Block::Paragraph(vec![Inline::Text("a".to_owned()),]),
                Block::Paragraph(vec![Inline::Text("b".to_owned())])
            ])])]
        }
    );
}

#[test]
fn blockquote_skip1() {
    let config =
        MarkdownParserConfig::default().with_block_blockquote_behavior(ElementBehavior::Skip);
    let doc = parse_markdown(MarkdownParserState::with_config(config), "> a\n>\n>> b").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Empty]
        }
    );
}

#[test]
fn blockquote_skip2() {
    let config =
        MarkdownParserConfig::default().with_block_blockquote_behavior(ElementBehavior::Skip);
    let doc = parse_markdown(MarkdownParserState::with_config(config), "a\n> a\n>\n>> b").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![
                Block::Paragraph(vec![Inline::Text("a".to_owned())]),
                Block::Empty
            ]
        }
    );
}

#[test]
fn blockquote_ignore1() {
    let config =
        MarkdownParserConfig::default().with_block_blockquote_behavior(ElementBehavior::Ignore);
    let doc = parse_markdown(MarkdownParserState::with_config(config), "> a\n>\n>> b").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Text(
                "> a\n>\n>> b".to_owned()
            )]),]
        }
    );
}

#[test]
fn blockquote_ignore2() {
    let config =
        MarkdownParserConfig::default().with_block_blockquote_behavior(ElementBehavior::Ignore);
    let doc = parse_markdown(MarkdownParserState::with_config(config), "a\n> a\n>\n>> b").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Text(
                "a\n> a\n>\n>> b".to_owned()
            )]),]
        }
    );
}
