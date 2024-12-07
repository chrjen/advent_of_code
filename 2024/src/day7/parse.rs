use nom::{
    character::complete::{self, line_ending, space0, space1},
    combinator::all_consuming,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

fn arguments(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, complete::i64)(input)
}

fn equation(input: &str) -> IResult<&str, (i64, Vec<i64>)> {
    separated_pair(
        complete::i64,
        complete::char(':'),
        delimited(space0, arguments, space0),
    )(input)
}

pub(super) fn parse_reports(input: &str) -> IResult<&str, Vec<(i64, Vec<i64>)>> {
    all_consuming(separated_list0(line_ending, equation))(input)
}
