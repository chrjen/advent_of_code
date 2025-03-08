use std::collections::HashMap;

use nom::{
    IResult,
    bytes::complete::take_till,
    character::complete::{self, alphanumeric1, line_ending, one_of, space0},
    combinator::{all_consuming, map, opt},
    multi::{fold_many0, many0},
    sequence::{delimited, separated_pair, terminated},
};

use super::{Direction, Map, Node};

pub(super) fn parse_instructions(input: &str) -> IResult<&str, Vec<Direction>> {
    many0(map(one_of("RrLl"), |c| Direction::try_from(c).unwrap()))(input)
}

pub(super) fn parse_node(input: &str) -> IResult<&str, (&str, Node)> {
    let equal = delimited(space0, complete::char('='), space0);
    let comma = delimited(space0, complete::char(','), space0);
    let node_parser = map(
        delimited(
            complete::char('('),
            separated_pair(alphanumeric1, comma, alphanumeric1),
            complete::char(')'),
        ),
        |(left, right)| Node { left, right },
    );
    separated_pair(alphanumeric1, equal, node_parser)(input)
}

pub(super) fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, instructions) = parse_instructions(input)?;
    let (input, _) = take_till(|c| c != '\n')(input)?;
    let (input, nodes) = all_consuming(fold_many0(
        terminated(parse_node, opt(line_ending)),
        HashMap::new,
        |mut acc: HashMap<_, _>, (name, node)| {
            acc.insert(name, node);
            acc
        },
    ))(input)?;

    Ok((
        input,
        Map {
            instructions,
            nodes,
        },
    ))
}
