use std::collections::HashSet;

use super::Coord;

pub(super) fn parse_garden_map(input: &str) -> (Coord, HashSet<Coord>) {
    let mut rocks = HashSet::new();
    let mut start = None;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    rocks.insert((x as i32 + 1, y as i32 + 1));
                }
                'S' => {
                    start = Some((x as i32 + 1, y as i32 + 1));
                }
                '.' => {}
                _ => panic!(
                    "got unknown character '{c}' at line: {}, col: {}",
                    y + 1,
                    x + 1
                ),
            }
        }
    }

    (start.expect("input should contain a start square"), rocks)
}
