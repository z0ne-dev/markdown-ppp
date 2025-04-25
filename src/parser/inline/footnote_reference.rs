use crate::ast::Inline;
use alloc::borrow::ToOwned;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, char},
    combinator::map,
    sequence::delimited,
    IResult, Parser,
};

pub(crate) fn footnote_reference<'a>(input: &'a str) -> IResult<&'a str, Inline> {
    map(
        delimited(tag("[^"), alphanumeric1, char(']')),
        |s: &'a str| Inline::FootnoteReference(s.to_owned()),
    )
    .parse(input)
}
