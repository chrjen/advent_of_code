use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, line_ending, space0},
    combinator::{all_consuming, map, map_res, opt, recognize},
    multi::separated_list0,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

use super::data::{Num, Trajectory};

pub(super) fn spaced_separator(separator: char) -> impl Fn(&str) -> IResult<&str, char> {
    move |input: &str| -> IResult<&str, char> {
        delimited(space0, complete::char(separator), space0)(input)
    }
}

pub(super) fn big_rational_parser(input: &str) -> IResult<&str, Num> {
    map_res(recognize(tuple((opt(tag("-")), digit1))), |v: &str| {
        v.parse::<Num>()
    })(input)
}

pub(super) fn vec3(input: &str) -> IResult<&str, (Num, Num, Num)> {
    tuple((
        terminated(big_rational_parser, spaced_separator(',')),
        terminated(big_rational_parser, spaced_separator(',')),
        big_rational_parser,
    ))(input)
}

pub(super) fn trajectory(input: &str) -> IResult<&str, Trajectory> {
    let (input, ((px, py, pz), (vx, vy, vz))) =
        separated_pair(vec3, spaced_separator('@'), vec3)(input)?;

    Ok((
        input,
        Trajectory {
            px,
            py,
            pz,
            vx,
            vy,
            vz,
        },
    ))
}

pub(super) fn parse_trajectories(input: &str) -> IResult<&str, Box<[Trajectory]>> {
    all_consuming(map(separated_list0(line_ending, trajectory), |v| {
        v.into_boxed_slice()
    }))(input)
}
