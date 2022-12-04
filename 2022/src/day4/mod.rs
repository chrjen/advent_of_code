use regex::Regex;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 4: Camp Cleanup",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let reg = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    let pairs = input.lines().map(|line| {
        let caps = reg.captures(line).unwrap();
        (
            caps[1].parse::<u32>().unwrap()..=caps[2].parse::<u32>().unwrap(),
            caps[3].parse::<u32>().unwrap()..=caps[4].parse::<u32>().unwrap(),
        )
    });

    let part1 = pairs
        .clone()
        .filter(|(first, second)| {
            first.contains(second.start()) && first.contains(second.end())
                || second.contains(first.start()) && second.contains(first.end())
        })
        .count();

    let part2 = pairs
        .filter(|(first, second)| {
            first.contains(second.start())
                || first.contains(second.end())
                || second.contains(first.start())
                || second.contains(first.end())
        })
        .count();

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
        "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
        "2"
    );
    example!(p1, p1_example_2, "3-5,2-4", "0");
    solution!(p1, p1_solution, "456");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
        "4"
    );
    solution!(p2, p2_solution, "808");
}
