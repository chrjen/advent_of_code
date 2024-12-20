pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 20: Race Condition",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use nalgebra::Vector2;
use rayon::prelude::*;

pub fn solve(input: &[u8]) -> (String, String) {
    solve_(input, 100, 100)
}

pub fn solve_(input: &[u8], p1_threshold: u32, p2_threshold: u32) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let mut walls: HashSet<Vector2<i32>> = HashSet::new();
    let mut end: Option<Vector2<i32>> = None;

    for (y, line) in (1..).zip(input.lines()) {
        for (x, c) in (1..).zip(line.chars()) {
            match c {
                'E' | 'e' => {
                    if end.is_some() {
                        panic!("found multiple end locations");
                    }
                    end = Some(Vector2::new(x, y))
                }
                '#' => {
                    walls.insert(Vector2::new(x, y));
                }
                _ => {}
            }
        }
    }

    let end = end.unwrap_or_else(|| panic!("input missing end location"));

    let mut next: VecDeque<(u32, Vector2<i32>)> = VecDeque::new();
    let mut visited: HashSet<Vector2<i32>> = HashSet::new();
    let mut distances: HashMap<Vector2<i32>, u32> = HashMap::new();
    next.push_back((0, end));

    const OFFSETS: &[Vector2<i32>] = &[
        Vector2::new(1, 0),
        Vector2::new(0, 1),
        Vector2::new(-1, 0),
        Vector2::new(0, -1),
    ];

    while let Some((distance, current)) = next.pop_front() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);
        distances.insert(current, distance);

        for offset in OFFSETS {
            let neighbour = current + offset;
            if !walls.contains(&neighbour) {
                next.push_back((distance + 1, neighbour));
            }
        }
    }

    // Part 1
    let cheat_offsets = &[
        Vector2::new(2, 0),
        Vector2::new(0, 2),
        Vector2::new(-2, 0),
        Vector2::new(0, -2),
    ];

    let part1: u32 = distances
        .par_iter()
        .map(|(current, distance)| {
            let mut count = 0;
            for offset in cheat_offsets {
                let neighbour = current + offset;
                if let Some(&other_distance) = distances.get(&neighbour) {
                    if other_distance > (distance + 2)
                        && other_distance - (distance + 2) >= p1_threshold
                    {
                        count += 1;
                    }
                }
            }
            count
        })
        .sum();

    // Part 2
    let cheat_offsets = (-20..=20_i32)
        .cartesian_product(-20..=20_i32)
        .filter_map(|(x, y)| {
            let distance = (x.abs() + y.abs()) as u32;
            if distance <= 20 {
                Some((distance, Vector2::new(x, y)))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let part2: u32 = distances
        .par_iter()
        .map(|(current, distance)| {
            let mut count = 0;
            for (cheat_distance, offset) in cheat_offsets.iter() {
                let neighbour = current + offset;
                if let Some(&other_distance) = distances.get(&neighbour) {
                    if other_distance > (distance + cheat_distance)
                        && other_distance - (distance + cheat_distance) >= p2_threshold
                    {
                        count += 1;
                    }
                }
            }
            count
        })
        .sum();

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::solution;

    // Part 1
    #[test]
    fn p1_example_1() {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#.#####.#.#.###
#.###.#.#.#...#
#.###.#.#.###.#
#.#..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        println!("input: {}", input);
        let (result, _) = solve_(str::as_bytes(input), 8, 70);
        assert_eq!(result, "19");
    }
    solution!(p1, p1_solution, "1524");

    // Part 2
    #[test]
    fn p2_example_1() {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        println!("input: {}", input);
        let (_, result) = solve_(str::as_bytes(input), 8, 70);
        assert_eq!(result, "41");
    }
    solution!(p2, p2_solution, "1033746");
}
