pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 14: Restroom Redoubt",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod parse;

// use std::collections::HashSet;

use nalgebra::Vector2;

#[derive(Debug, Clone, PartialEq)]
struct Robot {
    pos: Vector2<i64>,
    vel: Vector2<i64>,
}

pub fn solve(input: &[u8]) -> (String, String) {
    solve_(input, 101, 103)
}

pub fn solve_(input: &[u8], width: i64, height: i64) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let mut robots: Vec<Robot> = parse::parse_robots(input).expect("input should be valid");

    // Part 1
    let mut q1_count = 0;
    let mut q2_count = 0;
    let mut q3_count = 0;
    let mut q4_count = 0;

    for robot in robots.iter() {
        let pos = robot.pos + 100 * robot.vel;
        let pos_x = pos.x.rem_euclid(width);
        let pos_y = pos.y.rem_euclid(height);

        if pos_x < width / 2 && pos_y < height / 2 {
            q1_count += 1;
        } else if pos_x > width / 2 && pos_y < height / 2 {
            q2_count += 1;
        } else if pos_x < width / 2 && pos_y > height / 2 {
            q3_count += 1;
        } else if pos_x > width / 2 && pos_y > height / 2 {
            q4_count += 1;
        }
    }

    let part1 = q1_count * q2_count * q3_count * q4_count;

    // Part 2
    let mut part2 = None;
    // let mut positions: HashSet<Vector2<i64>> = HashSet::new();

    for i in 1..=width * height {
        let mut q1_count = 0;
        let mut q2_count = 0;
        let mut q3_count = 0;
        let mut q4_count = 0;

        // positions.clear();
        for robot in robots.iter_mut() {
            robot.pos += robot.vel;
            robot.pos.x = robot.pos.x.rem_euclid(width);
            robot.pos.y = robot.pos.y.rem_euclid(height);
            // positions.insert(robot.pos);

            if robot.pos.x < width / 2 && robot.pos.y < height / 2 {
                q1_count += 1;
            } else if robot.pos.x > width / 2 && robot.pos.y < height / 2 {
                q2_count += 1;
            } else if robot.pos.x < width / 2 && robot.pos.y > height / 2 {
                q3_count += 1;
            } else if robot.pos.x > width / 2 && robot.pos.y > height / 2 {
                q4_count += 1;
            }
        }

        // Non-christmas tree configurations are likely to be uniformly random so
        // we can exploit that by looking for a time with a skewed distribution.
        // Here I am trying to find a quadrant with significantly more robots than
        // what is likely through random chance, specifically any with more than
        // half of all robots. The specific chosen threshold was arbitrary.
        if q1_count > robots.len() / 2
            || q2_count > robots.len() / 2
            || q3_count > robots.len() / 2
            || q4_count > robots.len() / 2
        {
            // println!("\nFound after {i} iterations.");
            // for y in 0..width {
            //     for x in 0..width {
            //         if positions.contains(&Vector2::new(x, y)) {
            //             print!("#");
            //         } else {
            //             print!(".")
            //         }
            //     }
            //     println!();
            // }

            part2 = Some(i);
            break;
        }
    }

    (part1.to_string(), common::from_option(part2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::solution;

    #[test]
    fn p1_example_1() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        println!("input: {}", input);
        let (result, _) = solve_(str::as_bytes(input), 11, 7);
        assert_eq!(result, "12");
    }
    solution!(p1, p1_solution, "225521010");

    // Part 2
    solution!(p2, p2_solution, "7774");
}
