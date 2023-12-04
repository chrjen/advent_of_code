use std::collections::HashMap;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 4: Scratchcards",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod parse;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let cards: Box<_> = input
        .lines()
        .map(|line| parse::parse_card(line).unwrap().1)
        .collect();

    // Part 1
    let part1: u32 = cards
        .iter()
        .map(|(win_num, num)| {
            let points = num.intersection(win_num).count();
            points.checked_sub(1).map_or(0, |v| 2_u32.pow(v as u32))
        })
        .sum();

    // Part 2
    let mut num_copies: HashMap<usize, u32> = HashMap::new();
    let part2: u32 = cards
        .iter()
        .enumerate()
        .map(|(i, (win_num, num))| {
            let points = num.intersection(win_num).count();
            let count = num_copies.get(&i).copied().unwrap_or(1);
            for j in 1..=points {
                num_copies
                    .entry(i + j)
                    .and_modify(|v| *v += count)
                    .or_insert(count + 1);
            }

            count
        })
        .sum();

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
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        "13"
    );
    solution!(p1, p1_solution, "32609");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        "30"
    );
    solution!(p2, p2_solution, "14624680");
}
