use nom::{
    bytes::complete::tag_no_case,
    character::complete::{self, digit1, line_ending, space0, space1},
    combinator::opt,
    multi::{many0, separated_list1},
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

pub(super) fn parse_kerning_race(input: &str) -> IResult<&str, Race> {
    let (input, time) = delimited(
        tag_no_case("time:"),
        many0(preceded(space1, digit1)),
        line_ending,
    )(input)?;

    let time: u64 = time
        .into_iter()
        .fold(String::new(), |mut acc, s| {
            acc.push_str(s);
            acc
        })
        .parse()
        .unwrap();

    let (input, distance) = delimited(
        tag_no_case("distance:"),
        many0(preceded(space1, digit1)),
        opt(line_ending),
    )(input)?;

    let distance: u64 = distance
        .into_iter()
        .fold(String::new(), |mut acc, s| {
            acc.push_str(s);
            acc
        })
        .parse()
        .unwrap();

    Ok((input, Race { time, distance }))
}
