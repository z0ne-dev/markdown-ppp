use crate::ast::Link;
use crate::parser::link_util::{link_destination, link_label, link_title};
use crate::parser::MarkdownParserState;
use nom::{
    character::complete::{char, multispace0},
    combinator::opt,
    sequence::{delimited, preceded},
    IResult, Parser,
};

pub(crate) fn inline_link<'a>(
    state: crate::Xrc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Link> {
    move |input: &'a str| {
        let (input, (children, (destination, title))) = (
            link_label(state.clone()),
            delimited(
                char('('),
                (
                    preceded(multispace0, link_destination),
                    opt(preceded(multispace0, link_title)),
                ),
                preceded(multispace0, char(')')),
            ),
        )
            .parse(input)?;

        let link = Link {
            destination,
            title,
            children,
        };

        Ok((input, link))
    }
}
