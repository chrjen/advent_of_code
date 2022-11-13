use regex::Regex;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 6: Probably a Fire Hazard",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    const GRID_SIZE: usize = 1_000_000;
    let input = String::from_utf8_lossy(input);
    let mut grid: Vec<u8> = vec![0; GRID_SIZE];

    let parse = |s: &str| -> (usize, usize, usize, usize) {
        let reg = Regex::new(r"(\d+),(\d+) through (\d+),(\d+)").unwrap();
        let cap = reg.captures(s).unwrap();

        (
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
            cap[4].parse().unwrap(),
        )
    };

    // Part 1
    for line in input.lines() {
        if line.starts_with("turn on") {
            let (x0, y0, x1, y1) = parse(&line[8..]);
            for x in x0..=x1 {
                for y in y0..=y1 {
                    grid[y * 1000 + x] = 1;
                }
            }
        } else if line.starts_with("turn off") {
            let (x0, y0, x1, y1) = parse(&line[9..]);
            for x in x0..=x1 {
                for y in y0..=y1 {
                    grid[y * 1000 + x] = 0;
                }
            }
        } else if line.starts_with("toggle") {
            let (x0, y0, x1, y1) = parse(&line[7..]);
            for x in x0..=x1 {
                for y in y0..=y1 {
                    grid[y * 1000 + x] = 1 - grid[y * 1000 + x];
                }
            }
        } else {
            panic!("unknown command");
        }
    }
    let sum1: u32 = grid.iter().map(|x| *x as u32).sum();

    // Part 2
    for cell in grid.iter_mut().take(GRID_SIZE) {
        *cell = 0;
    }

    for line in input.lines() {
        if line.starts_with("turn on") {
            let (x0, y0, x1, y1) = parse(&line[8..]);
            for x in x0..=x1 {
                for y in y0..=y1 {
                    grid[y * 1000 + x] += 1;
                }
            }
        } else if line.starts_with("turn off") {
            let (x0, y0, x1, y1) = parse(&line[9..]);
            for x in x0..=x1 {
                for y in y0..=y1 {
                    if grid[y * 1000 + x] == 0 {
                        continue;
                    }
                    grid[y * 1000 + x] -= 1;
                }
            }
        } else if line.starts_with("toggle") {
            let (x0, y0, x1, y1) = parse(&line[7..]);
            for x in x0..=x1 {
                for y in y0..=y1 {
                    grid[y * 1000 + x] += 2;
                }
            }
        } else {
            panic!("unknown command");
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
