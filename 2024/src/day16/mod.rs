pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 16: Reindeer Maze",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod data;

use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use nalgebra::Vector2;

use data::{Direction, TileNode};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let mut walls: HashSet<Vector2<i32>> = HashSet::new();
    let mut start: Option<Vector2<i32>> = None;
    let mut end: Option<Vector2<i32>> = None;

    for (y, line) in (1..).zip(input.lines()) {
        for (x, c) in (1..).zip(line.chars()) {
            match c {
                'S' | 's' => {
                    if start.is_some() {
                        panic!("found multiple start locations");
                    }
                    start = Some(Vector2::new(x, y))
                }
                'E' | 'e' => {
                    if end.is_some() {
                        panic!("found multiple end locations");
                    }
                    end = Some(Vector2::new(x, y))
                }
                '#' => {
                    walls.insert(Vector2::new(x, y));
                }
                _ => {}
            }
        }
    }

    // Uses Dijkstra's algorithm to find the shortest paths. It's a mess, but
    // it works and I am too tired to keep working on this problem.
    let start_pos = start.unwrap_or_else(|| panic!("input missing start location"));
    let end_pos = end.unwrap_or_else(|| panic!("input missing end location"));

    let mut next: Vec<TileNode> = Vec::new();
    let mut cost: HashMap<TileNode, u32> = HashMap::new();
    let mut visited: HashSet<TileNode> = HashSet::new();
    let mut previous: HashMap<TileNode, Vec<TileNode>> = HashMap::new();

    let start = TileNode::new(start_pos, Direction::East);
    cost.insert(start, 0);
    next.push(start);

    while !next.is_empty() {
        let current = next.remove(next.iter().position_min_by_key(|v| cost.get(v)).unwrap());
        let current_cost = *cost
            .get(&current)
            .expect("should have a cost set by this point");
        visited.insert(TileNode::new(current.pos, current.dir));

        for dir in Direction::all_directions() {
            if dir == current.dir {
                let neighbour_pos = current.pos + dir.to_vector();
                let new_cost = current_cost + 1;

                if walls.contains(&neighbour_pos)
                    || visited.contains(&TileNode::new(neighbour_pos, dir))
                {
                    continue;
                }

                if !next.contains(&TileNode::new(neighbour_pos, dir)) {
                    next.push(TileNode::new(neighbour_pos, dir));
                }

                let neighbour_cost = cost
                    .entry(TileNode::new(neighbour_pos, dir))
                    .or_insert(u32::MAX);

                match new_cost.cmp(neighbour_cost) {
                    std::cmp::Ordering::Less => {
                        *neighbour_cost = new_cost;
                        previous.insert(
                            TileNode::new(neighbour_pos, dir),
                            vec![TileNode::new(current.pos, current.dir)],
                        );
                    }
                    std::cmp::Ordering::Equal => {
                        previous
                            .entry(TileNode::new(neighbour_pos, dir))
                            .and_modify(|v| v.push(TileNode::new(current.pos, current.dir)))
                            .or_insert_with(|| vec![TileNode::new(current.pos, current.dir)]);
                    }
                    std::cmp::Ordering::Greater => { /* Do nothing. */ }
                }
            } else {
                if current.dir.rot_cost(dir) == 2000 {
                    continue;
                }

                let new_cost = current_cost + current.dir.rot_cost(dir);

                if visited.contains(&TileNode::new(current.pos, dir)) {
                    continue;
                }

                if !next.contains(&TileNode::new(current.pos, dir)) {
                    next.push(TileNode::new(current.pos, dir));
                }

                let neighbour_cost = cost
                    .entry(TileNode::new(current.pos, dir))
                    .or_insert(u32::MAX);

                match new_cost.cmp(neighbour_cost) {
                    std::cmp::Ordering::Less => {
                        *neighbour_cost = new_cost;
                        previous.insert(
                            TileNode::new(current.pos, dir),
                            vec![TileNode::new(current.pos, current.dir)],
                        );
                    }
                    std::cmp::Ordering::Equal => {
                        previous
                            .entry(TileNode::new(current.pos, dir))
                            .and_modify(|v| v.push(TileNode::new(current.pos, current.dir)))
                            .or_insert_with(|| vec![TileNode::new(current.pos, current.dir)]);
                    }
                    std::cmp::Ordering::Greater => { /* Do nothing. */ }
                }
            }
        }
    }

    // Part 1
    let part1 = cost
        .get(&TileNode::new(end_pos, Direction::North))
        .expect("should be a path from start to end");

    // Part 2
    let mut best_tiles: HashSet<Vector2<i32>> = HashSet::new();
    let mut next: Vec<TileNode> = Vec::new();
    next.push(TileNode::new(end_pos, Direction::North));

    while let Some(current) = next.pop() {
        best_tiles.insert(current.pos);

        for &prev in previous.get(&current).into_iter().flatten() {
            next.push(prev);
        }
    }

    let part2 = best_tiles.len();

    // println!("=-=-=-=-=-=-=-=-=");
    // let max_x = walls.iter().map(|v| v.x).max().unwrap_or(0);
    // let max_y = walls.iter().map(|v| v.y).max().unwrap_or(0);
    // for y in 1..=max_x {
    //     for x in 1..=max_y {
    //         if walls.contains(&Vector2::new(x, y)) {
    //             print!("#")
    //         } else if best_tiles.contains(&(Vector2::new(x, y))) {
    //             print!("O")
    //         } else {
    //             print!(".")
    //         }
    //     }
    //     println!();
    // }

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
        "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
        "7036"
    );
    example!(
        p1,
        p1_example_2,
        "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
        "11048"
    );
    solution!(
        p1,
        p1_solution,
        "99448",
        ignore = "too slow in debug release"
    );

    // Part 2
    example!(
        p2,
        p2_example_1,
        "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
        "45"
    );
    example!(
        p2,
        p2_example_2,
        "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
        "64"
    );
    solution!(p2, p2_solution, "498", ignore = "too slow in debug release");
}
