pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 7: Bridge Repair",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod parse;

use std::ops::{Add, Mul};

use itertools::Itertools;

fn concat_i64(lhs: i64, rhs: i64) -> i64 {
    let exp = if rhs == 0 { 1 } else { rhs.ilog10() + 1 };
    lhs * 10_i64.pow(exp) + rhs
}

/// Brute forces all permutations until a matching permutation is found or all
/// possible permutations are exhausted.
fn brute_force(
    equations: &[(i64, Vec<i64>)],
    ops: impl Iterator<Item = fn(i64, i64) -> i64> + Clone,
) -> i64 {
    let mut sum: i64 = 0;
    for (target, args) in equations.iter() {
        // Permutations with replacement.
        let operation_combinations =
            itertools::repeat_n(ops.clone(), args.len() - 1).multi_cartesian_product();

        let mut args_iter = args.iter();
        let initial = args_iter.next().expect("should be at least one argument");
        for combination in operation_combinations {
            let result = args_iter
                .clone()
                .zip(combination.into_iter())
                .fold(*initial, |acc, (arg, op)| op(acc, *arg));

            if *target == result {
                sum += target;
                break;
            }
        }
    }

    sum
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let (_, equations) = parse::parse_reports(input).expect("Input should be valid");

    // Part 1
    let part1: i64 = brute_force(&equations, [<i64 as Add>::add, Mul::<i64>::mul].into_iter());

    // Part 2
    let part2: i64 = brute_force(
        &equations,
        [<i64 as Add>::add, Mul::<i64>::mul, concat_i64].into_iter(),
    );

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
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        "3749"
    );
    solution!(p1, p1_solution, "2314935962622");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        "11387"
    );
    solution!(p2, p2_solution, "401477450831495");

    use rstest::rstest;

    #[rstest]
    #[case(1, 1, 11)]
    #[case(1, 0, 10)]
    #[case(0, 1, 1)]
    #[case(0, 200, 200)]
    #[case(200, 999, 200999)]
    #[case(100, 1, 1001)]
    #[case(12, 789, 12789)]
    fn p2_concat_i64(#[case] lhs: i64, #[case] rhs: i64, #[case] expected: i64) {
        assert_eq!(concat_i64(lhs, rhs), expected);
    }
}
