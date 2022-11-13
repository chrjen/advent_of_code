use regex::Regex;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 6: Probably a Fire Hazard",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

type Cord = (usize, usize, usize, usize);

enum Instruction {
    TurnOn(Cord),
    TurnOff(Cord),
    Toggle(Cord),
}

pub fn solve(input: &[u8]) -> (String, String) {
    const GRID_SIZE: usize = 1_000_000;
    let input = String::from_utf8_lossy(input);
    let mut grid: Vec<u8> = vec![0; GRID_SIZE];

    // Parse all instructions.
    let instructions = {
        let mut result = Vec::new();

        let parse = |s: &str| -> Cord {
            let reg = Regex::new(r"(\d+),(\d+) through (\d+),(\d+)").unwrap();
            let cap = reg.captures(s).unwrap();

            (
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
                cap[3].parse().unwrap(),
                cap[4].parse().unwrap(),
            )
        };

        for line in input.lines() {
            if line.starts_with("turn on") {
                result.push(Instruction::TurnOn(parse(&line[8..])));
            } else if line.starts_with("turn off") {
                result.push(Instruction::TurnOff(parse(&line[9..])));
            } else if line.starts_with("toggle") {
                result.push(Instruction::Toggle(parse(&line[7..])));
            } else {
                panic!("unknown command");
            }
        }

        result
    };

    // Loop over subgrid specified by `cord` modifying each grid cell.
    let map_grid = |grid: &mut Vec<u8>, cord: &Cord, f: &mut dyn FnMut(&mut u8)| {
        let (x0, y0, x1, y1) = *cord;
        for x in x0..=x1 {
            for y in y0..=y1 {
                f(&mut grid[y * 1000 + x]);
            }
        }
    };

    // Part 1
    for instruction in instructions.iter() {
        use Instruction::*;
        match instruction {
            TurnOn(cord) => {
                map_grid(&mut grid, cord, &mut |x| *x = 1);
            }
            TurnOff(cord) => {
                map_grid(&mut grid, cord, &mut |x| *x = 0);
            }
            Toggle(cord) => {
                map_grid(&mut grid, cord, &mut |x| *x = 1 - *x);
            }
        }
    }
    let sum1: u32 = grid.iter().map(|x| *x as u32).sum();

    // Part 2
    for cell in grid.iter_mut().take(GRID_SIZE) {
        *cell = 0;
    }
    for instruction in instructions.iter() {
        use Instruction::*;
        match instruction {
            TurnOn(cord) => {
                map_grid(&mut grid, cord, &mut |x| *x += 1);
            }
            TurnOff(cord) => {
                map_grid(&mut grid, cord, &mut |x| {
                    if *x == 0 {
                        return;
                    }
                    *x -= 1;
                });
            }
            Toggle(cord) => {
                map_grid(&mut grid, cord, &mut |x| *x += 2);
            }
        }
    }

    let sum2: u32 = grid.iter().map(|x| *x as u32).sum();

    (sum1.to_string(), sum2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(p1, p1_example_1, "turn on 0,0 through 999,999", "1000000");
    example!(p1, p1_example_2, "toggle 0,0 through 999,0", "1000");
    example!(p1, p1_example_3, "turn off 499,499 through 500,500", "0");
    solution!(p1, p1_solution, "543903");

    // Part 2
    example!(p2, p2_example_1, "turn on 0,0 through 0,0", "1");
    example!(p2, p2_example_2, "turn off 0,0 through 0,0", "0");
    example!(p2, p2_example_3, "toggle 0,0 through 999,999", "2000000");
    solution!(p2, p2_solution, "14687245");
}
