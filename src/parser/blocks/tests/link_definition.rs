use crate::ast::*;
use crate::parser::config::{ElementBehavior, MarkdownParserConfig};
use crate::parser::{parse_markdown, MarkdownParserState};
use alloc::rc::Rc;
use core::cell::RefCell;

#[test]
fn link_definition1() {
    let doc = parse_markdown(MarkdownParserState::default(), "[foo]: /url \"title\"").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Definition(LinkDefinition {
                label: "foo".to_owned(),
                destination: "/url".to_owned(),
                title: Some("title".to_owned())
            })]
        }
    );
}

#[test]
fn link_definition2() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "   [foo]: 
      /url  
           'the title'
",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Definition(LinkDefinition {
                label: "foo".to_owned(),
                destination: "/url".to_owned(),
                title: Some("the title".to_owned())
            })]
        }
    );
}

#[test]
fn link_definition3() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "[Foo*bar\\]]:my_(url) 'title (with parens)'",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Definition(LinkDefinition {
                label: "Foo*bar]".to_owned(),
                destination: "my_(url)".to_owned(),
                title: Some("title (with parens)".to_owned())
            })]
        }
    );
}

#[test]
fn link_definition4() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "[Foo bar]:
<my url>
'title'",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Definition(LinkDefinition {
                label: "Foo bar".to_owned(),
                destination: "my url".to_owned(),
                title: Some("title".to_owned())
            })]
        }
    );
}

#[test]
fn link_definition5() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "[foo]: /url '
title
line1
line2
'",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Definition(LinkDefinition {
                label: "foo".to_owned(),
                destination: "/url".to_owned(),
                title: Some("\ntitle\nline1\nline2\n".to_owned())
            })]
        }
    );
}

#[test]
fn link_definition6() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "[foo]:
/url",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Definition(LinkDefinition {
                label: "foo".to_owned(),
                destination: "/url".to_owned(),
                title: None
            })]
        }
    );
}

#[test]
fn link_definition7() {
    let doc = parse_markdown(MarkdownParserState::default(), "[foo]: <>").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Definition(LinkDefinition {
                label: "foo".to_owned(),
                destination: "".to_owned(),
                title: None
            })]
        }
    );
}

#[test]
fn link_definition_mapped1() {
    let config = MarkdownParserConfig::default().with_block_link_definition_behavior(
        ElementBehavior::Map(Rc::new(RefCell::new(Box::new(|block| {
            if let Block::Definition(v) = block {
                Block::Definition(LinkDefinition {
                    label: format!("mapped {}", v.label),
                    destination: format!("mapped {}", v.destination),
                    title: v.title.map(|t| format!("mapped {}", t)),
                })
            } else {
                block
            }
        })))),
    );
    let doc = parse_markdown(
        MarkdownParserState::with_config(config),
        "[foo]: /url \"title\"",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Definition(LinkDefinition {
                label: "mapped foo".to_owned(),
                destination: "mapped /url".to_owned(),
                title: Some("mapped title".to_owned())
            })]
        }
    );
}
