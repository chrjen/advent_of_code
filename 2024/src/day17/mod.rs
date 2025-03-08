pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 17: Chronospatial Computer",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod data;
mod parse;

use std::mem;

use data::{Cpu, CpuError};
use itertools::Itertools;

/// The maximum number of ticks a program can make before we
/// considered it as being stuck and terminates it.
const MAX_TICKS: usize = 1000;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let (mut cpu, program) = parse::parse_program(input).expect("input should be valid");
    let program = program.as_slice();

    // println!("Disassembly:");
    // for line in Cpu::disassemble(&program, true)
    //     .expect("program should be valid")
    //     .lines()
    // {
    //     println!("    {line}");
    // }

    // Part 1
    let mut program_output: Vec<u8> = Vec::new();

    for ticks in 0.. {
        if ticks == MAX_TICKS {
            panic!("cpu took too long");
        }
        match cpu.tick(program) {
            Ok(Some(out)) => program_output.push(out),
            Ok(None) => { /* Do nothing. */ }
            Err(CpuError::Halt) => break,
            Err(e) => panic!("got cpu error {e:?}"),
        }
    }

    let part1: String = program_output.iter().map(u8::to_string).join(",");

    // Part 2
    // This solution takes advantage of some key observation about the input to
    // solve part 2. The first being that the program consists of a single loop
    // that produce a single output per iteration. Registers B and C does not
    // maintain their states after a loop iteration and are instead overwritten
    // and can be determined from the value in register A at the start of each loop.
    // This means it does not matter what we initialise registers B or C to.
    // The program only has a single `ADV 3` instruction modifying register A and only
    // a single `BDV` or `CDV` instruction. This means that in a single loop the last
    // 10 bits of register A is the only thing that determines the output. After each
    // loop the content of register A is the same except shifted to the right 3 bits.
    //
    // Now, we know the end state of register A has to be 0 or otherwise the program
    // would loop forever. So if we initialise register A to be 0 and then work
    // backwards trying out all combination for the 3 bit that would have been shifted
    // out, there is only 8 combinations after all, we can figure out all possible
    // values for the first or leftmost 3 bits of the solution. Only some 3 bit combinations
    // cause the program to output the last byte of its program, so we only keep those.
    // Once we know the possible values for the first 3 bits we repeat with the second
    // to last byte, shifting in another set of 3 bits to find all solutions that
    // produce the last two bytes as output. If we keep doing this, shifting in
    // a new set of 3 bits each time we eventually get a list of solutions that when
    // initialising register A with will make the program produce itself as an output.
    // We then only need to pick the first, or lowest, one as the final answer.
    fn do_part2(program: &[u8]) -> Result<Option<u64>, CpuError> {
        let mut next: Vec<u64> = Vec::new();
        let mut current: Vec<u64> = vec![0];

        for byte in program.iter().rev().copied() {
            for a_left in current.drain(..) {
                for a_right in 0..8_u64 {
                    let a = (a_left << 3) | a_right;
                    let mut ticks = 0..;
                    let mut cpu = Cpu::new(a, 0, 0);

                    let output = loop {
                        if ticks.next().unwrap() == MAX_TICKS {
                            panic!("cpu took too long");
                        }
                        match cpu.tick(program) {
                            Ok(Some(output)) => break output,
                            Ok(None) => { /* Do nothing. */ }
                            Err(err) => panic!("part2: program exited too early, got {err:?}"),
                        }
                    };

                    if output == byte {
                        next.push(a);
                    }
                }
            }

            mem::swap(&mut next, &mut current);
        }

        Ok(current.first().copied())
    }

    let part2: String = match do_part2(program) {
        Ok(Some(initial_a)) => initial_a.to_string(),
        Ok(None) => "No solution".to_string(),
        Err(err) => format!("!cpu exception: {err:?}"),
    };

    (part1.to_string(), part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
        "4,6,3,5,6,3,5,2,1,0"
    );
    solution!(p1, p1_solution, "3,4,3,1,7,6,5,6,0");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
        "117440"
    );
    solution!(p2, p2_solution, "109019930331546");
}
