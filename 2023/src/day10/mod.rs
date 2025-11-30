use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::RangeInclusive,
};

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 10: Pipe Maze",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod data;
mod parse;

use data::{Direction, Field, Pipe};

type Coord = (i32, i32);

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let field: Field = parse::parse_field(&input);

    // Part 1
    let mut tile_queue: VecDeque<(Coord, Coord, Direction, usize)> = VecDeque::new();
    let mut history: HashMap<Coord, (Coord, Direction, usize)> = HashMap::new();
    let mut field_loop: Field = Field::default();
    let mut loop_length = 0;

    let (start_x, start_y) = field.start;
    tile_queue.push_back(((start_x, start_y - 1), field.start, Direction::South, 0));
    tile_queue.push_back(((start_x, start_y + 1), field.start, Direction::North, 0));
    tile_queue.push_back(((start_x - 1, start_y), field.start, Direction::East, 0));
    tile_queue.push_back(((start_x + 1, start_y), field.start, Direction::West, 0));

    while !tile_queue.is_empty() {
        let (coord, prev_coord, from, distance) = tile_queue.pop_back().unwrap();
        if let Some(pipe) = field.layout.get(&coord)
            && let Some((new_coord, new_from)) = pipe.travel(coord, from)
        {
            history.insert(coord, (prev_coord, from, distance + 1));
            if new_coord == field.start {
                history.insert(new_coord, (coord, from, distance + 2));
                loop_length = loop_length.max(distance + 2);

                let last = coord;
                let mut from = from;
                let mut coord = last;
                loop {
                    if coord == field.start {
                        field_loop
                            .layout
                            .insert(coord, Pipe::from_directions(new_from, from));
                        break;
                    }
                    let pipe = field.layout.get(&coord).copied().unwrap();
                    field_loop.layout.insert(coord, pipe);
                    (coord, from, _) = history.get(&coord).copied().unwrap();
                }
                break;
            }
            tile_queue.push_back((new_coord, coord, new_from, distance + 1))
        }
    }

    let part1 = loop_length / 2;

    // println!();
    // _display_distances(&history);
    // println!("{field_loop}");
    // println!();

    // Part 2
    // This solution uses a new grid of nodes that is offset (-0.5, -0.5)
    // compared to the original grid (field.layout). When moving along this
    // new grid of nodes, two tiles from the original grid is used to determine
    // if the path is clear to any neighbouring nodes. This allows the nodes
    // to travel in between the gaps of the loop.
    //
    // To count the inside tiles another DFS is performed from the start tile.
    // Since the start tile has four nodes surrounding it, some of which is
    // outside the loop, we simply try all four. Since there is only an outside
    // and inside in the input, if we end up going out of bounds we know we are
    // on the outside and can start again from another start node. Likewise if
    // we don't end up out of bounds we know we are inside the loop.
    //
    // Alternative solution is to go row by row and count tiles that are after
    // an odd number of loop pipes, i.e. tiles in the field_loop variable, and
    // ignoring tiles after an even number of tiles. Also know as even-odd rule
    // algorithm for determining if a point is inside a simple polygon.
    let is_free = |tile0: &Coord, tile1: &Coord, direction: &Direction| -> bool {
        let tile0 = field_loop.layout.get(tile0);
        let tile1 = field_loop.layout.get(tile1);
        let going_vertical = matches!(direction, Direction::North | Direction::South);

        match (tile0, tile1) {
            (None, None) => true,
            (None, Some(_)) => true,
            (Some(_), None) => true,
            (Some(pipe0), Some(pipe1)) => match (pipe0, pipe1) {
                (Pipe::Vertical, Pipe::Vertical) => going_vertical,
                (Pipe::Horizontal, Pipe::Horizontal) => !going_vertical,
                (Pipe::Vertical, Pipe::Horizontal) => true,
                (Pipe::Horizontal, Pipe::Vertical) => true,
                (Pipe::NorthEast, Pipe::Vertical) => true,
                (Pipe::Vertical, Pipe::NorthEast) => going_vertical,
                (Pipe::Vertical, Pipe::NorthWest) => going_vertical,
                (Pipe::Vertical, Pipe::SouthWest) => true,
                (Pipe::Vertical, Pipe::SouthEast) => true,
                (Pipe::Horizontal, Pipe::NorthEast) => true,
                (Pipe::Horizontal, Pipe::NorthWest) => !going_vertical,
                (Pipe::Horizontal, Pipe::SouthWest) => !going_vertical,
                (Pipe::Horizontal, Pipe::SouthEast) => true,
                (Pipe::NorthEast, Pipe::Horizontal) => !going_vertical,
                (Pipe::NorthEast, Pipe::NorthEast) => true,
                (Pipe::NorthEast, Pipe::NorthWest) => !going_vertical,
                (Pipe::NorthEast, Pipe::SouthWest) => !going_vertical,
                (Pipe::NorthEast, Pipe::SouthEast) => true,
                (Pipe::NorthWest, Pipe::Vertical) => true,
                (Pipe::NorthWest, Pipe::Horizontal) => true,
                (Pipe::NorthWest, Pipe::NorthEast) => true,
                (Pipe::NorthWest, Pipe::NorthWest) => true,
                (Pipe::NorthWest, Pipe::SouthWest) => true,
                (Pipe::NorthWest, Pipe::SouthEast) => true,
                (Pipe::SouthWest, Pipe::Vertical) => going_vertical,
                (Pipe::SouthWest, Pipe::Horizontal) => true,
                (Pipe::SouthWest, Pipe::NorthEast) => going_vertical,
                (Pipe::SouthWest, Pipe::NorthWest) => going_vertical,
                (Pipe::SouthWest, Pipe::SouthWest) => true,
                (Pipe::SouthWest, Pipe::SouthEast) => true,
                (Pipe::SouthEast, Pipe::Vertical) => going_vertical,
                (Pipe::SouthEast, Pipe::Horizontal) => !going_vertical,
                (Pipe::SouthEast, Pipe::NorthEast) => going_vertical,
                (Pipe::SouthEast, Pipe::NorthWest) => false,
                (Pipe::SouthEast, Pipe::SouthWest) => !going_vertical,
                (Pipe::SouthEast, Pipe::SouthEast) => true,
            },
        }
    };

    let (x, y) = field.start;
    let start_nodes = &[(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)];

    let mut node_visited: HashSet<Coord> = HashSet::new();
    let mut tile_inside: HashSet<Coord> = HashSet::new();
    let (x_bound, y_bound) = get_bounds(field_loop.layout.keys());

    for start_node in start_nodes {
        tile_inside.clear();
        if node_visited.contains(start_node) {
            continue;
        }

        let mut node_queue: VecDeque<Coord> = VecDeque::new();
        let mut out_of_bounds = false;

        node_queue.push_back(*start_node);
        'outer: while !node_queue.is_empty() {
            let node = node_queue.pop_back().unwrap();
            node_visited.insert(node);

            if !field_loop.layout.contains_key(&node) {
                tile_inside.insert(node);
            }

            let (x, y) = node;
            let neighbours = &[
                ((x, y + 1), (x - 1, y), (x, y), Direction::South),
                ((x, y - 1), (x - 1, y - 1), (x, y - 1), Direction::North),
                ((x + 1, y), (x, y - 1), (x, y), Direction::East),
                ((x - 1, y), (x - 1, y - 1), (x - 1, y), Direction::West),
            ];

            for (neighbour, tile0, tile1, direction) in neighbours {
                if node_visited.contains(neighbour) || !is_free(tile0, tile1, direction) {
                    continue;
                }
                if !x_bound.contains(&neighbour.0) || !y_bound.contains(&neighbour.1) {
                    out_of_bounds = true;
                    break 'outer;
                }
                node_queue.push_back(*neighbour);
            }
        }

        if !out_of_bounds {
            break;
        }
    }

    let part2 = tile_inside.len();

    // println!();
    // _display_tiles(&field_loop, &node_visited);
    // _display_tiles(&field_loop, &tile_inside);
    // println!();

    (part1.to_string(), part2.to_string())
}

fn get_bounds<'a>(
    coords: impl Iterator<Item = &'a Coord> + Clone,
) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let min_x = coords.clone().map(|&(x, _)| x).min().unwrap();
    let min_y = coords.clone().map(|&(_, y)| y).min().unwrap();
    let max_x = coords.clone().map(|&(x, _)| x).max().unwrap();
    let max_y = coords.map(|&(_, y)| y).max().unwrap();

    (min_x..=max_x, min_y..=max_y)
}

/// Shows the distance numbers. Breaks for large numbers so should only be used on examples.
fn _display_distances(history: &HashMap<Coord, (Coord, Direction, usize)>) {
    let (x_bound, y_bound) = get_bounds(history.keys());

    for y in y_bound {
        for x in x_bound.clone() {
            let distance = history.get(&(x, y));
            match distance {
                Some((_, _, distance)) => print!("{:^4}", distance),
                None => print!("  . "),
            }
        }
        println!();
    }
}

fn _display_tiles(field: &Field, tiles: &HashSet<Coord>) {
    let (x_bound, y_bound) = get_bounds(field.layout.keys().chain(tiles.iter()));

    for y in y_bound {
        for x in x_bound.clone() {
            let pipe = field.layout.get(&(x, y));
            let tile = tiles.get(&(x, y));
            match (tile, pipe) {
                (Some(_), _) => print!("\x1b[33m█\x1b[0m"),
                (None, Some(pipe)) => print!("\x1b[32m{}\x1b[0m", pipe),
                _ => print!("\x1b[90m.\x1b[0m"),
            }
        }
        println!();
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
        "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        "4"
    );
    example!(
        p1,
        p1_example_2,
        "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        "8"
    );
    solution!(p1, p1_solution, "6773");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        "1"
    );
    example!(
        p2,
        p2_example_2,
        "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        "1"
    );
    example!(
        p2,
        p2_example_3,
        "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        "4"
    );
    example!(
        p2,
        p2_example_4,
        "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........",
        "4"
    );
    example!(
        p2,
        p2_example_5,
        ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        "8"
    );
    example!(
        p2,
        p2_example_6,
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        "10"
    );
    solution!(p2, p2_solution, "493");
}
