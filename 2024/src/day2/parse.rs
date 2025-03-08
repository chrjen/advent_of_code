use nom::{
    IResult,
    character::complete::{self, line_ending, space1},
    combinator::all_consuming,
    multi::{separated_list0, separated_list1},
};

pub(super) fn parse_report(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space1, complete::i32)(input)
}

pub(super) fn parse_reports(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    all_consuming(separated_list0(line_ending, parse_report))(input)
}
