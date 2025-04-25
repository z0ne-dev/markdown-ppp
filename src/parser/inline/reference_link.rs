use crate::ast::{Inline, LinkReference};
use crate::parser::link_util::link_label;
use crate::parser::MarkdownParserState;
use alloc::rc::Rc;
use nom::{branch::alt, bytes::complete::tag, sequence::terminated, IResult, Parser};

pub(crate) fn reference_link<'a>(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Inline> {
    move |input: &'a str| {
        alt((
            reference_link_full(state.clone()),
            reference_link_collapsed(state.clone()),
            reference_link_shortcut(state.clone()),
        ))
        .parse(input)
    }
}

pub(crate) fn reference_link_full<'a>(
    _state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Inline> {
    move |input: &'a str| {
        let (input, (text, label)) = (link_label, link_label).parse(input)?;
        let link_reference = LinkReference { label, text };
        Ok((input, Inline::LinkReference(link_reference)))
    }
}

pub(crate) fn reference_link_collapsed<'a>(
    _state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Inline> {
    move |input: &'a str| {
        let (input, text) = terminated(link_label, tag("[]")).parse(input)?;
        let link_reference = LinkReference {
            label: text.clone(),
            text,
        };
        Ok((input, Inline::LinkReference(link_reference)))
    }
}

pub(crate) fn reference_link_shortcut<'a>(
    _state: Rc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Inline> {
    move |input: &'a str| {
        let (input, text) = link_label.parse(input)?;
        let link_reference = LinkReference {
            label: text.clone(),
            text,
        };
        Ok((input, Inline::LinkReference(link_reference)))
    }
}
