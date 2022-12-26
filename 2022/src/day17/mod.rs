mod data;

use std::collections::HashMap;

use data::*;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 17: Pyroclastic Flow",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

const PART1_NUM_ITERATIONS: usize = 2022;
const PART2_NUM_ITERATIONS: usize = 1_000_000_000_000;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let wind = input.as_ref().chars().filter_map(|c| match c {
        '>' => Some(Wind::Right),
        '<' => Some(Wind::Left),
        _ if c.is_ascii_whitespace() => None,
        _ => panic!("illegal character '{}' found", c),
    });

    let mut world = World::new(wind, 0, 6, 0);
    let mut states = HashMap::new();

    // Part 1.
    for i in 0..PART1_NUM_ITERATIONS {
        let state = world.drop_rock();

        states.insert(state, (i, world.top())); // For part2.
    }
    let part1 = world.top();

    // Part 2.
    let mut shortcut_height = 0;
    for i in PART1_NUM_ITERATIONS..PART2_NUM_ITERATIONS {
        let state = world.drop_rock();

        // Record new state and check if it has been seen before.
        // If it has been seen then that indicates a likely cycle and we
        // can use that as a shortcut to the solution.
        if let Some((old_i, old_top)) = states.insert(state, (i, world.top())) {
            let iter_left = PART2_NUM_ITERATIONS - (i + 1);
            let cycle_len = i - old_i;
            let n_cycles_left = iter_left / cycle_len;
            shortcut_height = n_cycles_left as i64 * (world.top() - old_top);

            for _ in (i + n_cycles_left * cycle_len + 1)..PART2_NUM_ITERATIONS {
                world.drop_rock();
            }

            break;
        }
    }
    let part2 = world.top() + shortcut_height;

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
        ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>",
        "3068"
    );
    solution!(p1, p1_solution, "3200");

    // Part 2
    example!(
        p2,
        p2_example_1,
        ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>",
        "1514285714288"
    );
    solution!(p2, p2_solution, "1584927536247");
}
