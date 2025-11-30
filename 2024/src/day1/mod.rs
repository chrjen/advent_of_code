pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 1: Historian Hysteria",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod parse;

trait Solver {
    fn solve_partial() -> Box<dyn PartSolver>;
}

trait PartSolver {
    fn solve_part1(&self) -> u32;
    fn solve_part2(&self) -> u32;
}

struct Data {
    left_list: Vec<u32>,
    right_list: Vec<u32>,
}

impl PartSolver for Data {
    fn solve_part1(&self) -> u32 {
        return self
            .left_list
            .iter()
            .zip(self.right_list.iter())
            .map(|(lhs, rhs)| lhs.abs_diff(*rhs))
            .sum();
    }

    fn solve_part2(&self) -> u32 {
        let freq_map = self
            .right_list
            .iter()
            .fold(HashMap::<u32, u32>::new(), |mut acc, value| {
                *acc.entry(*value).or_insert(0) += 1;
                acc
            });

        return self
            .left_list
            .iter()
            .map(|value| value * freq_map.get(value).unwrap_or(&0))
            .sum();
    }
}

use std::collections::HashMap;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let (_, (mut left_list, mut right_list)) =
        parse::parse_lists(input).expect("input should be valid");

    left_list.sort_unstable();
    right_list.sort_unstable();

    let common = Common {
        left_list,
        right_list,
    };

    (
        solve_part1(&common).to_string(),
        solve_part2(&common).to_string(),
    )
}

struct Common {
    left_list: Vec<u32>,
    right_list: Vec<u32>,
}

fn solve_part1(common: &Common) -> u32 {
    return common
        .left_list
        .iter()
        .zip(common.right_list.iter())
        .map(|(lhs, rhs)| lhs.abs_diff(*rhs))
        .sum();
}

fn solve_part2(common: &Common) -> u32 {
    let freq_map = common
        .right_list
        .iter()
        .fold(HashMap::<u32, u32>::new(), |mut acc, value| {
            *acc.entry(*value).or_insert(0) += 1;
            acc
        });

    return common
        .left_list
        .iter()
        .map(|value| value * freq_map.get(value).unwrap_or(&0))
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "3   4
4   3
2   5
1   3
3   9
3   3",
        "11"
    );
    solution!(p1, p1_solution, "2769675");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "3   4
4   3
2   5
1   3
3   9
3   3",
        "31"
    );
    solution!(p2, p2_solution, "24643097");
}
