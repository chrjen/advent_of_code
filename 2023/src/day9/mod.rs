pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 9: Mirage Maintenance",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

use std::mem;

use itertools::Itertools;

mod parse;

#[derive(Debug)]
struct Sequence {
    differences: Vec<Vec<i64>>,
}

impl Sequence {
    /// Creates a `Sequence` and returns it. It also calculate all difference
    /// sub-sequences that is used on other calculations.
    ///
    /// For a sequence like `1, 3, 6, 10, 15, 21` you have the sequence of
    /// the differences between any pair of elements in order. For the example
    /// we get the sub-sequence `2, 3, 4, 5, 6` which in turn give the
    /// sub-sequence `1, 1, 1, 1`, so in total two sub-sequences here.
    fn new(mut initial: Vec<i64>) -> Self {
        let mut differences = vec![initial.clone()];
        let mut next = Vec::new();
        loop {
            for (n0, n1) in initial.drain(..).tuple_windows() {
                next.push(n1 - n0);
            }
            if next.is_empty() || next.iter().copied().all(|v| v == 0) {
                break;
            }
            differences.push(next.clone());
            mem::swap(&mut initial, &mut next);
        }
        Sequence { differences }
    }

    fn predict_next(&self) -> i64 {
        self.differences
            .iter()
            .rev()
            .map(|s| s.last().copied().unwrap())
            .sum()
    }

    fn predict_previous(&self) -> i64 {
        self.differences
            .iter()
            .rev()
            .fold(0, |acc, s| s.first().copied().unwrap() - acc)
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let sequences: Vec<Sequence> = parse::parse_sequences(&input).expect("valid_input").1;

    let part1: i64 = sequences.iter().map(Sequence::predict_next).sum();
    let part2: i64 = sequences.iter().map(Sequence::predict_previous).sum();

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
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        "114"
    );
    solution!(p1, p1_solution, "1861775706");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        "2"
    );
    solution!(p2, p2_solution, "1082");
}
