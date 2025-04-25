use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn emphasis1() {
    let doc = parse_markdown(MarkdownParserState::default(), "*foo bar*").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Emphasis(vec![
                Inline::Text("foo bar".to_string())
            ])])],
        }
    );
}

#[test]
fn emphasis2() {
    let doc = parse_markdown(MarkdownParserState::default(), "* a *").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::List(List {
                kind: ListKind::Bullet(ListBulletKind::Star),
                items: vec![ListItem {
                    task: None,
                    blocks: vec![Block::Paragraph(vec![Inline::Text("a *".to_owned())])]
                }]
            })]
        }
    );
}

#[test]
fn emphasis3() {
    let doc = parse_markdown(MarkdownParserState::default(), "foo ___bar___").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![
                Inline::Text("foo ".to_owned()),
                Inline::Strong(vec![Inline::Emphasis(vec![Inline::Text("bar".to_owned())])])
            ])]
        }
    );
}

#[test]
fn emphasis4() {
    let doc = parse_markdown(MarkdownParserState::default(), "**foo ___bar___ baz**").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Strong(vec![
                Inline::Text("foo ".to_owned()),
                Inline::Strong(vec![Inline::Emphasis(vec![Inline::Text("bar".to_owned())])]),
                Inline::Text(" baz".to_owned())
            ])])]
        }
    );
}
