mod blocks;
mod config;
mod inline;
mod link_util;
mod util;

use crate::ast::Document;
use crate::parser::config::MarkdownParserConfig;
use alloc::rc::Rc;
use nom::{
    branch::alt,
    character::complete::{line_ending, space1},
    combinator::eof,
    error::make_error,
    multi::many0,
    sequence::terminated,
    Parser,
};

pub struct MarkdownParserState {
    pub config: Rc<MarkdownParserConfig>,
}

impl MarkdownParserState {
    pub fn with_config(config: MarkdownParserConfig) -> Self {
        Self {
            config: Rc::new(config),
        }
    }
}

impl Default for MarkdownParserState {
    fn default() -> Self {
        Self::with_config(MarkdownParserConfig::default())
    }
}

/// Parse the given Markdown string into an AST.
pub fn parse_markdown(
    state: MarkdownParserState,
    input: &str,
) -> Result<Document, nom::Err<nom::error::Error<&str>>> {
    let empty_lines = many0(alt((space1, line_ending)));
    let mut parser = terminated(
        many0(crate::parser::blocks::block(Rc::new(state))),
        terminated(empty_lines, eof),
    );
    match parser.parse(input) {
        Ok((remaining, blocks)) => {
            if remaining.is_empty() {
                Ok(Document { blocks })
            } else {
                Err(nom::Err::Error(make_error(
                    remaining,
                    nom::error::ErrorKind::Eof,
                )))
            }
        }
        Err(err) => Err(err),
    }
}
