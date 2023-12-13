use nom::{
    character::complete::{self, line_ending, one_of, space1},
    combinator::{all_consuming, map, map_res},
    multi::{many0, many1, separated_list1},
    sequence::{separated_pair, terminated},
    Err, IResult, Parser,
};

use super::nonogram::{Row, Tile};

fn tiles(input: &str) -> IResult<&str, Vec<Tile>> {
    many1(map_res(one_of("?.#"), |c| match c {
        '?' => Ok(Tile::Empty),
        '#' => Ok(Tile::Fill),
        '.' => Ok(Tile::Cross),
        _ => Err(Err::Failure(format!("found unknown tile {c}"))),
    }))(input)
}

fn hints(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(complete::char(','), complete::u64.map(|v| v as usize))(input)
}

pub(super) fn row(input: &str) -> IResult<&str, Row> {
    map(separated_pair(tiles, space1, hints), |(tiles, hints)| Row {
        tiles,
        hints,
    })(input)
}

pub(super) fn parse_rows(input: &str) -> IResult<&str, Vec<Row>> {
    all_consuming(terminated(
        separated_list1(line_ending, row),
        many0(line_ending),
    ))(input)
}
