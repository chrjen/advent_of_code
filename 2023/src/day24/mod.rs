pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 24: Never Tell Me The Odds",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod data;
mod parse;

use self::data::Trajectory;
use itertools::Itertools;
use num::{BigInt, BigRational, Zero};
use std::ops::RangeInclusive;

pub fn solve(input: &[u8]) -> (String, String) {
    _solve(
        input,
        BigRational::from_integer(BigInt::from(200000000000000_u64))
            ..=BigRational::from_integer(BigInt::from(400000000000000_u64)),
    )
}

pub fn _solve(input: &[u8], range: RangeInclusive<BigRational>) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let trajectories = parse::parse_trajectories(&input)
        .expect("input should be valid")
        .1;

    // Part 1
    let part1: usize = trajectories
        .iter()
        .tuple_combinations()
        .map(|(t0, t1)| {
            if let Some((ref x, ref y)) = Trajectory::trajectory_intersection(t0, t1) {
                if range.contains(x)
                    && range.contains(y)
                    && t0.point_time(x, y) >= BigRational::zero()
                    && t1.point_time(x, y) >= BigRational::zero()
                {
                    1
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum();

    (part1.to_string(), 0.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::solution;
    use num::{BigInt, BigRational};

    #[test]
    fn p1_example_1() {
        let input = "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        println!("input: {input}");
        let (result, _) = _solve(
            str::as_bytes(input),
            BigRational::from_integer(BigInt::from(7))
                ..=BigRational::from_integer(BigInt::from(27)),
        );
        assert_eq!(result, "2");
    }
    solution!(
        p1,
        p1_solution,
        "12740",
        ignore = "takes too long in debug mode"
    );

    // Part 2
    // example!(p2, p2_example_1, "", "0");
    // example!(p2, p2_example_2, "", "0");
    // example!(p2, p2_example_3, "", "0");
    // example!(p2, p2_example_4, "", "0");
    // example!(p2, p2_example_5, "", "0");
    // solution!(p2, p2_solution, "100");
}
