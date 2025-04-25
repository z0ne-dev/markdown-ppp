use crate::ast::Inline;
use crate::parser::MarkdownParserState;
use alloc::rc::Rc;
use nom::{character::complete::char, sequence::preceded, IResult, Parser};

pub(crate) fn image<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Inline> {
    move |input: &'a str| {
        let (input, link) = preceded(
            char('!'),
            crate::parser::inline::inline_link::inline_link(state.clone()),
        )
        .parse(input)?;

        Ok((input, Inline::Image(link)))
    }
}
