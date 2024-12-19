pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 19: Linen Layout",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod parse;

use std::collections::{HashMap, HashSet};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let (patterns, designs) = parse::parse_input(input).expect("input should be valid");

    let part1 = designs
        .iter()
        .filter(|design| is_possible_design(&patterns, design))
        .count();

    let mut cache = HashMap::new();
    let part2 = designs
        .iter()
        .map(|design| count_possible_arrangements(&mut cache, &patterns, design))
        .sum::<usize>();

    (part1.to_string(), part2.to_string())
}

/// Checks if a design can be construction from the given patterns or not.
///
/// This function uses DFS to quickly search for a possible solution and returns
/// as soon as a solution is found.
fn is_possible_design(patterns: &[&str], design: &str) -> bool {
    let mut next: Vec<&str> = Vec::new();
    let mut visited: HashSet<&str> = HashSet::new();
    next.push(design);

    while let Some(current) = next.pop() {
        if current.is_empty() {
            return true;
        }

        if visited.contains(current) {
            continue;
        }

        visited.insert(current);

        for pattern in patterns {
            if let Some(leftover) = current.strip_prefix(pattern) {
                next.push(leftover);
            }
        }
    }

    false
}

/// Counts the number of possible arrangements of the patterns produce the given design.
///
/// This function makes use of dynamic programming principles and caches the results of previous
/// calculations to speed up the computation. The cache is passed in as a function argument so it
/// can be reused by later function calls or otherwise analysed.
fn count_possible_arrangements<'a>(
    cache: &mut HashMap<&'a str, usize>,
    patterns: &[&str],
    design: &'a str,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(count) = cache.get(design) {
        return *count;
    }

    let mut count = 0;

    for pattern in patterns {
        if let Some(leftover) = design.strip_prefix(pattern) {
            count += count_possible_arrangements(cache, patterns, leftover);
        }
    }
    cache.insert(design, count);

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
        "6"
    );
    solution!(p1, p1_solution, "319");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
        "16"
    );
    solution!(p2, p2_solution, "692575723305545");
}
