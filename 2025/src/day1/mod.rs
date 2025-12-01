use common_v2::prelude::*;

pub struct Solution;

pub struct State {
    rotations: Box<[i32]>,
}

impl Solver for Solution {
    fn title() -> &'static str {
        return "Day 1: Secret Entrance";
    }

    fn input() -> &'static [u8] {
        static INPUT: &'static [u8] = std::include_bytes!("input");
        return &INPUT;
    }

    fn initial(input: &[u8]) -> Box<dyn PartSolver + 'static> {
        let input = String::from_utf8_lossy(input);
        let mut rotations: Vec<_> = Vec::new();
        for line in input.lines() {
            let (direction, offset) = line.split_at(1);
            let offset: i32 = offset
                .parse()
                .expect("Rotation offset should be an integer.");
            rotations.push(match direction {
                "L" => -offset,
                "R" => offset,
                c => panic!("Expected direction to be L or R, got '{c}'."),
            });
        }

        Box::new(State {
            rotations: rotations.into_boxed_slice(),
        })
    }
}

impl PartSolver for State {
    fn part1(&self) -> Output {
        let mut position = 50;
        let mut password = 0;

        for rotation in self.rotations.iter() {
            position += rotation;
            position = position.rem_euclid(100);
            if position == 0 {
                password += 1;
            }
        }

        password.into()
    }

    fn part2(&self) -> Output {
        let mut position = 50;
        let mut password = 0;

        for &rotation in self.rotations.iter() {
            let step = rotation.signum();
            for _ in 0..rotation.abs() {
                position = (position + step).rem_euclid(100);
                if position == 0 {
                    password += 1;
                }
            }
        }

        password.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_v2::{example, solution};

    example!(
        p1,
        p1_example_2,
        "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        3
    );
    solution!(p1, p1_solution, 1078);

    example!(
        p2,
        p2_example_1,
        "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        6
    );

    solution!(p2, p2_solution, 6412);
}
