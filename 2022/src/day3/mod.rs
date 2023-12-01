use std::collections::HashSet;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 3: Rucksack Reorganization",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    if !input.is_ascii() {
        panic!("input is not ascii");
    }

    fn to_priority(c: char) -> u32 {
        match c {
            c if c.is_lowercase() => (c as u32 - 'a' as u32) + 1,
            c if c.is_uppercase() => (c as u32 - 'A' as u32) + 27,
            c => panic!("got illegal character '{}'", c),
        }
    }

    // Part 1.
    let part1: u32 = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);

            let set: HashSet<char> = left.chars().collect();

            right
                .chars()
                .find(move |c| set.contains(c))
                .map_or_else(|| panic!("didn't find a solution"), to_priority)
        })
        .sum();

    // Part 2.
    let mut lines = input.lines();
    let mut total: u32 = 0;
    let part2 = loop {
        let Some(first) = lines.next() else {
            break total;
        };
        let second = lines.next().unwrap();
        let third = lines.next().unwrap();

        // Fill first set with all characters from first rucksack.
        let set_1: HashSet<char> = first.chars().collect();

        // Find all shared letters from first and second rucksack and put them in `set_2`.
        let set_2: HashSet<char> = second.chars().filter(move |c| set_1.contains(c)).collect();

        // Find all shared letters from `set_2` and third rucksack.
        total += third
            .chars()
            .find(move |c| set_2.contains(c))
            .map_or_else(|| panic!("unable to find badge"), to_priority);
    };

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
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        "157"
    );
    solution!(p1, p1_solution, "8176");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg",
        "18"
    );
    example!(
        p2,
        p2_example_2,
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        "52"
    );
    example!(
        p2,
        p2_example_3,
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        "70"
    );
    solution!(p2, p2_solution, "2689");
}
