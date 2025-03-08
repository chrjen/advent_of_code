use fxhash::FxHashMap;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        self, alphanumeric1, line_ending, multispace0, multispace1, one_of, space0, space1,
    },
    combinator::map,
    error::Error,
    multi::{fold_many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
};

use super::data::LogicGate;

fn logic_input(input: &str) -> IResult<&str, (&str, bool)> {
    separated_pair(
        alphanumeric1,
        delimited(space0, complete::char(':'), space0),
        map(one_of("01"), |s| match s {
            '0' => false,
            '1' => true,
            _ => unreachable!(),
        }),
    )(input)
}

fn logic_inputs(input: &str) -> IResult<&str, FxHashMap<&str, bool>> {
    fold_many1(
        terminated(logic_input, multispace1),
        FxHashMap::default,
        |mut acc, (key, value)| {
            acc.insert(key, value);
            acc
        },
    )(input)
}

fn logic_gate(input: &str) -> IResult<&str, LogicGate> {
    let operation = alt((tag("AND"), tag("OR"), tag("XOR")));
    let (input, (left, op, right, output)) = tuple((
        alphanumeric1,
        delimited(space1, operation, space1),
        alphanumeric1,
        preceded(delimited(space1, tag("->"), space1), alphanumeric1),
    ))(input)?;

    match op {
        "AND" => Ok((input, LogicGate::And(left, right, output))),
        "OR" => Ok((input, LogicGate::Or(left, right, output))),
        "XOR" => Ok((input, LogicGate::Xor(left, right, output))),
        _ => unreachable!(),
    }
}

fn logic_gates(input: &str) -> IResult<&str, Vec<LogicGate>> {
    separated_list1(line_ending, logic_gate)(input)
}

type LogicCircuit<'a> = (FxHashMap<&'a str, bool>, Vec<LogicGate<'a>>);
pub(super) fn parse_input(input: &str) -> Result<LogicCircuit, nom::Err<Error<&str>>> {
    separated_pair(logic_inputs, multispace0, logic_gates)(input).map(|(_, v)| v)
}
