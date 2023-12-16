use std::{cmp::Reverse, collections::BinaryHeap};

pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 1: Calorie Counting",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let mut top3 = BinaryHeap::new();
    let mut count = 0;

    for elf in input.split("\n\n") {
        for line in elf.lines() {
            count += line.parse::<i64>().unwrap();
        }

        top3.push(Reverse(count));

        if top3.len() > 3 {
            top3.pop();
        }

        count = 0;
    }

    let sum = top3.iter().map(|x| x.0).sum::<i64>();
    top3.pop();
    top3.pop();

    (top3.peek().unwrap().0.to_string(), sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        "24000"
    );
    solution!(p1, p1_solution, "71506");

    // Part 2
    example!(
        p2,
        p2_example_1,
        r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        "45000"
    );
    solution!(p2, p2_solution, "209603");
}
