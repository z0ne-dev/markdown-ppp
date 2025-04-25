use crate::ast::Block;
use crate::parser::util::*;
use crate::parser::MarkdownParserState;
use nom::{
    character::complete::char,
    multi::{many1, many_m_n, separated_list1},
    sequence::preceded,
    IResult, Parser,
};
use std::rc::Rc;

pub(crate) fn blockquote<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Block>> {
    move |input: &'a str| {
        let (_, prefix) = preceded(
            many_m_n(0, 3, char(' ')),
            separated_list1(many_m_n(0, 4, char(' ')), char('>')),
        )
        .parse(input)?;

        let deepness = prefix.len();

        let expect_prefix = (
            many_m_n(0, 3, char(' ')),
            separated_list_m_n(1, deepness, many_m_n(0, 4, char(' ')), char('>')),
        );

        let (input, lines) =
            many1(preceded(expect_prefix, line_terminated(not_eof_or_eol0))).parse(input)?;
        let inner = lines.join("\n");

        let (_, inner) = many1(crate::parser::blocks::block(state.clone()))
            .parse(&inner)
            .map_err(|err| err.map_input(|_| input))?;

        Ok((input, inner))
    }
}
