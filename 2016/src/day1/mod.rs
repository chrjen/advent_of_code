use std::collections::HashSet;

use regex::Regex;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 1: No Time for a Taxicab",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

#[derive(Debug, Clone, Copy)]
enum Rotation {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate(&mut self, rot: Rotation) {
        match (*self, rot) {
            (Direction::Up, Rotation::Left) => *self = Direction::Left,
            (Direction::Up, Rotation::Right) => *self = Direction::Right,
            (Direction::Down, Rotation::Left) => *self = Direction::Right,
            (Direction::Down, Rotation::Right) => *self = Direction::Left,
            (Direction::Left, Rotation::Left) => *self = Direction::Down,
            (Direction::Left, Rotation::Right) => *self = Direction::Up,
            (Direction::Right, Rotation::Left) => *self = Direction::Up,
            (Direction::Right, Rotation::Right) => *self = Direction::Down,
        }
    }

    fn walk(&self, dist: i32, (x, y): &mut (i32, i32)) {
        match self {
            Direction::Up => *y += dist,
            Direction::Down => *y -= dist,
            Direction::Left => *x -= dist,
            Direction::Right => *x += dist,
        }
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let reg = Regex::new(r"([RL])(\d+),?\s*").unwrap();

    let walk: Vec<(Rotation, i32)> = reg
        .captures_iter(&input)
        .map(|cap| match &cap[1] {
            "R" => (Rotation::Right, cap[2].parse().unwrap()),
            "L" => (Rotation::Left, cap[2].parse().unwrap()),
            _ => unreachable!(),
        })
        .collect();

    // Part 1.
    let mut dir = Direction::Up;
    let mut pos = (0, 0);
    for &(rot, dist) in walk.iter() {
        dir.rotate(rot);
        dir.walk(dist, &mut pos);
    }
    let part1 = (pos.0 + pos.1).abs();

    // Part 2,
    let mut dir = Direction::Up;
    let mut pos = (0, 0);
    let mut part2 = 0;
    let mut map = HashSet::new();
    map.insert((0, 0));
    'outer: for &(rot, dist) in walk.iter() {
        dir.rotate(rot);
        for _ in 0..dist {
            dir.walk(1, &mut pos);
            if !map.insert(pos) {
                part2 = (pos.0 + pos.1).abs();
                break 'outer;
            }
        }
    }

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(p1, p1_example_1, "R2, L3", "5");
    example!(p1, p1_example_2, "R2, R2, R2", "2");
    example!(p1, p1_example_3, "R5, L5, R5, R3", "12");
    solution!(p1, p1_solution, "353");

    // Part 2
    example!(p2, p2_example_1, "R8, R4, R4, R8,", "4");
    solution!(p2, p2_solution, "152");
}
