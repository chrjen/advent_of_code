use nom::{
    IResult,
    character::complete::{self, alpha1, line_ending, multispace0, space0},
    error::Error,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
};

fn towel_patterns(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(delimited(space0, complete::char(','), space0), alpha1)(input)
}

fn towel_designs(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(line_ending, alpha1)(input)
}

type Towels<'a> = (Vec<&'a str>, Vec<&'a str>);
pub(super) fn parse_input(input: &str) -> Result<Towels, nom::Err<Error<&str>>> {
    separated_pair(towel_patterns, multispace0, towel_designs)(input).map(|(_, v)| v)
}
