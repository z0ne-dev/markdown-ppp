use crate::ast::{CodeBlock, CodeBlockKind};
use crate::parser::util::*;
use crate::parser::MarkdownParserState;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{not, opt, peek, recognize, value},
    multi::{many0, many1, many_m_n},
    sequence::preceded,
    IResult, Parser,
};

pub(crate) fn code_block<'a>(
    state: crate::Xrc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, CodeBlock> {
    move |input: &'a str| {
        alt((
            code_block_indented(state.clone()),
            code_block_fenced(state.clone()),
        ))
        .parse(input)
    }
}

pub(crate) fn code_block_indented<'a>(
    _state: crate::Xrc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, CodeBlock> {
    move |input: &'a str| {
        let line_parser = preceded(
            alt((value((), many_m_n(4, 4, char(' '))), value((), char('\t')))),
            line_terminated(not_eof_or_eol0),
        );

        let (input, lines) = many1(line_parser).parse(input)?;
        let literal = lines.join("\n");

        let code_block = CodeBlock {
            kind: CodeBlockKind::Indented,
            literal,
        };

        Ok((input, code_block))
    }
}

pub(crate) fn code_block_fenced<'a>(
    _state: crate::Xrc<MarkdownParserState>,
) -> impl FnMut(&'a str) -> IResult<&'a str, CodeBlock> {
    move |input: &'a str| {
        let (input, space_prefix) = many_m_n(0, 3, char(' ')).parse(input)?;
        let prefix_length = space_prefix.len();

        let (input, (fence, info)) = line_terminated((
            recognize(alt((
                many_m_n(3, usize::MAX, char('`')),
                many_m_n(3, usize::MAX, char('~')),
            ))),
            opt(recognize(not_eof_or_eol1)),
        ))
        .parse(input)?;
        let ending_fence = || {
            line_terminated((
                many_m_n(0, 3, char(' ')),
                tag(fence),
                many0(char(fence.chars().next().unwrap())),
            ))
        };

        let (input, lines) = many0(preceded(
            peek(not(ending_fence())),
            preceded(
                many_m_n(0, prefix_length, char(' ')),
                line_terminated(not_eof_or_eol0),
            ),
        ))
        .parse(input)?;
        let (input, _) = ending_fence().parse(input)?;

        let literal = lines.join("\n");
        let code_block = CodeBlock {
            kind: CodeBlockKind::Fenced {
                info: info.map(|v| v.to_owned()),
            },
            literal,
        };

        Ok((input, code_block))
    }
}
