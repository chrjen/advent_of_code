use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{self},
    combinator::{map_parser, opt, peek, value},
    multi::fold_many0,
    sequence::terminated,
    IResult,
};

/// Takes a Vec<T> and pushes item onto the end if and only if item is Some(T).
fn push_some<T>(mut vec: Vec<T>, item: Option<T>) -> Vec<T> {
    if let Some(item) = item {
        vec.push(item);
    }
    vec
}

/// Parses a single digit including the spelt out form of each digit.
pub(super) fn parse_digit(input: &str) -> IResult<&str, i32> {
    alt((
        map_parser(take(1usize), complete::i32),
        value(1, tag("one")),
        value(2, tag("two")),
        value(3, tag("three")),
        value(4, tag("four")),
        value(5, tag("five")),
        value(6, tag("six")),
        value(7, tag("seven")),
        value(8, tag("eight")),
        value(9, tag("nine")),
    ))(input)
}

pub(super) fn parse_line(input: &str) -> Result<Vec<i32>, nom::Err<nom::error::Error<&str>>> {
    let parser = terminated(opt(peek(parse_digit)), complete::anychar);
    fold_many0(parser, Vec::new, push_some)(input).map(|(_, v)| v)
}
