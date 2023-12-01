use std::fmt::Debug;

use self::parse::{parse_alphanumeric_digit0, parse_digit0};

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 1: Trebuchet?!",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod parse;

/// Goes line by line and parses all number using the provided parser and then
/// finds the first and last digit on each line. It then sums up all of these
/// numbers for a final sum of all lines.
fn sum_lines<'a, F, E>(mut parser: F, input: &'a str) -> u32
where
    F: FnMut(&'a str) -> Result<(&'a str, Vec<u32>), E>,
    E: Debug,
{
    let mut sum: u32 = 0u32;
    for (i, line) in input.lines().enumerate() {
        let (_, numbers) = parser(line).expect("should parse successfully");
        let first = numbers
            .first()
            .unwrap_or_else(|| panic!("line {} should contain at least one digit", i + 1));
        let last = numbers.last().unwrap();
        sum += first * 10 + last;
    }
    sum
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    (
        sum_lines(parse_digit0, input).to_string(),
        sum_lines(parse_alphanumeric_digit0, input).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        "142"
    );
    solution!(p1, p1_solution, "54968");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "two1nine
eightwo5three
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        "281"
    );
    solution!(p2, p2_solution, "54094");
}
