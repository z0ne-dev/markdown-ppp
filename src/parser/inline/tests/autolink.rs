use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn autolink1() {
    let doc = parse_markdown(MarkdownParserState::default(), "<http://foo.bar.baz>").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Autolink(
                "http://foo.bar.baz".to_owned()
            )])]
        }
    );
}

#[test]
fn autolink2() {
    let doc = parse_markdown(MarkdownParserState::default(), "<irc://foo.bar:2233/baz>").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Autolink(
                "irc://foo.bar:2233/baz".to_owned()
            )])]
        }
    );
}

#[test]
fn autolink3() {
    let doc = parse_markdown(MarkdownParserState::default(), "<MAILTO:FOO@BAR.BAZ>").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Autolink(
                "MAILTO:FOO@BAR.BAZ".to_owned()
            )])]
        }
    );
}

#[test]
fn autolink4() {
    let doc = parse_markdown(MarkdownParserState::default(), "<http://foo.bar/baz bim>").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Text(
                "<http://foo.bar/baz bim>".to_owned()
            )])]
        }
    );
}

#[test]
fn autolink5() {
    let doc = parse_markdown(MarkdownParserState::default(), "<http://example.com/\\[\\>").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Autolink(
                "http://example.com/\\[\\".to_owned()
            )])]
        }
    );
}

#[test]
fn autolink6() {
    let doc = parse_markdown(MarkdownParserState::default(), "<>").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Text("<>".to_owned())])]
        }
    );
}
