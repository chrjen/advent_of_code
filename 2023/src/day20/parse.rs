use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, space0},
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};

use super::data::{Module, Pulse, System};

pub(super) fn spaced_separator(separator: &str) -> impl Fn(&str) -> IResult<&str, &str> + '_ {
    move |input: &str| -> IResult<&str, &str> { delimited(space0, tag(separator), space0)(input) }
}

pub(super) fn outputs(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(spaced_separator(","), alpha1)(input)
}

pub(super) fn broadcaster(input: &str) -> IResult<&str, (&str, Module)> {
    map(
        separated_pair(tag("broadcaster"), spaced_separator("->"), outputs),
        |(name, outputs)| (name, Module::Broadcaster { outputs }),
    )(input)
}

pub(super) fn flip_flop(input: &str) -> IResult<&str, (&str, Module)> {
    map(
        separated_pair(
            preceded(complete::char('%'), alpha1),
            spaced_separator("->"),
            outputs,
        ),
        |(name, outputs)| {
            (
                name,
                Module::FlipFlop {
                    state: Pulse::Low,
                    outputs,
                },
            )
        },
    )(input)
}

pub(super) fn conjunction(input: &str) -> IResult<&str, (&str, Module)> {
    map(
        separated_pair(
            preceded(complete::char('&'), alpha1),
            spaced_separator("->"),
            outputs,
        ),
        |(name, outputs)| {
            (
                name,
                Module::Conjunction {
                    inputs: Default::default(),
                    outputs,
                },
            )
        },
    )(input)
}

pub(super) fn module(input: &str) -> IResult<&str, (&str, Module)> {
    alt((broadcaster, flip_flop, conjunction))(input)
}

pub(super) fn parse_system(input: &str) -> IResult<&str, System> {
    let system_parser = separated_list0(line_ending, module)
        .map(|v| v.into_iter().collect::<HashMap<&str, Module>>())
        .map(System::new);

    all_consuming(system_parser)(input)
}
