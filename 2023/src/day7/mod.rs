pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 7: Camel Cards",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod data;
mod parse;

use data::{Card, Hand};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let (_, mut hands) = parse::parse_hands(input.as_ref()).expect("valid input");

    // Part 1
    hands.sort_unstable_by(Hand::cmp_cards);
    let part1: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, v)| (i as u32 + 1) * v.bet)
        .sum();

    // Part 2
    hands.sort_unstable_by(Hand::cmp_cards_joker);
    let part2: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, v)| (i as u32 + 1) * v.bet)
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
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        "6440"
    );
    solution!(p1, p1_solution, "251927063");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        "5905"
    );
    example!(
        p2,
        p2_example_2,
        "5JJ52 28
JKKK2 10",
        "66"
    );
    solution!(p2, p2_solution, "255632664");
}
