use std::collections::HashMap;

mod parse;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 22: Monkey Map",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

#[derive(Debug, Clone, Copy)]
enum Movement {
    Forward(u32),
    Right,
    Left,
}

#[derive(Debug, Clone, Copy)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Facing {
    fn rotate(self, movement: Movement) -> Self {
        match (self, movement) {
            (_, Movement::Forward(_)) => self,
            (Facing::Up, Movement::Right) => Facing::Right,
            (Facing::Right, Movement::Right) => Facing::Down,
            (Facing::Down, Movement::Right) => Facing::Left,
            (Facing::Left, Movement::Right) => Facing::Up,

            (Facing::Up, Movement::Left) => Facing::Left,
            (Facing::Left, Movement::Left) => Facing::Down,
            (Facing::Down, Movement::Left) => Facing::Right,
            (Facing::Right, Movement::Left) => Facing::Up,
        }
    }

    fn to_number(self) -> usize {
        match self {
            Facing::Up => 3,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Right => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Air,
    Wall,
    Teleporter {
        vertical: (usize, usize, Facing),
        horizontal: (usize, usize, Facing),
    },
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Air),
            '#' => Ok(Tile::Wall),
            _ => Err(format!("'{value}' is not a valid tile")),
        }
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let (map_input, path_input) = input
        .split_once("\n\n")
        .unwrap_or_else(|| panic!(r"missing '\n\n' delimiter separating map and path input"));

    let map = parse::map(map_input).unwrap();
    let path = parse::path(path_input).unwrap();

    let min_x: usize = 1;
    let min_y: usize = 1;
    let max_x: usize = map.keys().map(|&(x, _)| x).max().unwrap();
    let max_y: usize = map.keys().map(|&(_, y)| y).max().unwrap();

    // Teleporter setup.
    let mut map_part1 = map.clone();

    // Horizontal.
    for y in min_y..=max_y {
        let mut start: Option<(usize, usize)> = None;

        for x in min_x - 1..=max_x {
            let tile = map_part1.get(&(x + 1, y));

            match (start, tile) {
                (None, None) => continue,
                (None, Some(_)) => {
                    start = Some((x, y));
                }
                (Some((x0, y0)), None) => {
                    map_part1.insert(
                        (x0, y0),
                        Tile::Teleporter {
                            vertical: (0, 0, Facing::Left),
                            horizontal: (x, y, Facing::Left),
                        },
                    );
                    map_part1.insert(
                        (x + 1, y),
                        Tile::Teleporter {
                            vertical: (0, 0, Facing::Right),
                            horizontal: (x0 + 1, y0, Facing::Right),
                        },
                    );
                    start = None;
                }
                (Some(_), Some(_)) => continue,
            }
        }
    }

    // Vertical.
    for x in min_x..=max_x {
        let mut start: Option<(usize, usize)> = None;

        for y in min_y - 1..=max_y {
            let tile = map_part1.get(&(x, y + 1));

            match (start, tile.cloned()) {
                (None, None) | (None, Some(Tile::Teleporter { .. })) => continue,
                (None, Some(_)) => {
                    start = Some((x, y));
                }
                (Some((x0, y0)), None) => {
                    if let Some(Tile::Teleporter { horizontal, .. }) = map_part1.get(&(x0, y0)) {
                        map_part1.insert(
                            (x0, y0),
                            Tile::Teleporter {
                                vertical: (x, y, Facing::Up),
                                horizontal: *horizontal,
                            },
                        );
                    } else {
                        map_part1.insert(
                            (x0, y0),
                            Tile::Teleporter {
                                vertical: (x, y, Facing::Up),
                                horizontal: (0, 0, Facing::Up),
                            },
                        );
                    }
                    map_part1.insert(
                        (x, y + 1),
                        Tile::Teleporter {
                            vertical: (x0, y0 + 1, Facing::Down),
                            horizontal: (0, 0, Facing::Down),
                        },
                    );
                    start = None;
                }
                (Some((x0, y0)), Some(Tile::Teleporter { horizontal, .. })) => {
                    if let Some(Tile::Teleporter { horizontal, .. }) = map_part1.get(&(x0, y0)) {
                        map_part1.insert(
                            (x0, y0),
                            Tile::Teleporter {
                                vertical: (x, y, Facing::Up),
                                horizontal: *horizontal,
                            },
                        );
                    } else {
                        map_part1.insert(
                            (x0, y0),
                            Tile::Teleporter {
                                vertical: (x, y, Facing::Up),
                                horizontal: (0, 0, Facing::Up),
                            },
                        );
                    }
                    map_part1.insert(
                        (x, y + 1),
                        Tile::Teleporter {
                            vertical: (x0, y0 + 1, Facing::Down),
                            horizontal,
                        },
                    );
                    start = None;
                }
                (Some(_), Some(_)) => continue,
            }
        }
    }

    // Perform walk.
    let mut facing = Facing::Right;
    let mut pos: (usize, usize) = {
        let start_x = (min_x..=max_x)
            .find(|&x| map.contains_key(&(x, min_y)))
            .unwrap();
        (start_x, min_y)
    };

    let mut history = HashMap::new();
    for &movement in path.iter() {
        match movement {
            Movement::Forward(n) => {
                'moving: for _ in 0..n {
                    use Facing::*;
                    let (mut x, mut y) = pos;
                    match facing {
                        Up => y -= 1,
                        Down => y += 1,
                        Left => x -= 1,
                        Right => x += 1,
                    }

                    loop {
                        let tile = map_part1[&(x, y)];
                        match (facing, tile) {
                            (_, Tile::Air) => {
                                history.insert(pos, facing);
                                pos = (x, y);
                                break;
                            }
                            (_, Tile::Wall) => break 'moving,
                            (Left, Tile::Teleporter { horizontal, .. })
                            | (Right, Tile::Teleporter { horizontal, .. }) => {
                                (x, y, facing) = horizontal;
                            }
                            (Up, Tile::Teleporter { vertical, .. })
                            | (Down, Tile::Teleporter { vertical, .. }) => {
                                (x, y, facing) = vertical;
                            }
                        }
                    }
                }
            }
            Movement::Right | Movement::Left => facing = facing.rotate(movement),
        }
    }

    // Print map.
    // for y in min_y - 1..=max_y + 1 {
    //     for x in min_x - 1..=max_x + 1 {
    //         let tile = map_part1.get(&(x, y));
    //         let his = history.get(&(x, y));

    //         if let Some(his) = his {
    //             print!(
    //                 "{}",
    //                 match his {
    //                     Facing::Up => '^',
    //                     Facing::Down => 'v',
    //                     Facing::Left => '<',
    //                     Facing::Right => '>',
    //                 }
    //             );
    //         } else if let Some(tile) = tile {
    //             match tile {
    //                 Tile::Air => print!("."),
    //                 Tile::Wall => print!("#"),
    //                 Tile::Teleporter {
    //                     vertical: (0, 0, _),
    //                     ..
    //                 } => print!("|"),
    //                 Tile::Teleporter {
    //                     horizontal: (0, 0, _),
    //                     ..
    //                 } => print!("-"),
    //                 Tile::Teleporter { .. } => print!("+"),
    //             }
    //         } else {
    //             print!(" ");
    //         }
    //     }
    //     println!();
    // }

    let part1 = pos.1 * 1000 + pos.0 * 4 + facing.to_number();

    (part1.to_string(), "no yet implemented".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5",
        "6032"
    );
    solution!(p1, p1_solution, "66292");

    // Part 2
    // example!(p2, p2_example_1, "", "0");
    // example!(p2, p2_example_2, "", "0");
    // example!(p2, p2_example_3, "", "0");
    // example!(p2, p2_example_4, "", "0");
    // example!(p2, p2_example_5, "", "0");
    // solution!(p2, p2_solution, "100");
}
