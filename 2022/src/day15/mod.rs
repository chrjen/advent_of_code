use std::collections::HashMap;

mod data;
mod parse;

use data::Tile;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 15: Beacon Exclusion Zone",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    solve_for(2_000_000, 4000000, input)
}

fn solve_for(y: i32, bounds: i32, input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let locations: Vec<(data::Loc, data::Loc)> = parse::sensor_outputs_parser(&input)
        .unwrap_or_else(|err| panic!("parsing error: {err}"))
        .1;

    let mut tiles = HashMap::new();
    for (sensor, beacon) in locations.iter() {
        tiles.insert(*sensor, Tile::Sensor(sensor.manhattan(beacon)));
        tiles.insert(*beacon, Tile::Beacon);
    }

    let world = data::World { tiles };

    (
        world.scan_line(y).to_string(),
        common::from_option(
            world
                .locate_beacon(0..=bounds, 0..=bounds)
                .map(|l| l.x as u64 * 4000000 + l.y as u64),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::solution;

    // Part 1
    #[test]
    pub fn p1_example_1() {
        let input = b"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let (result, _) = solve_for(10, 20, input);
        assert_eq!(result, "26");
    }
    solution!(
        p1,
        p1_solution,
        "5564017",
        ignore = "too slow in debug release"
    );

    // Part 2
    #[test]
    pub fn p2_example_1() {
        let input = b"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let (_, result) = solve_for(10, 20, input);
        assert_eq!(result, "56000011");
    }
    solution!(
        p2,
        p2_solution,
        "11558423398893",
        ignore = "too slow in debug release"
    );
}
