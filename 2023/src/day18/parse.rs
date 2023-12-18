use nom::{
    character::complete::{self, line_ending, one_of, space0},
    combinator::{all_consuming, map, map_res},
    multi::separated_list0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser,
};

use super::data::{Colour, DigStep, Dir};

pub(super) fn dir(input: &str) -> IResult<&str, Dir> {
    let (input, c) = one_of("ULRD")(input)?;

    let dir = match c {
        'U' => Dir::Up,
        'L' => Dir::Left,
        'D' => Dir::Down,
        'R' => Dir::Right,
        _ => unreachable!("got character {c}"), // Should never happen due to use of `one_of` above.
    };

    Ok((input, dir))
}

pub(super) fn colour(input: &str) -> IResult<&str, Colour> {
    let (input, colour) = map_res(
        delimited(
            complete::char('('),
            preceded(complete::char('#'), complete::hex_digit1),
            complete::char(')'),
        ),
        |hex_str: &str| u32::from_str_radix(hex_str, 16),
    )(input)?;

    Ok((input, Colour(colour)))
}

pub(super) fn dig_step(input: &str) -> IResult<&str, DigStep> {
    map(
        tuple((
            terminated(dir, space0),
            terminated(complete::u64.map(|v| v as usize), space0),
            terminated(colour, space0),
        )),
        |(dir, dist, colour)| DigStep { dir, dist, colour },
    )(input)
}

pub(super) fn parse_dig_plan(input: &str) -> IResult<&str, Box<[DigStep]>> {
    map(all_consuming(separated_list0(line_ending, dig_step)), |v| {
        v.into_boxed_slice()
    })(input)
}
