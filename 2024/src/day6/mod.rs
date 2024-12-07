pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 6: Guard Gallivant",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }

    fn apply(&self, v: (i32, i32)) -> (i32, i32) {
        let offset = self.offset();
        (v.0 + offset.0, v.1 + offset.1)
    }

    fn rot_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let mut map: HashMap<(i32, i32), Tile> = HashMap::new();

    let (mut start_x, mut start_y): (i32, i32) = (0, 0);

    for (y, line) in (1..).zip(input.lines()) {
        for (x, c) in (1..).zip(line.chars()) {
            match c {
                '#' => {
                    map.insert((x, y), Tile::Wall);
                }
                '.' | 'O' => { /* Do nothing */ }
                '^' => {
                    start_x = x;
                    start_y = y;
                }
                _ => {
                    panic!("Found unknown character '{c}' in input");
                }
            }
        }
    }
    let max_x = map.keys().map(|(x, _)| x).max().unwrap_or(&1) + 1;
    let max_y = map.keys().map(|(_, y)| y).max().unwrap_or(&1) + 1;

    // Add boarders around the map help tell when we leave.
    for (x, y) in (0..=max_x)
        .map(|x| (x, 0))
        .chain((0..=max_x).map(|x| (x, max_y)))
        .chain((0..=max_y).map(|y| (0, y)))
        .chain((0..=max_y).map(|y| (max_x, y)))
    {
        map.insert((x, y), Tile::Exit);
    }

    // Part 1
    let mut pos = (start_x, start_y);
    let mut dir = Direction::North;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(pos);

    loop {
        let new_pos = dir.apply(pos);

        match map.get(&new_pos) {
            Some(Tile::Wall) => {
                dir = dir.rot_right();
            }
            Some(Tile::Exit) => break,
            None => {
                pos = new_pos;
                visited.insert(pos);
            }
        }
    }

    let part1 = visited.len();

    // Part 2
    let mut pos = (start_x, start_y);
    let mut dir = Direction::North;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut visited_dir: HashSet<((i32, i32), Direction)> = HashSet::new();
    let mut loop_point: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(pos);
    visited_dir.insert((pos, dir));

    loop {
        let new_pos = dir.apply(pos);

        match map.get(&new_pos) {
            Some(Tile::Wall) => {
                dir = dir.rot_right();
            }
            Some(Tile::Exit) => break,
            None => {
                if !visited.contains(&new_pos) {
                    // Try placing an obstruction in front.
                    let obstruction = new_pos;
                    let mut pos = pos;
                    let mut dir = dir.rot_right();
                    let mut visited: HashSet<(i32, i32)> = visited.clone();
                    let mut visited_dir: HashSet<((i32, i32), Direction)> = visited_dir.clone();

                    loop {
                        let new_pos = dir.apply(pos);

                        if visited_dir.contains(&(new_pos, dir)) {
                            // We are looping.
                            loop_point.insert(obstruction);
                            break;
                        }

                        match map.get(&new_pos) {
                            Some(Tile::Wall) => {
                                dir = dir.rot_right();
                            }
                            Some(Tile::Exit) => break,
                            None if new_pos == obstruction => {
                                dir = dir.rot_right();
                            }
                            None => {
                                pos = new_pos;
                                visited.insert(pos);
                                visited_dir.insert((pos, dir));
                            }
                        }
                    }
                }

                pos = new_pos;
                visited.insert(pos);
                visited_dir.insert((pos, dir));
            }
        }
    }

    loop_point.remove(&(start_x, start_y));
    let part2 = loop_point.len();

    println!();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if loop_point.contains(&(x, y)) {
                print!("O");
            } else {
                match map.get(&(x, y)) {
                    Some(Tile::Wall) => print!("#"),
                    Some(Tile::Exit) => print!("*"),
                    None => {
                        if loop_point.contains(&(x, y)) {
                            print!("O");
                        } else if visited.contains(&(x, y)) {
                            print!("x");
                        } else {
                            print!(".");
                        }
                    }
                }
            }
        }
        println!();
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
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        "41"
    );
    solution!(p1, p1_solution, "5153");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#.O^.....
......OO#.
#O.O......
......#O..",
        "6"
    );
    example!(
        p2,
        p2_example_2,
        ".#....
.O.O.#
#..#..
..#...
O^...#
....#.",
        "3"
    );
    example!(
        p2,
        p2_example_3,
        ".##..
....#
O....
.^.#.
.....",
        "1"
    );
    example!(
        p2,
        p2_example_4,
        "..#.....
.#....#.
.OO..O..
.....#..
#^......
.#......
....#...",
        "3"
    );
    solution!(p2, p2_solution, "1711");
}
