use crate::ast::Inline;
use nom::multi::many_m_n;
use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::value,
    sequence::pair,
    IResult, Parser,
};

pub(crate) fn hard_newline(input: &str) -> IResult<&str, Inline> {
    value(
        Inline::LineBreak,
        alt((
            value((), pair(char('\\'), line_ending)),
            value((), pair(many_m_n(2, usize::MAX, char(' ')), line_ending)),
        )),
    )
    .parse(input)
}
