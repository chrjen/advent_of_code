pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 10: Hoof It",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let mut map: HashMap<(i32, i32), u32> = HashMap::new();
    let mut trailheads: Vec<(i32, i32)> = Vec::new();
    let offsets = [(-1, 0), (0, -1), (1, 0), (0, 1)].into_iter();

    for (y, line) in (1..).zip(input.lines()) {
        for (x, c) in (1..).zip(line.chars()) {
            match c {
                '0'..='9' => {
                    if let Some(digit) = c.to_digit(10) {
                        map.insert((x, y), digit);
                        if digit == 0 {
                            trailheads.push((x, y));
                        }
                    }
                }
                '.' => { /* Do nothing */ }
                _ => {
                    panic!("Found unknown character '{c}' in input");
                }
            }
        }
    }

    // Part 1
    let mut part1 = 0;

    // Basic BFS from each trailhead.
    for trailhead in trailheads.iter().copied() {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut visit_queue: VecDeque<(i32, i32)> = VecDeque::new();
        visited.insert(trailhead);
        visit_queue.push_back(trailhead);

        while let Some(current) = visit_queue.pop_front() {
            if map.get(&current).is_some_and(|v| *v == 9) {
                part1 += 1;
            }

            let current_height = map.get(&current).unwrap();

            for offset in offsets.clone() {
                let neighbour = (current.0 + offset.0, current.1 + offset.1);
                let is_single_step = map
                    .get(&neighbour)
                    .is_some_and(|h| *h == current_height + 1);

                if is_single_step && !visited.contains(&neighbour) {
                    visited.insert(neighbour);
                    visit_queue.push_back(neighbour);
                }
            }
        }
    }

    // Part 2
    let mut part2 = 0;

    // Changes from part 1 is that we don't keep track of visited locations.
    // This makes it so we can explore a given location possibly multiple times,
    // which we want now, and since we always step upwards we can't loop anyway.
    for trailhead in trailheads {
        let mut visit_queue: VecDeque<(i32, i32)> = VecDeque::new();
        visit_queue.push_back(trailhead);

        while let Some(current) = visit_queue.pop_front() {
            if map.get(&current).is_some_and(|v| *v == 9) {
                part2 += 1;
            }

            let current_height = map.get(&current).unwrap();

            for offset in offsets.clone() {
                let neighbour = (current.0 + offset.0, current.1 + offset.1);
                let is_single_step = map
                    .get(&neighbour)
                    .is_some_and(|h| *h == current_height + 1);

                if is_single_step {
                    visit_queue.push_back(neighbour);
                }
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
        "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9",
        "2"
    );
    example!(
        p1,
        p1_example_2,
        "..90..9
...1.98
...2..7
6543456
765.987
876....
987....
",
        "4"
    );
    example!(
        p1,
        p1_example_3,
        "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01",
        "3"
    );
    example!(
        p1,
        p1_example_4,
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        "36"
    );
    example!(
        p1,
        p1_example_5,
        "0123
1234
8765
9876",
        "1"
    );
    solution!(p1, p1_solution, "794");

    // Part 2
    example!(
        p2,
        p2_example_1,
        ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....",
        "3"
    );
    example!(
        p2,
        p2_example_2,
        "..90..9
...1.98
...2..7
6543456
765.987
876....
987....",
        "13"
    );
    example!(
        p2,
        p2_example_3,
        "012345
123456
234567
345678
4.6789
56789.",
        "227"
    );
    example!(
        p2,
        p2_example_4,
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        "81"
    );
    solution!(p2, p2_solution, "1706");
}
