mod keypad;

use keypad::*;

pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 2: Bathroom Security",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let keypad_moves: Vec<Vec<KeypadMove>> = input
        .lines()
        .map(|line| line.chars().map(KeypadMove::from_char).collect())
        .collect();

    // Part 1.
    let mut key = KeypadButton::Five;
    let mut passcode = String::new();

    for moves in keypad_moves.iter() {
        key = moves
            .iter()
            .fold(key, |key, &key_move| key.do_move(key_move));
        passcode.push(key.to_char());
    }
    let part1 = passcode;

    // Part 2.
    let mut key = ExtendedKeypadButton::Five;
    let mut passcode = String::new();

    for moves in keypad_moves.iter() {
        key = moves
            .iter()
            .fold(key, |key, &key_move| key.do_move(key_move));
        passcode.push(key.to_char());
    }
    let part2 = passcode;

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "ULL
RRDDD
LURDL
UUUUD",
        "1985"
    );
    solution!(p1, p1_solution, "65556");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "ULL
RRDDD
LURDL
UUUUD",
        "5DB3"
    );
    solution!(p2, p2_solution, "CB779");
}
