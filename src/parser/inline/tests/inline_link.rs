use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn inline_link1() {
    let doc = parse_markdown(MarkdownParserState::default(), "[foo](/url \"title\")").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Link(Link {
                destination: "/url".to_owned(),
                title: Some("title".to_owned()),
                children: vec![Inline::Text("foo".to_owned())]
            })])]
        }
    );
}

#[test]
fn inline_link2() {
    let doc = parse_markdown(MarkdownParserState::default(), "[foo](train.jpg)").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Link(Link {
                destination: "train.jpg".to_owned(),
                title: None,
                children: vec![Inline::Text("foo".to_owned())]
            })])]
        }
    );
}

#[test]
fn inline_link3() {
    let doc = parse_markdown(MarkdownParserState::default(), "[foo](<url>)").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Link(Link {
                destination: "url".to_owned(),
                title: None,
                children: vec![Inline::Text("foo".to_owned())]
            })])]
        }
    );
}
