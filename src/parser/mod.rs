mod blocks;
pub mod config;
mod inline;
mod link_util;
mod util;

use crate::ast::Document;
use crate::parser::config::MarkdownParserConfig;
use nom::{
    branch::alt,
    character::complete::{line_ending, space1},
    combinator::eof,
    multi::many0,
    sequence::terminated,
    Parser,
};

pub struct MarkdownParserState {
    pub config: crate::Xrc<MarkdownParserConfig>,
}

impl MarkdownParserState {
    pub fn with_config(config: MarkdownParserConfig) -> Self {
        Self {
            config: crate::Xrc::new(config),
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
        many0(crate::parser::blocks::block(crate::Xrc::new(state))),
        (empty_lines, eof),
    );
    let (_, blocks) = parser.parse(input)?;

    Ok(Document { blocks })
}
