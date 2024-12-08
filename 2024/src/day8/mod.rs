pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 8: Resonant Collinearity",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

use std::{
    collections::{HashMap, HashSet},
    vec,
};

use itertools::Itertools;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let max_x: i32 = input
        .lines()
        .next()
        .map_or(1, |v| v.len())
        .try_into()
        .expect("should fit inside an i32");
    let max_y: i32 = input
        .lines()
        .count()
        .try_into()
        .expect("should fit inside an i32");

    for (y, line) in (1..).zip(input.lines()) {
        for (x, c) in (1..).zip(line.chars()) {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    antennas
                        .entry(c)
                        .and_modify(|v| v.push((x, y)))
                        .or_insert_with(|| vec![(x, y)]);
                }
                '.' | '#' => { /* Do nothing */ }
                _ => {
                    panic!("Found unknown character '{c}' in input");
                }
            }
        }
    }

    // Part 1
    let part1: usize = {
        let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

        for freq_antennas in antennas.values() {
            for (antenna1, antenna2) in freq_antennas.iter().combinations(2).flatten().tuples() {
                let offset = (antenna1.0 - antenna2.0, antenna1.1 - antenna2.1);
                antinodes.insert((antenna1.0 + offset.0, antenna1.1 + offset.1));
                antinodes.insert((antenna2.0 - offset.0, antenna2.1 - offset.1));
            }
        }

        antinodes
            .iter()
            .filter(|&(x, y)| (1..=max_x).contains(x) && (1..=max_y).contains(y))
            .count()
    };

    // Part 2
    let part2: usize = {
        let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

        for freq_antennas in antennas.values() {
            for (antenna1, antenna2) in freq_antennas.iter().combinations(2).flatten().tuples() {
                let offset = (antenna1.0 - antenna2.0, antenna1.1 - antenna2.1);

                for n in 0.. {
                    let pos = (antenna1.0 + n * offset.0, antenna1.1 + n * offset.1);
                    if (1..=max_x).contains(&pos.0) && (1..=max_y).contains(&pos.1) {
                        antinodes.insert(pos);
                    } else {
                        break;
                    }
                }

                for n in 0.. {
                    let pos = (antenna2.0 - n * offset.0, antenna2.1 - n * offset.1);
                    if (1..=max_x).contains(&pos.0) && (1..=max_y).contains(&pos.1) {
                        antinodes.insert(pos);
                    } else {
                        break;
                    }
                }
            }
        }

        // println!();
        // for y in 0..=max_y {
        //     for x in 0..=max_x {
        //         if antinodes.contains(&(x, y)) {
        //             print!("#");
        //         } else {
        //             print!(".")
        //         }
        //     }
        //     println!();
        // }

        antinodes.len()
    };

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........",
        "2"
    );
    example!(
        p1,
        p1_example_2,
        "..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
..........",
        "4"
    );
    example!(
        p1,
        p1_example_3,
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
        "14"
    );
    solution!(p1, p1_solution, "426");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........",
        "9"
    );
    example!(
        p2,
        p2_example_2,
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
        "34"
    );
    solution!(p2, p2_solution, "1359");
}
