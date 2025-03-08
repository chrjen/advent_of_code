use std::{collections::VecDeque, fmt::Debug};

use num::Integer;
use regex::Regex;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 11: Monkey in the Middle",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

type Worry = i64;

#[derive(Clone)]
struct Monkey<F, E, T>
where
    F: Fn(T) -> T,
    E: Fn(T) -> usize,
{
    id: usize,
    items_initial: VecDeque<T>,
    items: VecDeque<T>,
    counter: usize,
    operation: F,
    test: E,
}

impl<F, E, T> std::fmt::Debug for Monkey<F, E, T>
where
    F: Fn(T) -> T,
    E: Fn(T) -> usize,
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("counter", &self.counter)
            .finish()
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let reg = Regex::new(
        r"Monkey (?P<index>\d+):
  Starting items: (?P<items>\d+(?:,\s*\w+)*)
  Operation: new = (?P<op_left>old|\w+)\s*(?P<op>[+\-*/])\s*(?P<op_right>old|\w+)
  Test: divisible by (?P<test>\d+)
    If true: throw to monkey (?P<true>\d+)
    If false: throw to monkey (?P<false>\d+)",
    )
    .unwrap();

    // Perform a single round of monkey action as specified in the task.
    fn do_round<F, E>(monkeys: &mut [Monkey<F, E, Worry>], relief: &Worry, modulus: &Worry)
    where
        F: Fn(Worry) -> Worry,
        E: Fn(Worry) -> usize,
    {
        for i in 0..monkeys.len() {
            // To get two mutable pointers to two elements in the same array, a raw pointer
            // is being used. Others alternatives were not found, sorry.
            let monkey = (&mut monkeys[i]) as *mut Monkey<F, E, Worry>;
            unsafe {
                for item in (*monkey).items.drain(..) {
                    let worry = (((*monkey).operation)(item) / relief).rem_euclid(*modulus);
                    let next_monkey = ((*monkey).test)(worry);
                    (*monkey).counter += 1;

                    if next_monkey == i {
                        panic!("monkey tried throwing it to themselves");
                    }

                    monkeys[next_monkey].items.push_back(worry);
                }
            }
        }
    }

    let mut modulus = 1;
    let mut monkeys: Vec<Monkey<_, _, Worry>> = input
        .split_terminator("\n\n")
        .map(|m| {
            let caps = reg.captures(m).unwrap();
            let items: VecDeque<_> = caps["items"]
                .split_terminator(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();

            Monkey {
                id: caps["index"].parse().unwrap(),
                items_initial: items.clone(),
                items,
                counter: 0,
                operation: {
                    let op = match &caps["op"] {
                        "+" => |a, b| a + b,
                        "-" => |a, b| a - b,
                        "*" => |a, b| a * b,
                        "/" => |a, b| a / b,
                        op => unreachable!("different operator matched {op}"),
                    };

                    match (&caps["op_left"], &caps["op_right"]) {
                        ("old", "old") => {
                            Box::new(move |old: Worry| op(old, old)) as Box<dyn Fn(Worry) -> Worry>
                        }
                        ("old", rhs) => {
                            let rhs = rhs.parse::<Worry>().unwrap();
                            Box::new(move |old| op(old, rhs))
                        }
                        (lhs, "old") => {
                            let lhs = lhs.parse::<Worry>().unwrap();
                            Box::new(move |old| op(lhs, old))
                        }
                        (lhs, rhs) => {
                            let lhs = lhs.parse::<Worry>().unwrap();
                            let rhs = rhs.parse::<Worry>().unwrap();
                            Box::new(move |_old| op(lhs, rhs))
                        }
                    }
                },
                test: {
                    let div: Worry = caps["test"].parse().unwrap();
                    let t: usize = caps["true"].parse().unwrap();
                    let f: usize = caps["false"].parse().unwrap();

                    // Modulus used to avoid the numbers growing too big.
                    // To test divisibility for all the monkeys, only the modulus of the least
                    // common multiple is needed.
                    modulus = modulus.lcm(&div);

                    move |item| {
                        if item % div == 0 { t } else { f }
                    }
                },
            }
        })
        .collect();

    let modulus = modulus;

    // Part 1.
    for _ in 0..20 {
        do_round(monkeys.as_mut_slice(), &3, &modulus);
    }
    monkeys.sort_unstable_by_key(|m| m.counter);
    let part1 = monkeys[monkeys.len() - 1].counter * monkeys[monkeys.len() - 2].counter;

    // Part 2.
    // Manually reset `monkeys` Vec to initial condition. Done due to lack of `clone` method.
    monkeys.sort_unstable_by_key(|m| m.id);
    monkeys.iter_mut().for_each(|m| m.counter = 0);
    monkeys
        .iter_mut()
        .for_each(|m| m.items.clone_from(&m.items_initial));
    for _ in 1..=10_000 {
        do_round(monkeys.as_mut_slice(), &1, &modulus);
    }
    monkeys.sort_unstable_by_key(|m| m.counter);
    let part2 = monkeys.pop().unwrap().counter * monkeys.pop().unwrap().counter;

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
        "10605"
    );
    solution!(p1, p1_solution, "56595");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
        "2713310158"
    );
    solution!(p2, p2_solution, "15693274740");
}
