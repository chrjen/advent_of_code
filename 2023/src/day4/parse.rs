use std::collections::HashSet;

use nom::{
    IResult,
    bytes::complete::{tag, tag_no_case},
    character::complete::{self, space0, space1},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
};

pub(super) fn parse_numbers(input: &str) -> IResult<&str, HashSet<u32>> {
    let list = separated_list1(space1, complete::u32);
    delimited(space0, list, space0)(input).map(|(input, v)| (input, v.into_iter().collect()))
}

pub(super) fn parse_card(input: &str) -> IResult<&str, (HashSet<u32>, HashSet<u32>)> {
    let title = delimited(
        tag_no_case("card"),
        delimited(space1, complete::u32, space0),
        tag(":"),
    );
    let numbers = separated_pair(parse_numbers, tag("|"), parse_numbers);

    preceded(title, numbers)(input)
}
