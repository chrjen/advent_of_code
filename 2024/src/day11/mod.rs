pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 11: Plutonian Pebbles",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

use std::{
    collections::HashMap,
    hash::Hash,
    ops::{AddAssign, SubAssign},
};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let mut stones: HashMap<u64, u64> = HashMap::new();
    for stone in input
        .split(" ")
        .map(|v| v.parse().expect("input should only contain numbers"))
    {
        stones.add_count(stone, 1);
    }

    // Part 1
    for _ in 0..25 {
        blink(&mut stones);
    }
    let part1: u64 = stones.values().sum();

    // Part 2
    for _ in 25..75 {
        blink(&mut stones);
    }
    let part2: u64 = stones.values().sum();

    (part1.to_string(), part2.to_string())
}

trait ChangeCount<T, U> {
    fn add_count(&mut self, stone: T, count: U);
    fn sub_count(&mut self, stone: T, count: U);
}

impl<T, U> ChangeCount<T, U> for HashMap<T, U>
where
    T: Eq + Hash,
    U: Copy + Default + AddAssign<U> + SubAssign<U>,
{
    fn add_count(&mut self, stone: T, count: U) {
        self.entry(stone)
            .and_modify(|v| {
                *v += count;
            })
            .or_insert(count);
    }

    fn sub_count(&mut self, stone: T, count: U) {
        self.entry(stone)
            .and_modify(|v| {
                *v -= count;
            })
            .or_default();
    }
}

fn blink(stones: &mut HashMap<u64, u64>) {
    for (stone, count) in stones
        .iter()
        .filter_map(|(s, count)| (*count > 0).then_some((*s, *count)))
        .collect::<Vec<_>>()
    {
        stones.sub_count(stone, count);

        if stone == 0 {
            stones.add_count(1, count);
            continue;
        }

        let num_digits = stone.ilog10() + 1;
        if num_digits % 2 == 0 {
            let pow10 = 10u64.pow(num_digits / 2);
            let left = stone / pow10;
            let right = stone % pow10;

            stones.add_count(left, count);
            stones.add_count(right, count);
        } else {
            stones.add_count(stone * 2024, count);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(p1, p1_example_1, "125 17", "55312");
    solution!(p1, p1_solution, "193899");

    // Part 2
    solution!(p2, p2_solution, "229682160383225");
}
