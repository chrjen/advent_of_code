use regex::Regex;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 2: Rock Paper Scissors",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let reg = Regex::new(r"(A|B|C)\s+(X|Y|Z)").unwrap();

    let (part1, part2): (u32, u32) = input
        .lines()
        .map(|line| {
            let caps = reg.captures(line).unwrap();
            let (left, right) = (&caps[1], &caps[2]);

            #[allow(clippy::identity_op)]
            match (left, right) {
                // For each pairing, calculate the total score.
                // Total score is returned as a tuple consisting of part 1 and part 2.
                // Part 1 total score is on the left, part 2 on the right.
                ("A", "X") => (3 + 1, 0 + 3),
                ("B", "X") => (0 + 1, 0 + 1),
                ("C", "X") => (6 + 1, 0 + 2),
                ("A", "Z") => (0 + 3, 6 + 2),
                ("B", "Z") => (6 + 3, 6 + 3),
                ("C", "Z") => (3 + 3, 6 + 1),
                ("A", "Y") => (6 + 2, 3 + 1),
                ("B", "Y") => (3 + 2, 3 + 2),
                ("C", "Y") => (0 + 2, 3 + 3),
                (_, _) => panic!("unknown strategy, got '{left} {right}'"),
            }
        })
        .reduce(|acc, result| (acc.0 + result.0, acc.1 + result.1))
        .unwrap();

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
        r"A Y
B X
C Z",
        "15"
    );
    solution!(p1, p1_solution, "10994");

    // Part 2
    example!(
        p2,
        p2_example_1,
        r"A Y
B X
C Z",
        "12"
    );
    solution!(p2, p2_solution, "12526");
}
