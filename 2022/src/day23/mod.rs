use std::collections::{HashMap, HashSet};

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 23: Unstable Diffusion",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

#[derive(Debug, Clone, Copy)]
enum Directions {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Directions {
    const fn neighbours((x, y): (i32, i32)) -> [(i32, i32); 8] {
        [
            (x, y - 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
            (x, y + 1),
            (x - 1, y + 1),
            (x - 1, y),
            (x - 1, y - 1),
        ]
    }

    const fn to_idx(self) -> usize {
        match self {
            Directions::North => 0,
            Directions::NorthEast => 1,
            Directions::East => 2,
            Directions::SouthEast => 3,
            Directions::South => 4,
            Directions::SouthWest => 5,
            Directions::West => 6,
            Directions::NorthWest => 7,
        }
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let mut elves: HashSet<(i32, i32)> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().flat_map(move |(col, c)| match c {
                '#' => Some((col.try_into().unwrap(), row.try_into().unwrap())),
                '.' => None,
                _ => panic!("illegal character '{c}' in input"),
            })
        })
        .collect();

    // Proposed moves is a map with desired location as key and a tuple of
    // original location and a bool as value. The bool is set to false if any
    // other elf also wants to move to that same location. If the bool is false,
    // then the original location should not be used.
    let mut proposed_moves: HashMap<(i32, i32), ((i32, i32), bool)> = HashMap::new();
    let mut round = 0;
    let mut part1 = 0;
    loop {
        for elf in elves.iter() {
            let &(x, y) = elf;

            let neighbours = Directions::neighbours((x, y));
            let is_empty: [bool; 8] = neighbours.map(|coord| !elves.contains(&coord));

            if is_empty.iter().all(|&b| b) {
                continue;
            }

            for i in 0..4 {
                use Directions::*;

                let (left, forward, right) = match (round + i) % 4 {
                    0 => (NorthWest.to_idx(), North.to_idx(), NorthEast.to_idx()), // North.
                    1 => (SouthWest.to_idx(), South.to_idx(), SouthEast.to_idx()), // South.
                    2 => (NorthWest.to_idx(), West.to_idx(), SouthWest.to_idx()),  // West.
                    3 => (NorthEast.to_idx(), East.to_idx(), SouthEast.to_idx()),  // East.
                    _ => unreachable!(),
                };

                if is_empty[left] && is_empty[forward] && is_empty[right] {
                    let next_move = neighbours[forward];

                    match proposed_moves.get_mut(&next_move) {
                        Some(((_, _), b)) => *b = false,
                        None => {
                            proposed_moves.insert(next_move, ((x, y), true));
                        }
                    }

                    break;
                }
            }
        }

        round += 1;

        if proposed_moves.is_empty() {
            break;
        }

        for proposal in proposed_moves.drain().filter(|&(_, (_, b))| b) {
            let (to, (from, _)) = proposal;
            elves.remove(&from);
            elves.insert(to);
        }

        if round == 10 {
            let min_x = elves.iter().map(|&(x, _)| x).min().unwrap();
            let min_y = elves.iter().map(|&(_, y)| y).min().unwrap();
            let max_x = elves.iter().map(|&(x, _)| x).max().unwrap();
            let max_y = elves.iter().map(|&(_, y)| y).max().unwrap();

            part1 = (max_x - min_x + 1) as usize * (max_y - min_y + 1) as usize - elves.len();
        }
    }

    // // Uncomment to print the final state.
    // let min_x = elves.iter().map(|&(x, _)| x).min().unwrap();
    // let min_y = elves.iter().map(|&(_, y)| y).min().unwrap();
    // let max_x = elves.iter().map(|&(x, _)| x).max().unwrap();
    // let max_y = elves.iter().map(|&(_, y)| y).max().unwrap();

    // println!();
    // for y in min_y..=max_y {
    //     for x in min_x..=max_x {
    //         if elves.contains(&(x, y)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    (part1.to_string(), round.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..",
        "110"
    );
    solution!(p1, p1_solution, "4082", ignore = "takes too long");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..",
        "20"
    );
    solution!(p2, p2_solution, "1065", ignore = "takes too long");
}
