pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 5: Print Queue",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod data;
mod parse;

use data::Page;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();
    let (_, (rules, updates)) = parse::parse_print_queue(input).expect("valid input");

    let mut updates: Vec<Vec<Page>> = updates
        .into_iter()
        .map(|v| {
            v.into_iter()
                .map(|number| Page::new(&rules, number))
                .collect()
        })
        .collect();

    // Part 1
    let mut part1 = 0;
    for update in updates.iter() {
        if update.is_sorted() {
            part1 += update[update.len() / 2].page_number;
        }
    }

    // Part 2
    let mut part2 = 0;
    for update in updates.iter_mut() {
        if !update.is_sorted() {
            update.sort_unstable();
            part2 += update[update.len() / 2].page_number;
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
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        "143"
    );
    solution!(p1, p1_solution, "6505");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        "123"
    );
    solution!(p2, p2_solution, "6897");
}
