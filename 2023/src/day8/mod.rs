pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 8: Haunted Wasteland",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

use std::collections::{HashMap, HashSet};

use num::integer::lcm;

mod parse;

const START_NODE: &str = "AAA";
const TARGET_NODE: &str = "ZZZ";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'R' => Ok(Direction::Right),
            'r' => Ok(Direction::Right),
            'L' => Ok(Direction::Left),
            'l' => Ok(Direction::Left),
            _ => Err(format!("got invalid direction '{c}'")),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

#[derive(Debug, Clone)]
struct Map<'a> {
    instructions: Vec<Direction>,
    nodes: HashMap<&'a str, Node<'a>>,
}

pub fn solve(input: &[u8]) -> (String, String) {
    _solve(input, false)
}

fn _solve(input: &[u8], part2_only: bool) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let (_, map) = parse::parse_map(input.as_ref()).expect("valid input");

    // Part1
    let mut count: usize = 0;
    let mut next_node = START_NODE;
    if !part2_only {
        for instruction in map.instructions.iter().cycle() {
            if next_node == TARGET_NODE {
                break;
            }

            let node = map
                .nodes
                .get(next_node)
                .expect("connected node should exist");

            next_node = match *instruction {
                Direction::Left => node.left,
                Direction::Right => node.right,
            };

            count += 1;
        }
    }
    let part1 = count;

    // Part 2
    let start_node_set: HashSet<&str> = map
        .nodes
        .keys()
        .filter(|name| name.ends_with('A'))
        .copied()
        .collect();

    let cycle_length: Vec<u64> = start_node_set
        .iter()
        .map(|start| {
            let mut loop_count = 0;
            let mut visited_nodes = HashMap::new();
            let mut next_node = *start;

            for (index, instruction) in map.instructions.iter().enumerate().cycle() {
                let node = map
                    .nodes
                    .get(next_node)
                    .expect("connected node should exist");

                next_node = match *instruction {
                    Direction::Left => node.left,
                    Direction::Right => node.right,
                };

                loop_count += 1;

                if visited_nodes.contains_key(&(next_node, index)) {
                    loop_count -= visited_nodes.get(&(next_node, index)).unwrap();
                    break;
                }
                visited_nodes.insert((next_node, index), loop_count);
            }
            loop_count
        })
        .collect();

    let part2 = cycle_length.iter().copied().fold(1, lcm);

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
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        "2"
    );
    example!(
        p1,
        p1_example_2,
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        "6"
    );
    solution!(p1, p1_solution, "13771");

    // Part 2
    #[test]
    fn p2_example_1() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            .as_bytes();

        let (_, result) = _solve(input, true);
        assert_eq!(result, "6")
    }
    solution!(p2, p2_solution, "13129439557681");
}
