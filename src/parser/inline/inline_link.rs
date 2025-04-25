use crate::ast::Link;
use crate::parser::link_util::{link_destination, link_label, link_title};
use crate::parser::MarkdownParserState;
use nom::{
    character::complete::{char, multispace0},
    combinator::opt,
    sequence::{delimited, preceded},
    IResult, Parser,
};
use std::rc::Rc;

pub(crate) fn inline_link<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Link> {
    move |input: &'a str| {
        let (input, (label, (destination, title))) = (
            link_label,
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

        let (_, children) = crate::parser::inline::inline_many0(state.clone())
            .parse(label.as_str())
            .map_err(|err| err.map_input(|_| input))?;

        let link = Link {
            destination,
            title,
            children,
        };

        Ok((input, link))
    }
}
