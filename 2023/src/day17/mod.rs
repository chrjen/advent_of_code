use std::collections::{BinaryHeap, HashMap, HashSet};

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 17: Clumsy Crucible",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod parse;

type Coord = (usize, usize);
type NodeID = (Coord, Dir, u32);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    North,
    West,
    South,
    East,
}

impl Default for Dir {
    fn default() -> Self {
        Self::North
    }
}

impl Dir {
    fn reverse(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::West => Self::East,
            Self::South => Self::North,
            Self::East => Self::West,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct BlockNode {
    coord: Coord,
    from: (Coord, u32),
    heat_loss: u32,
    dir: Dir,
    straight: u32,
}

impl Ord for BlockNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heat_loss
            .cmp(&other.heat_loss)
            .reverse()
            .then(self.straight.cmp(&other.straight).reverse())
    }
}

impl PartialOrd for BlockNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let heat_map: HashMap<(usize, usize), u32> = parse::parse_contraption(&input);

    let part1 = solve_part1(&heat_map);
    let part2 = solve_part2(&heat_map);

    (part1.to_string(), part2.to_string())
}

fn solve_part2(heat_map: &HashMap<(usize, usize), u32>) -> u32 {
    let mut heat_losses: HashMap<NodeID, (u32, NodeID)> = HashMap::new();
    let mut visited: HashSet<NodeID> = HashSet::new();
    let mut next: BinaryHeap<BlockNode> = BinaryHeap::new();
    let mut target: NodeID = Default::default();

    let x_max = heat_map.keys().map(|(x, _)| *x).max().unwrap_or_default();
    let y_max = heat_map.keys().map(|(_, y)| *y).max().unwrap_or_default();

    let start_east = BlockNode {
        coord: (2, 1),
        from: ((1, 1), 0),
        heat_loss: *heat_map.get(&(2, 1)).unwrap(),
        dir: Dir::East,
        straight: 0,
    };
    let start_south = BlockNode {
        coord: (1, 2),
        from: ((1, 1), 0),
        heat_loss: *heat_map.get(&(1, 2)).unwrap(),
        dir: Dir::South,
        straight: 0,
    };
    next.push(start_east);
    next.push(start_south);
    heat_losses.insert(
        ((2, 1), Dir::East, 0),
        (*heat_map.get(&(2, 1)).unwrap(), ((1, 1), Dir::East, 0)),
    );
    heat_losses.insert(
        ((1, 2), Dir::South, 0),
        (*heat_map.get(&(1, 2)).unwrap(), ((1, 1), Dir::South, 0)),
    );

    while let Some(current) = next.pop() {
        // If we found the end, and have walked straight at least four blocks,
        // we save the target state and exit early.
        if current.coord == (x_max, y_max) && current.straight >= 3 {
            target = (current.coord, current.dir, current.straight);
            break;
        }

        if !visited.insert((current.coord, current.dir, current.straight)) {
            continue;
        }

        let (x, y) = current.coord;
        let neighbours = [
            ((x, y - 1), Dir::North),
            ((x - 1, y), Dir::West),
            ((x, y + 1), Dir::South),
            ((x + 1, y), Dir::East),
        ];

        for (neighbour, neighbour_dir) in neighbours {
            if neighbour_dir == current.dir.reverse() {
                continue;
            }
            if let Some(heat_loss) = heat_map.get(&neighbour) {
                let new_heat_loss = current.heat_loss + heat_loss;
                if neighbour_dir != current.dir && current.straight < 3 {
                    continue;
                }
                if neighbour_dir == current.dir && current.straight + 1 >= 10 {
                    continue;
                }
                if neighbour_dir == current.dir {
                    let is_better = heat_losses
                        .get(&(neighbour, neighbour_dir, current.straight + 1))
                        .is_none_or(|&(hl, _)| new_heat_loss < hl);
                    if !is_better {
                        continue;
                    }
                    heat_losses.insert(
                        (neighbour, neighbour_dir, current.straight + 1),
                        (
                            new_heat_loss,
                            (current.coord, current.dir, current.straight),
                        ),
                    );
                    next.push(BlockNode {
                        coord: neighbour,
                        from: (current.coord, current.straight),
                        heat_loss: new_heat_loss,
                        dir: neighbour_dir,
                        straight: current.straight + 1,
                    })
                } else {
                    let is_better = heat_losses
                        .get(&(neighbour, neighbour_dir, 0))
                        .is_none_or(|&(hl, _)| new_heat_loss < hl);
                    if !is_better {
                        continue;
                    }
                    heat_losses.insert(
                        (neighbour, neighbour_dir, 0),
                        (
                            new_heat_loss,
                            (current.coord, current.dir, current.straight),
                        ),
                    );
                    next.push(BlockNode {
                        coord: neighbour,
                        from: (current.coord, current.straight),
                        heat_loss: new_heat_loss,
                        dir: neighbour_dir,
                        straight: 0,
                    })
                }
            }
        }
    }

    // println!();
    // // _print_heat_loss(&heat_map, &heat_losses);
    // _print_path(&heat_map, &heat_losses, target);
    // println!();

    *heat_losses
        .get(&target)
        .map(|(heat_loss, _)| heat_loss)
        .expect("should have visited bottom left corner")
}

fn solve_part1(heat_map: &HashMap<(usize, usize), u32>) -> u32 {
    let mut heat_losses: HashMap<NodeID, (u32, NodeID)> = HashMap::new();
    let mut visited: HashSet<NodeID> = HashSet::new();
    let mut next: BinaryHeap<BlockNode> = BinaryHeap::new();
    let mut target: NodeID = Default::default();

    let x_max = heat_map.keys().map(|(x, _)| *x).max().unwrap_or_default();
    let y_max = heat_map.keys().map(|(_, y)| *y).max().unwrap_or_default();

    let start_east = BlockNode {
        coord: (2, 1),
        from: ((1, 1), 0),
        heat_loss: *heat_map.get(&(2, 1)).unwrap(),
        dir: Dir::East,
        straight: 0,
    };
    let start_south = BlockNode {
        coord: (1, 2),
        from: ((1, 1), 0),
        heat_loss: *heat_map.get(&(1, 2)).unwrap(),
        dir: Dir::South,
        straight: 0,
    };
    next.push(start_east);
    next.push(start_south);
    heat_losses.insert(
        ((2, 1), Dir::East, 0),
        (*heat_map.get(&(2, 1)).unwrap(), ((1, 1), Dir::East, 0)),
    );
    heat_losses.insert(
        ((1, 2), Dir::South, 0),
        (*heat_map.get(&(1, 2)).unwrap(), ((1, 1), Dir::South, 0)),
    );

    while let Some(current) = next.pop() {
        // If we found the end we save the target state and exit early.
        if current.coord == (x_max, y_max) {
            target = (current.coord, current.dir, current.straight);
            break;
        }

        if !visited.insert((current.coord, current.dir, current.straight)) {
            continue;
        }

        let (x, y) = current.coord;
        let neighbours = [
            ((x, y - 1), Dir::North),
            ((x - 1, y), Dir::West),
            ((x, y + 1), Dir::South),
            ((x + 1, y), Dir::East),
        ];

        for (neighbour, neighbour_dir) in neighbours {
            if neighbour == current.from.0 {
                continue;
            }
            if let Some(heat_loss) = heat_map.get(&neighbour) {
                let new_heat_loss = current.heat_loss + heat_loss;
                if neighbour_dir == current.dir && current.straight >= 2 {
                    continue;
                }
                if neighbour_dir == current.dir {
                    let is_better = heat_losses
                        .get(&(neighbour, neighbour_dir, current.straight + 1))
                        .is_none_or(|&(hl, _)| new_heat_loss < hl);
                    if is_better {
                        heat_losses.insert(
                            (neighbour, current.dir, current.straight + 1),
                            (
                                new_heat_loss,
                                (current.coord, current.dir, current.straight),
                            ),
                        );
                        next.push(BlockNode {
                            coord: neighbour,
                            from: (current.coord, current.straight),
                            heat_loss: new_heat_loss,
                            dir: neighbour_dir,
                            straight: current.straight + 1,
                        })
                    }
                } else {
                    let is_better = heat_losses
                        .get(&(neighbour, neighbour_dir, 0))
                        .is_none_or(|&(hl, _)| new_heat_loss < hl);
                    if is_better {
                        heat_losses.insert(
                            (neighbour, neighbour_dir, 0),
                            (
                                new_heat_loss,
                                (current.coord, current.dir, current.straight),
                            ),
                        );
                        next.push(BlockNode {
                            coord: neighbour,
                            from: (current.coord, current.straight),
                            heat_loss: new_heat_loss,
                            dir: neighbour_dir,
                            straight: 0,
                        })
                    }
                }
            }
        }
    }

    // println!();
    // // _print_heat_loss(&heat_map, &heat_losses);
    // _print_path(&heat_map, &heat_losses, target);
    // println!();

    *heat_losses
        .get(&target)
        .map(|(heat_loss, _)| heat_loss)
        .expect("should have visited bottom left corner")
}

fn _print_path(
    heat_map: &HashMap<(usize, usize), u32>,
    heat_losses: &HashMap<NodeID, (u32, NodeID)>,
    target: NodeID,
) {
    let x_min = heat_map.keys().map(|(x, _)| *x).min().unwrap_or_default();
    let y_min = heat_map.keys().map(|(_, y)| *y).min().unwrap_or_default();
    let x_max = heat_map.keys().map(|(x, _)| *x).max().unwrap_or_default();
    let y_max = heat_map.keys().map(|(_, y)| *y).max().unwrap_or_default();

    let mut path: HashMap<Coord, Coord> = HashMap::new();

    let mut current = target;
    while let Some((_, prev)) = heat_losses.get(&current) {
        path.insert(current.0, prev.0);
        current = *prev;
    }

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if let Some((x0, y0)) = path.get(&(x, y)) {
                use std::cmp::Ordering as Ord;
                match (x.cmp(x0), y.cmp(y0)) {
                    (Ord::Less, Ord::Equal) => print!("\x1b[33m◀\x1b[0m"),
                    (Ord::Equal, Ord::Less) => print!("\x1b[33m▲\x1b[0m"),
                    (Ord::Equal, Ord::Greater) => print!("\x1b[33m▼\x1b[0m"),
                    (Ord::Greater, Ord::Equal) => print!("\x1b[33m▶\x1b[0m"),
                    (Ord::Less, Ord::Less) => print!("\x1b[33mA\x1b[0m"),
                    (Ord::Less, Ord::Greater) => print!("\x1b[33mB\x1b[0m"),
                    (Ord::Equal, Ord::Equal) => print!("\x1b[33mC\x1b[0m"),
                    (Ord::Greater, Ord::Less) => print!("\x1b[33mD\x1b[0m"),
                    (Ord::Greater, Ord::Greater) => print!("\x1b[33mE\x1b[0m"),
                };
            } else {
                let c = heat_map.get(&(x, y)).unwrap();
                print!("\x1b[90m{c}\x1b[0m");
            }
        }
        println!()
    }
}

fn _print_heat_loss(
    heat_map: &HashMap<(usize, usize), u32>,
    heat_loss: &HashMap<NodeID, (u32, NodeID)>,
) {
    let x_min = heat_map.keys().map(|(x, _)| *x).min().unwrap_or_default();
    let y_min = heat_map.keys().map(|(_, y)| *y).min().unwrap_or_default();
    let x_max = heat_map.keys().map(|(x, _)| *x).max().unwrap_or_default();
    let y_max = heat_map.keys().map(|(_, y)| *y).max().unwrap_or_default();

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if let Some(s) = heat_loss
                .iter()
                .filter_map(|((coord, _, _), (heat_loss, _))| {
                    (*coord == (x, y)).then_some(heat_loss)
                })
                .min()
            {
                print!("\x1b[32m{s:>5}\x1b[0m");
            } else {
                let c = heat_map.get(&(x, y)).unwrap();
                print!("\x1b[90m{c:>5}\x1b[0m");
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
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        "102"
    );
    solution!(p1, p1_solution, "1110");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        "94"
    );
    example!(
        p2,
        p2_example_2,
        "\
111111111111
999999999991
999999999991
999999999991
999999999991",
        "71"
    );
    example!(
        p2,
        p2_example_3,
        "\
111111111111111
999999999999991
999999999999991
999999999999991
999999999999991",
        "74"
    );
    solution!(p2, p2_solution, "1294");
}
