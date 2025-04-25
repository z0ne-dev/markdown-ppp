use nom::{
    branch::alt,
    bytes::complete::{take_while, take_while1},
    character::complete::{char, satisfy},
    combinator::{map, recognize},
    sequence::{delimited, pair, terminated},
    IResult, Parser,
};

pub(crate) fn autolink(input: &str) -> IResult<&str, String> {
    delimited(char('<'), alt((uri, email)), char('>')).parse(input)
}

/// uri: scheme ":" [^<>\u0000-\u0020]*
fn uri(input: &str) -> IResult<&str, String> {
    map(
        pair(
            terminated(scheme, char(':')),
            take_while(|c: char| {
                !c.is_ascii_control() && !c.is_ascii_whitespace() && c != '<' && c != '>'
            }),
        ),
        |(scheme_part, rest): (&str, &str)| {
            let mut s = String::from(scheme_part);
            s.push(':');
            s.push_str(rest);
            s
        },
    )
    .parse(input)
}

/// email: simplified form for <user@example.com>
fn email(input: &str) -> IResult<&str, String> {
    map(
        recognize(pair(
            take_while1(|c: char| c.is_ascii_alphanumeric() || "_-.".contains(c)),
            pair(
                char('@'),
                take_while1(|c: char| c.is_ascii_alphanumeric() || ".-".contains(c)),
            ),
        )),
        |v: &str| v.to_string(),
    )
    .parse(input)
}

/// scheme: [a-zA-Z][a-zA-Z0-9+.-]{1,31}
fn scheme(input: &str) -> IResult<&str, &str> {
    recognize(pair(satisfy(is_scheme_start), take_while1(is_scheme_char))).parse(input)
}

fn is_scheme_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '+' | '.' | '-')
}

fn is_scheme_start(c: char) -> bool {
    c.is_ascii_alphabetic()
}
