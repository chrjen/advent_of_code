use itertools::Itertools;
use nalgebra::Vector5;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 25: Code Chronicle",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let mut locks: Vec<Vector5<u8>> = Vec::new();
    let mut keys: Vec<Vector5<u8>> = Vec::new();

    for schematic in input.split("\n\n") {
        let mut lines = schematic.lines();
        let first_line = lines.next().expect("schematic should not be empty");

        let mut lock_or_key = Vector5::new(0, 0, 0, 0, 0);
        for line in lines {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    lock_or_key[i] += 1;
                }
            }
        }

        if first_line == "#####" {
            locks.push(lock_or_key);
        } else if first_line == "....." {
            keys.push(lock_or_key);
        } else {
            panic!("Unknown schematic type:\n{schematic}")
        }
    }

    let mut part1 = 0;
    for (lock, key) in locks.iter().cartesian_product(keys.iter()) {
        if (lock + key).into_iter().all(|pin| *pin <= 6) {
            part1 += 1;
        }
    }

    (part1.to_string(), "Not solved, yet".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####",
        "3"
    );
    solution!(p1, p1_solution, "3242");

    // Part 2
    // example!(p2, p2_example_1, "", "0");
    // example!(p2, p2_example_2, "", "0");
    // example!(p2, p2_example_3, "", "0");
    // example!(p2, p2_example_4, "", "0");
    // example!(p2, p2_example_5, "", "0");
    // solution!(p2, p2_solution, "100");
}
