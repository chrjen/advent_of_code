pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 4: Ceres Search",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Letter {
    X,
    M,
    A,
    S,
}

impl Letter {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'X' => Some(Letter::X),
            'M' => Some(Letter::M),
            'A' => Some(Letter::A),
            'S' => Some(Letter::S),
            _ => None,
        }
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let mut letters: HashMap<(i32, i32), Letter> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if let Some(letter) = Letter::from_char(c) {
                letters.insert(((x + 1) as i32, (y + 1) as i32), letter);
            }
        }
    }
    let max_x = letters.keys().map(|(x, _)| x).max().unwrap_or(&1);
    let max_y = letters.keys().map(|(_, y)| y).max().unwrap_or(&1);

    // Part 1
    let mut part1: u32 = 0;
    let offsets: &[(i32, i32)] = &[
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for y in 1..=*max_y {
        for x in 1..=*max_x {
            if !matches!(letters.get(&(x, y)), Some(Letter::X)) {
                continue;
            }
            // "X" found, check each direction for the word "MAS".
            for (off_x, off_y) in offsets {
                if matches!(letters.get(&(x + off_x, y + off_y)), Some(Letter::M))
                    && matches!(
                        letters.get(&(x + 2 * off_x, y + 2 * off_y)),
                        Some(Letter::A)
                    )
                    && matches!(
                        letters.get(&(x + 3 * off_x, y + 3 * off_y)),
                        Some(Letter::S)
                    )
                {
                    part1 += 1;
                }
            }
        }
    }

    // Part 2
    let mut part2: u32 = 0;
    let offsets: &[(i32, i32)] = &[(-1, -1), (1, -1), (-1, 1), (1, 1)];

    for y in 1..=*max_y {
        for x in 1..=*max_x {
            if !matches!(letters.get(&(x, y)), Some(Letter::A)) {
                continue;
            }
            // "A" found, check each diagonal for if it spell "MAS".
            let mut count = 0;
            for (off_x, off_y) in offsets {
                if matches!(letters.get(&(x + off_x, y + off_y)), Some(Letter::M))
                    && matches!(letters.get(&(x - off_x, y - off_y)), Some(Letter::S))
                {
                    count += 1;
                }
            }
            if count == 2 {
                part2 += 1;
            }
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
