use super::data::{HikingTrails, Tile};

pub(super) fn parse_hiking_trails(input: &str) -> HikingTrails {
    let mut trails = HikingTrails::new();
    let mut start = None;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = (x as i32 + 1, y as i32 + 1);
            match c {
                '#' => {
                    trails.map.insert(coord, Tile::Forest);
                }
                '^' => {
                    trails.map.insert(coord, Tile::SlopeNorth);
                }
                '>' => {
                    trails.map.insert(coord, Tile::SlopeEast);
                }
                'v' => {
                    trails.map.insert(coord, Tile::SlopeSouth);
                }
                '<' => {
                    trails.map.insert(coord, Tile::SlopeWest);
                }
                '.' => {
                    start.get_or_insert(coord);
                    trails.end = coord;
                }
                _ => panic!(
                    "got unknown character '{c}' at line: {}, col: {}",
                    y + 1,
                    x + 1
                ),
            }
        }
    }

    trails.start = start.expect("there should be at least one empty space");

    trails
}
