use nom::{
    branch::alt,
    bytes::complete::{tag_no_case, take_till},
    character::complete::{self, newline, space0, space1},
    combinator::{map_parser, opt},
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    IResult,
};

use super::{Game, Round};

pub(super) fn parse_cube<'a>(
    colour: &'a str,
) -> impl Fn(&'a str) -> IResult<&'a str, (u32, &'a str)> {
    move |input: &'a str| -> IResult<&'a str, (u32, &str)> {
        let separator = tuple((complete::char(','), space0));
        terminated(
            tuple((terminated(complete::u32, space0), tag_no_case(colour))),
            opt(separator),
        )(input)
    }
}

pub(super) fn parse_round(input: &str) -> IResult<&str, Round> {
    let cube_delimiter = delimited(space0, complete::char(','), space0);
    separated_list1(
        cube_delimiter,
        map_parser(
            take_till(|c| c == ','),
            alt((parse_cube("red"), parse_cube("green"), parse_cube("blue"))),
        ),
    )(input)
    .map(|(s, cubes)| {
        let mut round = Round {
            red: None,
            green: None,
            blue: None,
        };

        for &(count, colour) in cubes.iter() {
            match colour {
                "red" => round.red = Some(count),
                "green" => round.green = Some(count),
                "blue" => round.blue = Some(count),
                _ => unreachable!("got unknown colour {colour}"),
            }
        }

        (s, round)
    })
}

pub(super) fn parse_game(input: &str) -> IResult<&str, Game> {
    let game_id = delimited(
        tuple((tag_no_case("game"), space1)),
        complete::u32,
        tuple((complete::char(':'), space0)),
    );
    let round_delimiter = delimited(space0, complete::char(';'), space0);
    tuple((
        game_id,
        separated_list1(
            round_delimiter,
            map_parser(take_till(|c| c == ';'), parse_round),
        ),
    ))(input)
    .map(|(s, (id, rounds))| (s, Game { id, rounds }))
}

pub(super) fn parse_game0(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(newline, map_parser(take_till(|c| c == '\n'), parse_game))(input)
}
