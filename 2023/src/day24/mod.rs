pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 24: Never Tell Me The Odds",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod data;
mod parse;

use self::data::{Num, Trajectory};
use itertools::Itertools;
use num::Zero;
use std::ops::RangeInclusive;

pub fn solve(input: &[u8]) -> (String, String) {
    _solve(
        input,
        Num::from_integer(200000000000000_i128)..=Num::from_integer(400000000000000_i128),
    )
}

pub fn _solve(input: &[u8], range: RangeInclusive<Num>) -> (String, String) {
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
                    && t0.point_time(x, y) >= Num::zero()
                    && t1.point_time(x, y) >= Num::zero()
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

    // Part 2
    let (px, py, pz) =
        Trajectory::rock_trajectory_position(&trajectories[0], &trajectories[1], &trajectories[2]);
    let part2 = px + py + pz;

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

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
            Num::from_integer(7)..=Num::from_integer(27),
        );
        assert_eq!(result, "2");
    }
    solution!(p1, p1_solution, "12740");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3",
        "47"
    );
    // Start position: (131633231355646, 371683716481156, 238674624073734)
    // Velocity      : (268, -197, 68)
    solution!(p2, p2_solution, "741991571910536");
}
