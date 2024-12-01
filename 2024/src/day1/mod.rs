pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 1: Historian Hysteria",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod parse;

use std::collections::HashMap;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let (_, (mut left_list, mut right_list)) =
        parse::parse_lists(input).expect("input should be valid");

    left_list.sort_unstable();
    right_list.sort_unstable();

    // Part 1
    let part1: u32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(lhs, rhs)| lhs.abs_diff(*rhs))
        .sum();

    // Part 2
    let freq_map = right_list
        .into_iter()
        .fold(HashMap::<u32, u32>::new(), |mut acc, value| {
            *acc.entry(value).or_insert(0) += 1;
            acc
        });

    let part2: u32 = left_list
        .iter()
        .map(|value| value * freq_map.get(value).unwrap_or(&0))
        .sum();

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
