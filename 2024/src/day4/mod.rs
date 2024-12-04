pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 4: Ceres Search",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

use std::collections::HashMap;

use itertools::Itertools;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let mut letters: HashMap<(i32, i32), char> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            letters.insert(((x + 1) as i32, (y + 1) as i32), c);
        }
    }
    let max_x = letters.keys().map(|(x, _)| x).max().unwrap_or(&1);
    let max_y = letters.keys().map(|(_, y)| y).max().unwrap_or(&1);

    // Part 1
    let mut part1: u32 = 0;
    let target_word = "XMAS";
    let directions: &[(i32, i32)] = &[
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for (y, x) in (1..=*max_y).cartesian_product(1..=*max_x) {
        for (dir_x, dir_y) in directions {
            (0..)
                .map(|i| letters.get(&(x + i * dir_x, y + i * dir_y)))
                .zip(target_word.chars())
                .all(|(letter, target)| letter.copied().is_some_and(|l| l == target))
                .then(|| part1 += 1);
        }
    }

    // Part 2
    let mut part2: u32 = 0;
    let target_word = "MAS";
    let directions: &[(i32, i32)] = &[(-1, -1), (1, -1), (-1, 1), (1, 1)];

    for (y, x) in (1..=*max_y).cartesian_product(1..=*max_x) {
        let mut count = 0;
        for (dir_x, dir_y) in directions {
            (-1..)
                .map(|i| letters.get(&(x + i * dir_x, y + i * dir_y)))
                .zip(target_word.chars())
                .all(|(letter, target)| letter.copied().is_some_and(|l| l == target))
                .then(|| count += 1);
        }
        if count == 2 {
            part2 += 1;
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
        "..X...
.SAMX.
.A..A.
XMAS.S
.X....",
        "4"
    );
    example!(
        p1,
        p1_example_2,
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        "18"
    );
    solution!(p1, p1_solution, "2447");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "M.S
.A.
M.S
",
        "1"
    );
    example!(
        p2,
        p2_example_2,
        ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........",
        "9"
    );
    solution!(p2, p2_solution, "1868");
}
