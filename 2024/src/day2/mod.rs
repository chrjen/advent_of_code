pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 2: Red-Nosed Reports",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod parse;

use itertools::Itertools;

fn is_safe(report: &[i32]) -> bool {
    match report[0].cmp(&report[1]) {
        std::cmp::Ordering::Less => report
            .iter()
            .tuple_windows()
            .all(|(&a, &b)| matches!(b - a, 1..=3)),
        std::cmp::Ordering::Greater => report
            .iter()
            .tuple_windows()
            .all(|(&a, &b)| matches!(a - b, 1..=3)),
        std::cmp::Ordering::Equal => false,
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let (_, reports) = parse::parse_reports(input).expect("input should be valid");

    // Part 1
    let part1: i32 = reports
        .iter()
        .map(Vec::as_slice)
        .map(is_safe)
        .map(i32::from)
        .sum();

    // Part 2
    let mut part2 = 0;
    for report in reports {
        for i in 0..report.len() {
            let mut r = report.clone();
            r.remove(i);
            let safe = is_safe(&r);
            if safe {
                part2 += 1;
                break;
            }
        }
    }

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
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        "2"
    );
    solution!(p1, p1_solution, "242");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        "4"
    );
    solution!(p2, p2_solution, "311");
}
