use nom::{
    IResult,
    branch::alt,
    character::complete::{self, alpha1},
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::{separated_pair, terminated},
};

use super::data::{Operation, Step};

pub(super) fn step_dash(input: &str) -> IResult<&str, Step> {
    map(terminated(alpha1, complete::char('-')), |label: &str| {
        Step::new(label, Operation::Dash)
    })(input)
}

pub(super) fn step_equal(input: &str) -> IResult<&str, Step> {
    map(
        separated_pair(alpha1, complete::char('='), complete::u8),
        |(label, focal_length)| Step::new(label, Operation::Equal(focal_length)),
    )(input)
}

pub(super) fn parse_steps(input: &str) -> IResult<&str, Box<[Step]>> {
    map(
        all_consuming(separated_list0(
            complete::char(','),
            alt((step_dash, step_equal)),
        )),
        |v| v.into_boxed_slice(),
    )(input)
}
