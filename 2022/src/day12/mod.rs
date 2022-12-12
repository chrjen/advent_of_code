use std::collections::HashMap;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 12: Hill Climbing Algorithm",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

#[derive(Debug)]
struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    fn get(&self, x: usize, y: usize) -> &T {
        &self.data[y * self.width + x]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Start,
    End,
    Ground(char),
}

impl Tile {
    fn elevation(&self) -> u32 {
        match self {
            Tile::Start => b'a' as u32,
            Tile::End => b'z' as u32,
            Tile::Ground(x) => *x as u32,
        }
    }
}

struct PathFinder<'a> {
    grid: &'a Grid<Tile>,
    visited: HashMap<(usize, usize), usize>,
    unvisited: HashMap<(usize, usize), usize>,
}

impl<'a> PathFinder<'a> {
    fn new(grid: &'a Grid<Tile>) -> Self {
        PathFinder {
            grid,
            visited: HashMap::new(),
            unvisited: HashMap::new(),
        }
    }

    fn path_len<F>(&mut self, start: (usize, usize), f: F, rev: bool) -> Option<usize>
    where
        F: Fn((usize, usize), u32) -> bool,
    {
        self.unvisited.insert(start, 0);

        while !self.unvisited.is_empty() {
            let (&current, &current_value) = self.unvisited.iter().min_by_key(|&(_, v)| v).unwrap();
            let current_height = self.grid.get(current.0, current.1).elevation();

            if f(current, current_height) {
                return Some(current_value);
            }

            let too_steep = |neighbour: (usize, usize)| {
                if rev {
                    self.grid.get(neighbour.0, neighbour.1).elevation() < current_height - 1
                } else {
                    self.grid.get(neighbour.0, neighbour.1).elevation() > current_height + 1
                }
            };

            let mut process_tile = |neighbour: (usize, usize)| {
                if !self.visited.contains_key(&neighbour) && !too_steep(neighbour) {
                    if let Some(value) = self.unvisited.get_mut(&neighbour) {
                        *value = (*value).min(current_value + 1);
                    } else {
                        self.unvisited.insert(neighbour, current_value + 1);
                    }
                }
            };

            // Left.
            if current.0 > 0 {
                let neighbour = (current.0 - 1, current.1);
                process_tile(neighbour);
            }
            // Right.
            if current.0 < self.grid.width - 1 {
                let neighbour = (current.0 + 1, current.1);
                process_tile(neighbour);
            }
            // Top.
            if current.1 > 0 {
                let neighbour = (current.0, current.1 - 1);
                process_tile(neighbour);
            }
            // Bottom.
            if current.1 < self.grid.height - 1 {
                let neighbour = (current.0, current.1 + 1);
                process_tile(neighbour);
            }

            self.visited
                .insert(current, self.unvisited.remove(&current).unwrap());
        }

        None
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let grid = {
        let width = input.lines().next().unwrap().len();
        let data: Vec<Tile> = input
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| match c {
                'a'..='z' => Tile::Ground(c),
                'S' => Tile::Start,
                'E' => Tile::End,
                x => panic!("got invalid grid value '{x}'"),
            })
            .collect();
        let height = data.len() / width;

        Grid {
            width,
            height,
            data,
        }
    };

    let to_cord = |(index, _)| (index % grid.width, index / grid.width);

    let start = grid
        .data
        .iter()
        .enumerate()
        .find(|(_, t)| **t == Tile::Start)
        .map_or_else(|| panic!("no start"), to_cord);
    let end = grid
        .data
        .iter()
        .enumerate()
        .find(|(_, t)| **t == Tile::End)
        .map_or_else(|| panic!("no end"), to_cord);

    // Part 1 & 2.
    let part1 = PathFinder::new(&grid).path_len(start, |current, _| current == end, false);
    let part2 = PathFinder::new(&grid).path_len(
        end,
        |_, height| height == Tile::Ground('a').elevation(),
        true,
    );

    (common::from_option(part1), common::from_option(part2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
        "31"
    );
    solution!(p1, p1_solution, "339");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
        "29"
    );
    solution!(p2, p2_solution, "332");
}
