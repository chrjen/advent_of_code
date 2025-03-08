use nom::{
    IResult,
    character::complete::{self, line_ending, space0},
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::{delimited, separated_pair, terminated, tuple},
};

use super::data::Block;

pub(super) fn coords(input: &str) -> IResult<&str, (i32, i32, i32)> {
    tuple((
        terminated(
            delimited(space0, complete::i32, space0),
            complete::char(','),
        ),
        terminated(
            delimited(space0, complete::i32, space0),
            complete::char(','),
        ),
        delimited(space0, complete::i32, space0),
    ))(input)
}

pub(super) fn block(input: &str) -> IResult<&str, Block> {
    map(
        separated_pair(coords, complete::char('~'), coords),
        |((x0, y0, z0), (x1, y1, z1))| Block::new(x0, y0, z0, x1, y1, z1),
    )(input)
}

pub(super) fn blocks(input: &str) -> IResult<&str, Vec<Block>> {
    all_consuming(separated_list0(line_ending, block))(input)
}
