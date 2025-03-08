use std::collections::HashSet;

use regex::Regex;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 9: Rope Bridge",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

use Direction::*;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn to_offset(self) -> (i32, i32) {
        match self {
            Left => (-1, 0),
            Right => (1, 0),
            Up => (0, 1),
            Down => (0, -1),
        }
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let reg = Regex::new(r"(\w)\s+(\d+)").unwrap();

    let moves: Vec<(Direction, u32)> = input
        .lines()
        .map(|line| {
            let cap = reg
                .captures(line)
                .unwrap_or_else(|| panic!("failed to parse line '{}'", line));
            (
                match &cap[1] {
                    "R" => Right,
                    "L" => Left,
                    "U" => Up,
                    "D" => Down,
                    x => panic!("unknown direction '{}'", x),
                },
                cap[2]
                    .parse::<u32>()
                    .unwrap_or_else(|_| panic!("failed to parse number '{}'", &cap[2])),
            )
        })
        .collect();

    fn update_rope(direction: (i32, i32), rope: &mut [(i32, i32)]) {
        // Update head.
        let (dx, dy) = direction;
        let ((head_x, head_y), rope) = rope.split_first_mut().unwrap();

        *head_x += dx;
        *head_y += dy;

        if rope.is_empty() {
            return;
        }

        // Update tail.
        let (tail_x, tail_y) = rope.first_mut().unwrap();
        let (mut diff_x, mut diff_y) = (*head_x - *tail_x, *head_y - *tail_y);

        if diff_x.abs() <= 1 && diff_y.abs() <= 1 {
            return;
        }

        if diff_x.abs() > 1 {
            diff_x >>= 1; // Maps 2 to 1 and -2 to -1.
        }

        if diff_y.abs() > 1 {
            diff_y >>= 1; // Maps 2 to 1 and -2 to -1.
        }

        update_rope((diff_x, diff_y), rope)
    }

    fn process_moves(rope: &mut [(i32, i32)], moves: &[(Direction, u32)]) -> usize {
        let mut tail_pos: HashSet<(i32, i32)> = HashSet::new();

        for &(direction, distance) in moves.iter() {
            for _ in 0..distance {
                update_rope(direction.to_offset(), rope);
                tail_pos.insert(*rope.last().unwrap());
            }
        }

        tail_pos.len()
    }

    // Part 1.
    let mut rope = [(0, 0); 2];
    let part1 = process_moves(&mut rope, &moves);

    // Part 2.
    let mut rope = [(0, 0); 10];
    let part2 = process_moves(&mut rope, &moves);

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
        "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        "13"
    );
    solution!(p1, p1_solution, "6256");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        "1"
    );
    example!(
        p2,
        p2_example_2,
        "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
        "36"
    );
    solution!(p2, p2_solution, "2665");
}
