use regex::Regex;

pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 5: Supply Stacks",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

fn read_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();
    let num_stacks;
    {
        let mut lines = input.lines();
        let first = lines.next().unwrap();
        num_stacks = (first.len() + 1) / 4;
        for _ in 0..num_stacks {
            stacks.push(Vec::with_capacity(30));
        }
    }

    for line in input.lines().rev().skip(1) {
        let mut chars = line.chars();
        for stack in stacks.iter_mut().take(num_stacks) {
            chars.next();
            let c = chars.next().unwrap();
            if c != ' ' {
                stack.push(c);
            }
            chars.next();
            chars.next();
        }
    }

    stacks
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let reg_instruction = Regex::new(r"move (\d+) from (\w) to (\w)").unwrap();

    let (stack_drawing, instructions) = {
        let mut tmp = input.split("\n\n");
        (tmp.next().unwrap(), tmp.next().unwrap())
    };

    // Setup stack and instruction data structures.
    let stacks = read_stacks(stack_drawing);
    let instructions: Vec<_> = instructions
        .lines()
        .map(|line| {
            let cap = reg_instruction.captures(line).unwrap();
            (
                cap[1].parse::<usize>().unwrap(),
                cap[2].parse::<usize>().unwrap() - 1,
                cap[3].parse::<usize>().unwrap() - 1,
            )
        })
        .collect();

    // Part 1.
    let mut part_1_stacks = stacks.clone();
    for &(num, from, to) in instructions.iter() {
        for _ in 0..num {
            let tmp = part_1_stacks[from].pop().unwrap();
            part_1_stacks[to].push(tmp);
        }
    }

    // Part 2.
    let mut part_2_stacks = stacks;
    for &(num, from, to) in instructions.iter() {
        let idx = part_2_stacks[from].len() - num;
        for _ in 0..num {
            let c = part_2_stacks[from].remove(idx);
            part_2_stacks[to].push(c);
        }
    }

    (
        part_1_stacks.iter().map(|v| v.last().unwrap()).collect(),
        part_2_stacks.iter().map(|v| v.last().unwrap()).collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        "CMZ"
    );
    solution!(p1, p1_solution, "HBTMTBSDC");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        "MCD"
    );
    solution!(p2, p2_solution, "PQTJRSHWS");
}
