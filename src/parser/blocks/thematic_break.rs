use crate::parser::util::*;
use crate::parser::MarkdownParserState;
use nom::{
    branch::alt,
    character::complete::{char, space0},
    combinator::map,
    multi::{many, many_m_n},
    sequence::{preceded, terminated},
    IResult, Parser,
};

pub(crate) fn thematic_break<'a>(
    _state: crate::Xrc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, ()> {
    move |input: &str| {
        map(
            line_terminated(preceded(
                many_m_n(0, 3, char(' ')),
                terminated(
                    alt((
                        many(3.., char('-')),
                        many(3.., char('_')),
                        many(3.., char('*')),
                    )),
                    space0,
                ),
            )),
            |_: Vec<_>| (),
        )
        .parse(input)
    }
}
