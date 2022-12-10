use regex::Regex;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 10: Cathode-Ray Tube",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

use Instruction::*;

struct Cpu<'a> {
    cycle: usize,
    sub_cycle: u8,
    reg_ip: usize,
    reg_x: isize,
    reg_x_next: isize,
    memory: &'a [Instruction],
}

impl<'a> Cpu<'a> {
    fn new(memory: &'a [Instruction]) -> Self {
        Cpu {
            cycle: 0,
            sub_cycle: 0,
            reg_ip: 0,
            reg_x: 1,
            reg_x_next: 1,
            memory,
        }
    }

    fn tick(&mut self) -> bool {
        self.cycle += 1;
        self.sub_cycle += 1;
        self.reg_x = self.reg_x_next;

        match self.sub_cycle {
            1 => match self.memory.get(self.reg_ip) {
                Some(Noop) => {
                    self.reg_ip += 1;
                    self.sub_cycle = 0;
                }
                Some(_) => {}
                None => return false,
            },
            2 => match self.memory[self.reg_ip] {
                Addx(v) => {
                    self.reg_x_next = self.reg_x + v;
                    self.reg_ip += 1;
                    self.sub_cycle = 0;
                }
                x => panic!(
                    "invalid instruction '{:?}', sub_cycle={}",
                    x, self.sub_cycle
                ),
            },
            t => panic!("invalid state sub_cycle == '{}'", t),
        }

        true
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Noop,
    Addx(isize),
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let reg = Regex::new(r"(\w+)\s*(-?\d+)?").unwrap();

    let instruction: Vec<Instruction> = input
        .lines()
        .map(|line| {
            let cap = reg
                .captures(line)
                .unwrap_or_else(|| panic!("failed to parse instruction '{}'", line));
            match &cap[1] {
                "noop" => Noop,
                "addx" => Addx(cap[2].parse().unwrap()),
                x => panic!("got unknown instruction '{}'", x),
            }
        })
        .collect();

    let mut cpu = Cpu::new(&instruction);
    let mut total = 0;
    let mut next_sample_cycle = 20;
    let mut crt = String::with_capacity(41 * 6);
    let mut first = true;
    while cpu.tick() {
        if cpu.cycle == next_sample_cycle {
            total += cpu.reg_x * cpu.cycle as isize;
            next_sample_cycle += 40;
        }

        let crt_x_pos = (cpu.cycle - 1) % 40;

        if crt_x_pos == 0 {
            if first {
                first = false;
            } else {
                crt.push('\n');
            }
        }

        if (crt_x_pos as isize).abs_diff(cpu.reg_x) < 2 {
            crt.push('#');
        } else {
            crt.push('.');
        }
    }

    (total.to_string(), crt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
        "13140"
    );
    solution!(p1, p1_solution, "12540");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
        "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
    );
    solution!(
        p2,
        p2_solution,
        "####.####..##..####.####.#....#..#.####.
#....#....#..#....#.#....#....#..#.#....
###..###..#......#..###..#....####.###..
#....#....#.....#...#....#....#..#.#....
#....#....#..#.#....#....#....#..#.#....
#....####..##..####.####.####.#..#.####."
    );
}
