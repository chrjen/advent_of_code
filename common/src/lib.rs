use std::fmt::Display;

pub trait Solver {
    type Part1: Display;
    type Part2: Display;
    /// Given a puzzle input `solve` should return a tuple with a solution to
    /// both part1 and part2, in that order.
    fn solve(input: &[u8]) -> (Self::Part1, Self::Part2);
}

pub fn from_option<T: Display>(value: Option<T>) -> String {
    match value {
        Some(x) => format!("{}", x),
        None => String::from("no solution for input"),
    }
}
