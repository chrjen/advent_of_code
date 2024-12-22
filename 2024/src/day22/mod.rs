pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 22: Monkey Market",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

use std::{collections::HashMap, iter};

use fxhash::{FxBuildHasher, FxHashMap};
use itertools::Itertools;
use rayon::prelude::*;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let seeds: Vec<u32> = input.lines().flat_map(str::parse).collect();

    // Part 1
    let part1: usize = seeds
        .iter()
        .copied()
        .map(|seed| nth_secret(seed, 2000) as usize)
        .sum();

    // Part 2
    let price_changes = |seed| {
        iter::once(seed)
            .chain(SecretIter(seed).take(2000))
            .tuple_windows()
            .map(|(prev, current)| {
                let prev_price = prev % 10;
                let current_price = current % 10;
                let price_change = (current_price as i32 - prev_price as i32) as i8;
                // println!("{}: {} ({})", current, current_price, price_change);
                (current_price, price_change)
            })
    };

    // For each seed we calculate the price for each set of four changes, making
    // sure we only consider the first occurrence of those changes. At the end
    // we should have a map of each set of four changes and how many bananas it
    // would get us. Then we just need to find the maximum.
    let seeds_history: HashMap<(i8, i8, i8, i8), u32, FxBuildHasher> = seeds
        .par_iter()
        .copied()
        .map(|seed| {
            let mut seed_history: HashMap<(i8, i8, i8, i8), u32, FxBuildHasher> =
                FxHashMap::default();

            for (p1, p2, p3, p4) in price_changes(seed).tuple_windows() {
                let changes = (p1.1, p2.1, p3.1, p4.1);
                let current_price = p4.0;
                seed_history.entry(changes).or_insert(current_price);
            }

            seed_history
        })
        .reduce(FxHashMap::default, |mut acc, history| {
            for (key, value) in history.into_iter() {
                acc.entry(key).and_modify(|v| *v += value).or_insert(value);
            }
            acc
        });

    let part2 = seeds_history.values().max().unwrap_or(&0);

    (part1.to_string(), part2.to_string())
}

/// Calculates the nth-secret number given the initial seed value of the
/// pseudorandom sequence.
fn nth_secret(mut seed: u32, n: usize) -> u32 {
    for _ in 0..n {
        seed = (seed ^ (seed << 6)) & 0x00ffffff;
        seed = (seed ^ (seed >> 5)) & 0x00ffffff;
        seed = (seed ^ (seed << 11)) & 0x00ffffff;
    }
    seed
}

#[derive(Debug, Clone, Copy)]
struct SecretIter(u32);

impl Iterator for SecretIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = nth_secret(self.0, 1);
        Some(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "1
10
100
2024",
        "37327623"
    );
    solution!(p1, p1_solution, "14119253575");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "1
2
3
2024
",
        "23"
    );
    solution!(p2, p2_solution, "1600");
}
