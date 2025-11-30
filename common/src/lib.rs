use std::fmt::Display;

pub type SolverFn = fn(&[u8]) -> (String, String);

pub struct Solution<'a> {
    pub name: &'a str,
    pub input: &'a [u8],
    pub solve: SolverFn,
}

pub fn from_option<T: Display>(value: Option<T>) -> String {
    match value {
        Some(x) => format!("{}", x),
        None => String::from("no solution for input"),
    }
}

pub trait Solver {
    type Common;

    fn solve_common(input: &[u8]) -> Self::Common;

    fn solve_part1(&self, input: Self::Common) -> String;

    fn solve_part2(&self, input: Self::Common) -> String;
}

// pub fn test_solver(solver: Box<dyn Solver<Common = u32>>) {
//     todo!()
// }

pub trait CommonSolver<S: PartSolver> {
    fn solve_common(input: &[u8]) -> S;
}

pub trait PartSolver {
    fn solve_part1() -> String;
    fn solve_part2() -> String;
}

#[macro_export]
macro_rules! example {
    (p1, $name:ident, $input:literal, $output:literal $(, $attr:meta),*) => {
        #[test]
        $(#[$attr])*
        fn $name() {
            println!("input: {}", $input);
            let (result, _) = solve(str::as_bytes($input));
            assert_eq!(result, $output);
        }
    };
    (p2, $name:ident, $input:literal, $output:literal $(, $attr:meta),*) => {
        #[test]
        $(#[$attr])*
        fn $name() {
            println!("input: {}", $input);
            let (_, result) = solve(str::as_bytes($input));
            assert_eq!(result, $output);
        }
    };
}

#[macro_export]
macro_rules! solution {
    (p1, $name:ident, $output:literal $(, $attr:meta),*) => {
        #[test]
        $(#[$attr])*
        fn $name() {
            let (result, _) = solve(SOLUTION.input);
            assert_eq!(result, $output);
        }
    };
    (p2, $name:ident, $output:literal $(, $attr:meta),*) => {
        #[test]
        $(#[$attr])*
        fn $name() {
            let (_, result) = solve(SOLUTION.input);
            assert_eq!(result, $output);
        }
    };
}
