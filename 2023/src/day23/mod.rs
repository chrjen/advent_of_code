use std::collections::{HashMap, HashSet};

use crossterm::style::Stylize;
use itertools::Itertools;

use crate::day23::data::Tile;

use self::data::{Coord, HikingTrails};

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 23: A Long Walk",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod data;
mod parse;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let mut trails = parse::parse_hiking_trails(&input);

    // Make sure the ends are closed off.
    let (sx, sy) = trails.start;
    let (ex, ey) = trails.end;
    trails.map.insert((sx, sy - 1), Tile::Forest);
    trails.map.insert((ex, ey + 1), Tile::Forest);

    let trails = trails;

    // _print_hiking_trails(&trails);

    // Part 1
    // Basic DFS, though we don't terminate early and instead wait until we
    // have explored all paths, making sure to keep track of the longest
    // distance to each location.
    let mut next = Vec::new();
    let mut distances: HashMap<Coord, usize> = HashMap::new();
    next.push((trails.start, None, 0usize));

    while let Some((current, prev, dist)) = next.pop() {
        let (x, y) = current;

        let neighbours = [
            ((x, y - 1), Tile::SlopeNorth),
            ((x + 1, y), Tile::SlopeEast),
            ((x, y + 1), Tile::SlopeSouth),
            ((x - 1, y), Tile::SlopeWest),
        ];

        distances
            .entry(current)
            .and_modify(|d| {
                *d = (*d).max(dist);
            })
            .or_insert(dist);

        for (neighbour, neighbour_slope) in neighbours {
            if prev.is_some_and(|prev| neighbour == prev) {
                continue;
            }

            let tile = trails.map.get(&current);
            match tile {
                Some(
                    slope_tile @ (Tile::SlopeNorth
                    | Tile::SlopeEast
                    | Tile::SlopeSouth
                    | Tile::SlopeWest),
                ) => {
                    if neighbour_slope == *slope_tile {
                        next.push((neighbour, Some(current), dist + 1));
                    }
                }
                Some(Tile::Forest) => {}

                None => next.push((neighbour, Some(current), dist + 1)),
            }
        }
    }

    let part1 = distances
        .get(&trails.end)
        .expect("should have found the end location");

    // Part 2
    // We first need to simplify the graph by reducing the path between
    // intersections. Turning it from an unweighted graph to a weighted one.
    // We use DFS to quickly find the next intersection, give it a name,
    // and store edges to previous intersections. This is an optimisation to
    // make later path finding faster, especially part 2.
    let mut queue = Vec::new();
    let mut nodes: HashMap<Coord, Vec<(usize, Coord)>> = HashMap::new();
    let mut visited: HashSet<Coord> = HashSet::new();
    queue.push((trails.start, None, 0, trails.start));
    nodes.insert(trails.start, Vec::new());

    while let Some((current, prev, dist, prev_node)) = queue.pop() {
        let (x, y) = current;

        if !visited.insert(current) {
            if nodes.contains_key(&current) {
                nodes.get_mut(&prev_node).unwrap().push((dist, current));
                nodes.get_mut(&current).unwrap().push((dist, prev_node));
            }
            continue;
        }

        let mut neighbours = [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
            .into_iter()
            .filter(|neighbour| {
                let tile = trails.map.get(neighbour);
                let is_blocked = matches!(tile, Some(Tile::Forest));

                !is_blocked && !prev.is_some_and(|prev: Coord| *neighbour == prev)
            })
            .collect::<Vec<_>>();

        if neighbours.len() == 1 {
            queue.push((
                neighbours.pop().unwrap(),
                Some(current),
                dist + 1,
                prev_node,
            ));
        } else if neighbours.len() >= 2 {
            nodes.get_mut(&prev_node).unwrap().push((dist, current));
            nodes.insert(current, vec![(dist, prev_node)]);
            for neighbour in neighbours {
                queue.push((neighbour, Some(current), 1, current));
            }
        } else if neighbours.is_empty() {
            nodes.get_mut(&prev_node).unwrap().push((dist, current));
            nodes.insert(current, vec![(dist, prev_node)]);
        }
    }

    // // Prints out the new simplified graph based on the intersections.
    // for (coord, edges) in nodes.iter() {
    //     for (weight, coord2) in edges.iter() {
    //         println!("{:?} -{}-> {:?}", coord, weight, coord2);
    //     }
    // }

    // Basic DFS, same as part 1, though now we also keep track of ALL
    // previously explored nodes so that we don't visit any twice. In part 1
    // only the previous tile/node was enough.
    let mut next: Vec<(Coord, HashSet<Coord>, usize)> = Vec::new();
    let mut distances: HashMap<Coord, usize> = HashMap::new();
    next.push((trails.start, HashSet::new(), 0usize));

    while let Some((current, mut prev, dist)) = next.pop() {
        prev.insert(current);

        let neighbours = nodes.get(&current).unwrap();

        distances
            .entry(current)
            .and_modify(|d| {
                *d = (*d).max(dist);
            })
            .or_insert(dist);

        for (neighbour_dist, neighbour) in neighbours {
            if prev.contains(neighbour) {
                continue;
            }

            let tile = trails.map.get(&current);
            match tile {
                Some(Tile::Forest) => {}
                _ => next.push((*neighbour, prev.clone(), dist + neighbour_dist)),
            }
        }
    }

    let part2 = distances
        .get(&trails.end)
        .expect("should have found the end location");

    (part1.to_string(), part2.to_string())
}

fn _print_hiking_trails(trails: &HikingTrails) {
    let (&x_min, &x_max) = trails
        .map
        .keys()
        .map(|(x, _)| x)
        .minmax()
        .into_option()
        .unwrap_or((&0, &0));
    let (&y_min, &y_max) = trails
        .map
        .keys()
        .map(|(_, y)| y)
        .minmax()
        .into_option()
        .unwrap_or((&0, &0));

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if (x, y) == trails.start {
                print!("S");
                continue;
            }
            if (x, y) == trails.end {
                print!("E");
                continue;
            }
            match trails.map.get(&(x, y)) {
                Some(Tile::Forest) => print!("#"),
                Some(Tile::SlopeNorth) => print!("^"),
                Some(Tile::SlopeEast) => print!(">"),
                Some(Tile::SlopeSouth) => print!("v"),
                Some(Tile::SlopeWest) => print!("<"),
                None => print!("{}", ".".dark_grey()),
            }
        }
        println!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#",
        "94"
    );
    solution!(p1, p1_solution, "2278", ignore = "takes too long");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#",
        "154"
    );
    solution!(p2, p2_solution, "6734", ignore = "takes too long");
}
