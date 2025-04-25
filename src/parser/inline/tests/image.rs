use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn image1() {
    let doc = parse_markdown(MarkdownParserState::default(), "![foo](/url \"title\")").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Image(Link {
                destination: "/url".to_owned(),
                title: Some("title".to_owned()),
                children: vec![Inline::Text("foo".to_owned())]
            })])]
        }
    );
}

#[test]
fn image2() {
    let doc = parse_markdown(MarkdownParserState::default(), "![foo](train.jpg)").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Image(Link {
                destination: "train.jpg".to_owned(),
                title: None,
                children: vec![Inline::Text("foo".to_owned())]
            })])]
        }
    );
}

#[test]
fn image3() {
    let doc = parse_markdown(MarkdownParserState::default(), "![foo](<url>)").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Image(Link {
                destination: "url".to_owned(),
                title: None,
                children: vec![Inline::Text("foo".to_owned())]
            })])]
        }
    );
}
