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

    // Part 1
    let heat_map = parse::parse_contraption(&input);

    let mut heat_losses: HashMap<NodeID, (u32, NodeID)> = HashMap::new();
    let mut visited: HashSet<NodeID> = HashSet::new();
    let mut next: BinaryHeap<BlockNode> = BinaryHeap::new();

    let start = BlockNode {
        coord: (1, 1),
        from: ((0, 0), 0),
        heat_loss: 0,
        dir: Dir::South,
        straight: 0,
    };
    next.push(start);
    heat_losses.insert(((1, 1), Dir::South, 0), (0, ((0, 0), Dir::South, 0)));

    while let Some(current) = next.pop() {
        if !visited.insert((current.coord, current.dir, current.straight)) {
            continue;
        }

        let (x, y) = current.coord;
        let ((x0, y0), _) = current.from;
        let neighbours = [
            ((x, y + 1), Dir::North),
            ((x - 1, y), Dir::West),
            ((x, y - 1), Dir::South),
            ((x + 1, y), Dir::East),
        ];
        let straight_neighbour = (2 * x - x0, 2 * y - y0);

        for (neighbour, neighbour_dir) in neighbours {
            if neighbour == current.from.0 {
                continue;
            }
            if let Some(heat_loss) = heat_map.get(&neighbour) {
                let new_heat_loss = current.heat_loss + heat_loss;
                if neighbour == straight_neighbour && current.straight >= 2 {
                    continue;
                } else if neighbour == straight_neighbour {
                    let is_better = heat_losses
                        .get(&(neighbour, neighbour_dir, 0))
                        .map_or(true, |&(hl, _)| new_heat_loss < hl);
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
                        .map_or(true, |&(hl, _)| new_heat_loss < hl);
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

    let x_max = heat_map.keys().map(|(x, _)| *x).max().unwrap_or_default();
    let y_max = heat_map.keys().map(|(_, y)| *y).max().unwrap_or_default();

    // dbg!(x_max, y_max, &heat_losses);

    let part1 = heat_losses
        .iter()
        .filter_map(|((coord, _, _), (heat_loss, _))| {
            (*coord == (x_max, y_max)).then_some(heat_loss)
        })
        .min()
        .expect("should have visited bottom left corner");

    // let mut path: HashMap<Coord, Coord> = HashMap::new();

    // // let mut current = ((x_max, y_max), straight);
    // while let Some(&(_, from)) = heat_losses.get(&current) {
    //     path.insert(current.0, from.0);
    //     current = from;
    // }

    // println!();
    // _print_path(&heat_map, &path);
    // println!();

    (part1.to_string(), 0.to_string())
}

fn _print_path(heat_map: &HashMap<(usize, usize), u32>, path: &HashMap<Coord, Coord>) {
    let x_min = heat_map.keys().map(|(x, _)| *x).min().unwrap_or_default();
    let y_min = heat_map.keys().map(|(_, y)| *y).min().unwrap_or_default();
    let x_max = heat_map.keys().map(|(x, _)| *x).max().unwrap_or_default();
    let y_max = heat_map.keys().map(|(_, y)| *y).max().unwrap_or_default();

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if let Some((x0, y0)) = path.get(&(x, y)) {
                use std::cmp::Ordering as Ord;
                match (x.cmp(x0), y.cmp(y0)) {
                    (Ord::Less, Ord::Equal) => print!("\x1b[32m<\x1b[0m"),
                    (Ord::Equal, Ord::Less) => print!("\x1b[32m^\x1b[0m"),
                    (Ord::Equal, Ord::Greater) => print!("\x1b[32mV\x1b[0m"),
                    (Ord::Greater, Ord::Equal) => print!("\x1b[32m>\x1b[0m"),
                    (Ord::Less, Ord::Less) => print!("\x1b[32mA\x1b[0m"),
                    (Ord::Less, Ord::Greater) => print!("\x1b[32mB\x1b[0m"),
                    (Ord::Equal, Ord::Equal) => print!("\x1b[32mC\x1b[0m"),
                    (Ord::Greater, Ord::Less) => print!("\x1b[32mD\x1b[0m"),
                    (Ord::Greater, Ord::Greater) => print!("\x1b[32mE\x1b[0m"),
                };
            } else {
                let c = heat_map.get(&(x, y)).unwrap();
                print!("\x1b[90m{c}\x1b[0m");
            }
        }
        println!()
    }
}

fn _print_heat_loss(heat_map: &HashMap<(usize, usize), u32>, heat_loss: &HashMap<Coord, u32>) {
    let x_min = heat_map.keys().map(|(x, _)| *x).min().unwrap_or_default();
    let y_min = heat_map.keys().map(|(_, y)| *y).min().unwrap_or_default();
    let x_max = heat_map.keys().map(|(x, _)| *x).max().unwrap_or_default();
    let y_max = heat_map.keys().map(|(_, y)| *y).max().unwrap_or_default();

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if let Some(&s) = heat_loss.get(&(x, y)) {
                print!("\x1b[32m{s:>3}\x1b[0m");
            } else {
                let c = heat_map.get(&(x, y)).unwrap();
                print!("\x1b[90m{c:>3}\x1b[0m");
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
    // example!(p2, p2_example_1, "", "0");
    // example!(p2, p2_example_2, "", "0");
    // example!(p2, p2_example_3, "", "0");
    // example!(p2, p2_example_4, "", "0");
    // example!(p2, p2_example_5, "", "0");
    // solution!(p2, p2_solution, "100");
}
