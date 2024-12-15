pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 13: Claw Contraption",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

use itertools::Itertools;
use nalgebra::{Matrix2, Vector2};
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
struct ClawMachine {
    button_a: Vector2<i64>,
    button_b: Vector2<i64>,
    prize: Vector2<i64>,
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let mut machines: Vec<ClawMachine> = Vec::new();

    // A little lazy parsing the input today, but it works.
    let re = Regex::new(r"\d+").unwrap();
    for section in input.split("\n\n") {
        let mut lines = section.lines();
        let button_a: (i64, i64) = re
            .find_iter(lines.next().unwrap())
            .map(|m| m.as_str().parse().unwrap())
            .next_tuple()
            .unwrap();
        let button_b: (i64, i64) = re
            .find_iter(lines.next().unwrap())
            .map(|m| m.as_str().parse().unwrap())
            .next_tuple()
            .unwrap();
        let prize: (i64, i64) = re
            .find_iter(lines.next().unwrap())
            .map(|m| m.as_str().parse().unwrap())
            .next_tuple()
            .unwrap();
        machines.push(ClawMachine {
            button_a: Vector2::new(button_a.0, button_a.1),
            button_b: Vector2::new(button_b.0, button_b.1),
            prize: Vector2::new(prize.0, prize.1),
        });
    }

    // Part 1
    let mut part1: i64 = 0;
    for machine in machines.iter() {
        let mat = Matrix2::from_columns(&[machine.button_a, machine.button_b]);
        let mat_inv = Matrix2::new(mat.m22, -mat.m12, -mat.m21, mat.m11);
        let mat_det = mat.m11 * mat.m22 - mat.m12 * mat.m21;
        let mut presses = mat_inv * machine.prize;

        if presses.x.rem_euclid(mat_det) == 0 && presses.y.rem_euclid(mat_det) == 0 {
            presses /= mat_det;
            part1 += presses.x * 3 + presses.y
        }
    }

    // Part 2
    let mut part2: i64 = 0;
    for machine in machines.iter_mut() {
        machine.prize += Vector2::new(10000000000000, 10000000000000);
        let mat = Matrix2::from_columns(&[machine.button_a, machine.button_b]);
        let mat_inv = Matrix2::new(mat.m22, -mat.m12, -mat.m21, mat.m11);
        let mat_det = mat.m11 * mat.m22 - mat.m12 * mat.m21;
        let mut presses = mat_inv * machine.prize;

        if presses.x.rem_euclid(mat_det) == 0 && presses.y.rem_euclid(mat_det) == 0 {
            presses /= mat_det;
            part2 += presses.x * 3 + presses.y
        }
    }

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
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
        "480"
    );
    solution!(p1, p1_solution, "33921");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
        "875318608908"
    );
    solution!(p2, p2_solution, "82261957837868");
}
