use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn table1() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "| foo | bar |
| --- | --- |
| baz | bim |",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Table(Table {
                rows: vec![
                    vec![
                        vec![Inline::Text("foo".to_owned())],
                        vec![Inline::Text("bar".to_owned())]
                    ],
                    vec![
                        vec![Inline::Text("baz".to_owned())],
                        vec![Inline::Text("bim".to_owned())]
                    ]
                ],
                alignments: vec![Alignment::None, Alignment::None]
            })]
        }
    );
}

#[test]
fn table2() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "| foo | bar |
| :-- | --: |
| baz | bim |",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Table(Table {
                rows: vec![
                    vec![
                        vec![Inline::Text("foo".to_owned())],
                        vec![Inline::Text("bar".to_owned())]
                    ],
                    vec![
                        vec![Inline::Text("baz".to_owned())],
                        vec![Inline::Text("bim".to_owned())]
                    ]
                ],
                alignments: vec![Alignment::Left, Alignment::Right]
            })]
        }
    );
}

#[test]
fn table3() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "| foo | bar |
| --- | --- |
| baz | b\\|im |",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Table(Table {
                rows: vec![
                    vec![
                        vec![Inline::Text("foo".to_owned())],
                        vec![Inline::Text("bar".to_owned())]
                    ],
                    vec![
                        vec![Inline::Text("baz".to_owned())],
                        vec![Inline::Text("b|im".to_owned())]
                    ]
                ],
                alignments: vec![Alignment::None, Alignment::None]
            })]
        }
    );
}

#[test]
fn table4() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "| abc | def |
| --- | --- |
| bar |
| bar | baz | boo |",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Table(Table {
                rows: vec![
                    vec![
                        vec![Inline::Text("abc".to_owned())],
                        vec![Inline::Text("def".to_owned())]
                    ],
                    vec![
                        vec![Inline::Text("bar".to_owned())],
                        vec![Inline::Text("".to_owned())],
                    ],
                    vec![
                        vec![Inline::Text("bar".to_owned())],
                        vec![Inline::Text("baz".to_owned())],
                    ]
                ],
                alignments: vec![Alignment::None, Alignment::None]
            })]
        }
    );
}
