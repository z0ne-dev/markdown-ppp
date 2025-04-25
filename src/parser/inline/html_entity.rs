use crate::parser::MarkdownParserState;
use alloc::rc::Rc;
use alloc::string::ToString;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, hex_digit1, one_of},
    combinator::{map, map_opt, recognize},
    sequence::{delimited, preceded},
    IResult, Parser,
};

pub(crate) fn html_entity(
    state: Rc<MarkdownParserState>,
) -> impl FnMut(&str) -> IResult<&str, String> {
    move |input: &str| alt((html_entity_alpha(state.clone()), html_entity_numeric)).parse(input)
}

fn html_entity_alpha(state: Rc<MarkdownParserState>) -> impl FnMut(&str) -> IResult<&str, String> {
    move |input: &str| {
        map_opt(recognize((char('&'), alpha1, char(';'))), |s: &str| {
            state
                .as_ref()
                .config
                .html_entities_map
                .get(s)
                .map(|entity| entity.characters.to_owned())
        })
        .parse(input)
    }
}

fn html_entity_numeric(input: &str) -> IResult<&str, String> {
    let base16 = map_opt(preceded(one_of("xX"), hex_digit1), |s: &str| {
        u32::from_str_radix(s, 16).ok()
    });
    let base10 = map_opt(digit1, |s: &str| s.parse::<u32>().ok());

    map(
        map_opt(
            delimited(tag("&#"), alt((base10, base16)), char(';')),
            char::from_u32,
        ),
        |c: char| c.to_string(),
    )
    .parse(input)
}
