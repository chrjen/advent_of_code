pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 14: Parabolic Reflector Dish",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

use std::{collections::HashMap, ops::Range};

mod data;
mod parse;

use data::{Platform, Rock};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let platform_initial = parse::parse_input(&input);

    // Part 1
    let mut platform = platform_initial.clone();
    platform.fall_north();
    let part1 = platform.load_north();

    // Part 2
    let mut platform = platform_initial;
    let mut history: HashMap<u64, usize> = HashMap::new();
    let mut repeat_range: Range<usize> = 0..0;

    for cycle in 1..=1_000_000_000 {
        platform.fall_north();
        platform.fall_west();
        platform.fall_south();
        let hash = platform.fall_east();

        if let Some(prev_cycle) = history.get(&hash) {
            // Optimisation: when the cycles starts to repeat, we quit early.
            repeat_range = *prev_cycle..cycle;
            break;
        }

        history.insert(hash, cycle);
    }

    // Skip all the repeating cycles and only do the remaining ones.
    let remaining_cycles = (1_000_000_000 - repeat_range.end) % repeat_range.len();
    for _ in 0..remaining_cycles {
        platform.fall_north();
        platform.fall_west();
        platform.fall_south();
        platform.fall_east();
    }
    let part2 = platform.load_north();

    // println!("\n1000000000th cycle");
    // _print_platform(&platform);
    // println!();

    (part1.to_string(), part2.to_string())
}

fn _print_platform(platform: &Platform) {
    for y in platform.x_bound.clone() {
        for x in platform.y_bound.clone() {
            let c = match platform.rocks.get(&(x, y)) {
                Some(Rock::Cubed) => '#',
                Some(Rock::Round) => 'O',
                None => '.',
            };
            print!("{c}");
        }
        println!()
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
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        "136"
    );
    solution!(p1, p1_solution, "108826");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        "64"
    );
    solution!(p2, p2_solution, "99291");
}
