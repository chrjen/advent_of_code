pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 21: Step Counter",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod data;
mod parse;

pub type Coord = (i32, i32);

use std::collections::{HashSet, VecDeque};

use crossterm::style::Stylize;

pub fn solve(input: &[u8]) -> (String, String) {
    solve_for_steps(input, 64)
}

pub fn solve_for_steps(input: &[u8], steps: usize) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let (start, rocks) = parse::parse_garden_map(&input);

    // Part 1
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut current_dist = 0;
    queue.push_back((start, 0isize));

    let part1: usize = loop {
        let ((x, y), dist) = queue
            .pop_front()
            .unwrap_or_else(|| panic!("solve ended before finding a solution"));
        let new_dist = dist + 1;

        if new_dist == steps as isize + 2 {
            // println!("\n FINAL SOLUTION FOR STEP {steps}:");
            // _print_garden_map(&rocks, &visited);
            break visited.len();
        } else if new_dist != current_dist {
            // println!("\n AFTER {} STEPS:", current_dist - 2);
            // _print_garden_map(&rocks, &visited);
            visited.clear();
            current_dist = new_dist;
        }

        if !visited.insert((x, y)) {
            continue;
        }

        let neighbours = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].into_iter();

        for neighbour in neighbours {
            if rocks.contains(&neighbour) {
                continue;
            }

            queue.push_back((neighbour, new_dist));
        }
    };

    (part1.to_string(), 0.to_string())
}

fn _print_garden_map(rocks: &HashSet<Coord>, visited: &HashSet<Coord>) {
    let x_min = rocks
        .iter()
        .chain(visited.iter())
        .map(|(x, _)| *x)
        .min()
        .unwrap_or_default();
    let y_min = rocks
        .iter()
        .chain(visited.iter())
        .map(|(_, y)| *y)
        .min()
        .unwrap_or_default();
    let x_max = rocks
        .iter()
        .chain(visited.iter())
        .map(|(x, _)| *x)
        .max()
        .unwrap_or_default();
    let y_max = rocks
        .iter()
        .chain(visited.iter())
        .map(|(_, y)| *y)
        .max()
        .unwrap_or_default();

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if visited.get(&(x, y)).is_some() {
                print!("O");
            } else if rocks.get(&(x, y)).is_some() {
                print!("#");
            } else {
                print!("{}", ".".dark_grey());
            }
        }
        println!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::solution;

    #[test]
    fn p1_example_1() {
        let input = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        println!("input: {input}");
        let (result, _) = solve_for_steps(str::as_bytes(input), 6);
        assert_eq!(result, "16");
    }
    solution!(p1, p1_solution, "3542");

    // Part 2
    // example!(p2, p2_example_1, "", "0");
    // example!(p2, p2_example_2, "", "0");
    // example!(p2, p2_example_3, "", "0");
    // example!(p2, p2_example_4, "", "0");
    // example!(p2, p2_example_5, "", "0");
    // solution!(p2, p2_solution, "100");
}
