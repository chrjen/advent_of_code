pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 4: Scratchcards",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod parse;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    // Part 1
    let part1: u32 = input
        .lines()
        .map(|line| {
            let (_, (win_num, num)) = parse::parse_card(line).unwrap();
            let hits = num.intersection(&win_num).count();
            if hits == 0 {
                0
            } else {
                2_u32.pow((hits - 1) as u32)
            }
        })
        .sum();

    // Part 2s
    let mut num_copies = Vec::new();
    let part2: usize = input
        .lines()
        .map(|line| {
            let (_, (win_num, num)) = parse::parse_card(line).unwrap();
            let hits = num.intersection(&win_num).count();

            let count = num_copies.len() + 1;

            // Subtract one from all and remove zeroes so
            // that the length of num_copies stay accurate.
            num_copies = num_copies
                .drain(..)
                .filter_map(|mut v| {
                    v -= 1;
                    if v == 0 {
                        None
                    } else {
                        Some(v)
                    }
                })
                .collect();

            if hits != 0 {
                for _ in 0..count {
                    num_copies.push(hits);
                }
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
