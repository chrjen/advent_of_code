use std::collections::HashMap;

use nom::{
    Parser,
    branch::alt,
    character::complete::{self, one_of},
    multi::many0,
};

use super::{Movement, Tile};

pub(super) fn map(input: &str) -> Result<HashMap<(usize, usize), Tile>, String> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().flat_map(move |(col, c)| {
                if c == ' ' {
                    None
                } else {
                    let result = Tile::try_from(c);
                    Some(
                        result
                            .map(|tile| ((col + 1, row + 1), tile))
                            .map_err(|err| format!("Line {}:{}, {}", row, col, err)),
                    )
                }
            })
        })
        .collect()
}

pub(super) fn path(input: &str) -> Result<Vec<Movement>, nom::Err<nom::error::Error<&str>>> {
    many0::<_, _, nom::error::Error<_>, _>(alt((
        complete::u32.map(Movement::Forward),
        one_of("RL").map(|c| match c {
            'R' => Movement::Right,
            'L' => Movement::Left,
            _ => unreachable!(),
        }),
    )))(input)
    .map(|(_, path)| path)
}
