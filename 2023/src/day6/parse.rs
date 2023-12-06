use std::str::FromStr;

use nom::{
    bytes::complete::tag_no_case,
    character::complete::{self, digit1, line_ending, space0, space1},
    combinator::{map_res, opt},
    multi::{fold_many0, separated_list1},
    sequence::{delimited, preceded},
    IResult,
};

use super::Race;

pub(super) fn parse_races(input: &str) -> IResult<&str, Box<[Race]>> {
    let (input, times) = delimited(
        tag_no_case("time:"),
        delimited(space0, separated_list1(space1, complete::u64), space0),
        line_ending,
    )(input)?;

    let (input, distances) = delimited(
        tag_no_case("distance:"),
        delimited(space0, separated_list1(space1, complete::u64), space0),
        opt(line_ending),
    )(input)?;

    Ok((
        input,
        times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| Race { time, distance })
            .collect(),
    ))
}

/// Parses digits separated by zero or more spaces.
///
/// ```ignore
/// assert_eq!(parse_spaced_digits::<u32>(" 12 345  6"), Ok(("", 123456)))
/// ```
pub(super) fn parse_spaced_digits<F: FromStr>(input: &str) -> IResult<&str, F> {
    map_res(
        fold_many0(
            preceded(space1, digit1),
            String::new,
            |mut acc: String, digits: &str| {
                acc.push_str(digits);
                acc
            },
        ),
        |v| v.parse::<F>(),
    )(input)
}

pub(super) fn parse_kerning_race(input: &str) -> IResult<&str, Race> {
    let (input, time) = delimited(tag_no_case("time:"), parse_spaced_digits, line_ending)(input)?;

    let (input, distance) = delimited(
        tag_no_case("distance:"),
        parse_spaced_digits,
        opt(line_ending),
    )(input)?;

    Ok((input, Race { time, distance }))
}
