use nom::{
    character::complete::{self, line_ending, space1},
    combinator::all_consuming,
    multi::separated_list0,
    sequence::separated_pair,
    IResult, Parser,
};

pub(super) fn parse_line(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(complete::u32, space1, complete::u32)(input)
}

pub(super) fn parse_lists(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    all_consuming(separated_list0(line_ending, parse_line).map(|v| {
        v.into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut lvec, mut rvec), (lvalue, rvalue)| {
                lvec.push(lvalue);
                rvec.push(rvalue);
                (lvec, rvec)
            },
        )
    }))(input)
}
