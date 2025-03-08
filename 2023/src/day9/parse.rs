use nom::{
    IResult, Parser,
    character::complete::{self, line_ending, space1},
    combinator::{all_consuming, opt},
    multi::{many0, separated_list1},
    sequence::terminated,
};

use super::Sequence;

pub(super) fn sequence(input: &str) -> IResult<&str, Sequence> {
    terminated(
        separated_list1(space1, complete::i64).map(Sequence::new),
        opt(line_ending),
    )(input)
}

pub(super) fn parse_sequences(input: &str) -> IResult<&str, Vec<Sequence>> {
    all_consuming(many0(sequence))(input)
}
