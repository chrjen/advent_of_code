use nalgebra::Vector2;
use nom::{
    character::complete::{self, line_ending},
    combinator::all_consuming,
    error::{Error, ErrorKind},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

use super::Robot;

fn vector(input: &str) -> IResult<&str, Vector2<i64>> {
    separated_pair(complete::i64, complete::char(','), complete::i64)(input)
        .map(|(input, (v0, v1))| (input, Vector2::new(v0, v1)))
}

fn property(input: &str) -> IResult<&str, (char, Vector2<i64>)> {
    separated_pair(complete::anychar, complete::char('='), vector)(input)
}

fn robot(input: &str) -> IResult<&str, Robot> {
    let (input, (left, right)) = separated_pair(property, complete::space1, property)(input)?;
    let (pos, vel) = match (left, right) {
        (('p', pos), ('v', vel)) => (pos, vel),
        (('v', vel), ('p', pos)) => (pos, vel),
        _ => return Err(nom::Err::Error(Error::new(input, ErrorKind::NoneOf))),
    };
    Ok((input, Robot { pos, vel }))
}

pub(super) fn parse_robots(input: &str) -> Result<Vec<Robot>, nom::Err<Error<&str>>> {
    all_consuming(separated_list0(line_ending, robot))(input).map(|(_, v)| v)
}
