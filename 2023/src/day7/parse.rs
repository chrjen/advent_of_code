use nom::{
    bytes::complete::take,
    character::complete::{self, line_ending, one_of, space1},
    combinator::{map, map_parser, map_res},
    multi::{many1, separated_list0},
    sequence::separated_pair,
    IResult,
};

use super::{Card, Hand};

pub(super) fn parse_card(input: &str) -> IResult<&str, Card> {
    map_res(one_of("AKQJT98765432"), Card::from_char)(input)
}

pub(super) fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let parse_cards = map_parser(
        take(5usize),
        map_res(many1(parse_card), |v| v.into_boxed_slice().try_into()),
    );

    map(
        separated_pair(parse_cards, space1, complete::u32),
        |(cards, bet)| Hand::new(cards, bet),
    )(input)
}

pub(super) fn parse_hands(input: &str) -> IResult<&str, Box<[Hand]>> {
    map(separated_list0(line_ending, parse_hand), |v| {
        v.into_boxed_slice()
    })(input)
}
