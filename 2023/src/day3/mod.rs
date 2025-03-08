pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 3: Gear Ratios",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

use std::{collections::HashMap, rc::Rc};

mod parse;

#[derive(Clone, Copy, Debug)]
enum Part {
    Number(u32),
    Symbol(char),
}

#[derive(Debug)]
struct Engine {
    layout: HashMap<(usize, usize), Rc<Part>>,
}

impl Engine {
    fn new() -> Engine {
        Engine {
            layout: HashMap::new(),
        }
    }

    fn neighbours(&self, x: usize, y: usize) -> NeighbourIter {
        NeighbourIter {
            engine: self,
            x,
            y,
            index: 0,
        }
    }
}

struct NeighbourIter<'a> {
    engine: &'a Engine,
    x: usize,
    y: usize,
    index: usize,
}

impl<'a> Iterator for NeighbourIter<'a> {
    type Item = &'a Rc<Part>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 8 {
            return None;
        }
        let coord = match self.index {
            0 => (self.x - 1, self.y - 1),
            1 => (self.x, self.y - 1),
            2 => (self.x + 1, self.y - 1),
            3 => (self.x - 1, self.y),
            4 => (self.x + 1, self.y),
            5 => (self.x - 1, self.y + 1),
            6 => (self.x, self.y + 1),
            7 => (self.x + 1, self.y + 1),
            _ => unreachable!("Should never be index other than 0-7"),
        };

        let part = self.engine.layout.get(&coord);
        self.index += 1;

        if part.is_some() { part } else { self.next() }
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let (_, engine) = parse::parse_engine(input.as_ref()).unwrap();

    // Print a map with all digits replaced by an "x".
    // for y in 1..=10 {
    //     for x in 1..=10 {
    //         let part = engine.layout.get(&(x, y));
    //         match part.cloned().as_deref() {
    //             Some(Part::Number(_)) => print!("x",),
    //             Some(Part::Symbol(c)) => print!("{c}"),
    //             None => print!("."),
    //         }
    //     }
    //     println!();
    // }

    // Part 1
    let mut part1_sum = 0;
    let mut seen = Vec::new();
    for (&(x, y), value) in engine.layout.iter() {
        match **value {
            Part::Number(_) => continue,
            Part::Symbol(_) => {
                for part in engine.neighbours(x, y) {
                    match **part {
                        Part::Number(number) => {
                            if !seen.iter().any(|v| Rc::ptr_eq(v, part)) {
                                seen.push(part.clone());
                                part1_sum += number;
                            }
                        }
                        Part::Symbol(_) => continue,
                    }
                }
            }
        }
    }

    // Part 2
    let mut part2_sum = 0;
    for (&(x, y), value) in engine.layout.iter() {
        match **value {
            Part::Number(_) => continue,
            Part::Symbol('*') => {
                let mut seen = Vec::new();
                let mut sum = 1;
                for part in engine.neighbours(x, y) {
                    match **part {
                        Part::Number(number) => {
                            if !seen.iter().any(|v| Rc::ptr_eq(v, part)) {
                                seen.push(part.clone());
                                sum *= number;
                            }
                        }
                        Part::Symbol(_) => continue,
                    }
                }
                if seen.len() == 2 {
                    part2_sum += sum;
                }
            }
            Part::Symbol(_) => continue,
        }
    }

    (part1_sum.to_string(), part2_sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        "4361"
    );
    solution!(p1, p1_solution, "539637");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        "467835"
    );
    solution!(p2, p2_solution, "82818007");
}
