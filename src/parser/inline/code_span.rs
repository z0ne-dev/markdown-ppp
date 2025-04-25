use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, line_ending, space0},
    combinator::{not, peek, recognize, value},
    multi::many1,
    sequence::preceded,
    IResult, Parser,
};

pub(crate) fn code_span(input: &str) -> IResult<&str, String> {
    let (input, open_ticks) = backtick_string(input)?;
    let tick_count = open_ticks.len();
    let closing_tag_value = "`".repeat(tick_count);

    let not_a_closing_tag = (tag(closing_tag_value.as_str()), char('`'));
    let closing_tag = preceded(
        peek(not(not_a_closing_tag)),
        tag(closing_tag_value.as_str()),
    );
    let empty_line = (line_ending, space0, line_ending);
    let content_parser = preceded(
        peek(not(alt((value((), closing_tag), value((), empty_line))))),
        anychar,
    );

    let (input, content) = recognize(many1(content_parser)).parse(input)?;
    let (input, _) = tag(closing_tag_value.as_str()).parse(input)?;

    let mut content = content.replace("\r\n", " ").replace("\n", " ");
    if content.starts_with(' ') && content.ends_with(' ') && content.trim() != "" {
        content = content[1..content.len() - 1].to_string();
    }

    Ok((input, content))
}

fn backtick_string(input: &str) -> IResult<&str, &str> {
    recognize(many1(char('`'))).parse(input)
}
