use crate::ast::*;
use crate::parser::{parse_markdown, MarkdownParserState};

#[test]
fn html_block1() {
    let doc = parse_markdown(MarkdownParserState::default(), "<script>\n</script>").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::HtmlBlock("<script>\n</script>".to_owned())]
        }
    );

    let doc = parse_markdown(
        MarkdownParserState::default(),
        "<script>\n\n<h1>hello</h1></script>",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::HtmlBlock(
                "<script>\n\n<h1>hello</h1></script>".to_owned()
            )]
        }
    );
}

#[test]
fn html_block2() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "<!-- \n\nsome commented\n out code -->",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::HtmlBlock(
                "<!-- \n\nsome commented\n out code -->".to_owned()
            )]
        }
    );
}

#[test]
fn html_block3() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "  <? \n\nsome \n   code ?>  ",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::HtmlBlock("<? \n\nsome \n   code ?>".to_owned())]
        }
    );
}

#[test]
fn html_block4() {
    let doc = parse_markdown(MarkdownParserState::default(), "  <!A some \n\n\n text >  ").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::HtmlBlock("<!A some \n\n\n text >".to_owned())]
        }
    );
}

#[test]
fn html_block5() {
    let doc = parse_markdown(
        MarkdownParserState::default(),
        "  <![CDATA[ ]\n\n[[]]<> ]]> ",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::HtmlBlock("<![CDATA[ ]\n\n[[]]<> ]]>".to_owned())]
        }
    );
}

#[test]
fn html_block6() {
    let doc = parse_markdown(MarkdownParserState::default(), "  <body  \n\n").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::HtmlBlock("<body  \n".to_owned())]
        }
    );

    let doc = parse_markdown(
        MarkdownParserState::default(),
        "  <body a b=c d='e' f=\"g\" >\n</body>\n\n",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::HtmlBlock(
                "<body a b=c d='e' f=\"g\" >\n</body>\n".to_owned()
            )]
        }
    );

    let doc = parse_markdown(MarkdownParserState::default(), "  </body> <p>\n</p>\n\n").unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::HtmlBlock("</body> <p>\n</p>\n".to_owned())]
        }
    );
}

#[test]
fn html_block_skip1() {
    let config = crate::parser::config::MarkdownParserConfig::default()
        .with_block_html_block_behavior(crate::parser::config::ElementBehavior::Skip);
    let doc = parse_markdown(
        MarkdownParserState::with_config(config.clone()),
        "<script>\n</script>",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Empty]
        }
    );

    let doc = parse_markdown(
        MarkdownParserState::with_config(config),
        "<script>\n\n<h1>hello</h1></script>",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Empty]
        }
    );
}

#[test]
fn html_block_ignore1() {
    let config = crate::parser::config::MarkdownParserConfig::default()
        .with_block_html_block_behavior(crate::parser::config::ElementBehavior::Ignore);
    let doc = parse_markdown(
        MarkdownParserState::with_config(config.clone()),
        "<script>\n</script>",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Text(
                "<script>\n</script>".to_owned()
            )])]
        }
    );

    let doc = parse_markdown(
        MarkdownParserState::with_config(config),
        "<script>\n\n<h1>hello</h1></script>",
    )
    .unwrap();
    assert_eq!(
        doc,
        Document {
            blocks: vec![
                Block::Paragraph(vec![Inline::Text("<script>".to_owned())]),
                Block::Paragraph(vec![Inline::Text("<h1>hello</h1></script>".to_owned())])
            ]
        }
    );
}
