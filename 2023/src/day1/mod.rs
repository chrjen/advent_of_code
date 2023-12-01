pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 1: Trebuchet?!",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod parse;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let mut part1_sum = 0;

    for line in input.lines() {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;

        for ch in line.chars() {
            match (ch.is_numeric(), first) {
                (true, None) => {
                    first = Some(ch);
                    last = Some(ch)
                }
                (true, Some(_)) => last = Some(ch),
                (false, _) => (),
            }
        }

        let first = first.expect("line should have a first digit");
        let last = last.expect("line should have a last digit");

        let s: String = [first, last].iter().collect();
        part1_sum += s.parse::<i32>().expect("should be a number");
    }

    let mut part2_sum = 0;

    for line in input.lines() {
        let numbers = parse::parse_line(line).unwrap();
        let first = numbers.first().expect("line should have a first digit");
        let last = numbers.last().expect("line should have a last digit");
        part2_sum += first * 10 + last;
    }

    (part1_sum.to_string(), part2_sum.to_string())
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
