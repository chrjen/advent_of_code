use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{self},
    combinator::{map_parser, opt, peek, value},
    multi::fold_many0,
    sequence::terminated,
};

/// Takes a Vec<T> and pushes item onto the end if and only if item is Some(T).
fn push_some<T>(mut vec: Vec<T>, item: Option<T>) -> Vec<T> {
    if let Some(item) = item {
        vec.push(item);
    }
    vec
}

/// Parses a 0 or more single digits.
/// E.g. "4", "8", "9"
pub(super) fn parse_digit0(input: &str) -> IResult<&str, Vec<u32>> {
    let parser = terminated(
        opt(peek(map_parser(take(1usize), complete::u32))),
        complete::anychar,
    );
    fold_many0(parser, Vec::new, push_some)(input)
}

/// Parses a single digit including the spelt out form of each digit.
/// E.g. "4", "eight", "9"
pub(super) fn parse_alphanumeric_digit(input: &str) -> IResult<&str, u32> {
    alt((
        map_parser(take(1usize), complete::u32),
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

/// Parses a 0 or more digits including the spelt out form of each digit.
/// E.g. "4", "eight", "9"
pub(super) fn parse_alphanumeric_digit0(input: &str) -> IResult<&str, Vec<u32>> {
    let parser = terminated(opt(peek(parse_alphanumeric_digit)), complete::anychar);
    fold_many0(parser, Vec::new, push_some)(input)
}
