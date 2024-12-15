use std::collections::HashMap;

use nalgebra::Vector2;

use super::data::{Direction, Tile};

pub type Output = (HashMap<Vector2<i32>, Tile>, Vec<Direction>);

fn map(input: &str) -> Result<HashMap<Vector2<i32>, Tile>, &str> {
    let mut map = HashMap::new();
    for (y, line) in (0..).zip(input.lines()) {
        for (x, c) in (0..).zip(line.chars()) {
            if let Ok(tile) = Tile::try_from_char(c) {
                map.insert(Vector2::new(x, y), tile);
            }
        }
    }

    Ok(map)
}

fn directions(input: &str) -> Result<Vec<Direction>, &str> {
    Ok(input.chars().flat_map(Direction::try_from_char).collect())
}

pub(super) fn parse_input(input: &str) -> Result<Output, &str> {
    let mut split = input.split("\n\n");
    let map_input = split.next().ok_or("map input missing")?;
    let directions_input = split.next().ok_or("directions input missing")?;
    Ok((map(map_input)?, directions(directions_input)?))
}
