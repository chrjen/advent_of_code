use std::fmt::Display;

pub struct Solver<'a> {
    pub name: &'a str,
    pub input: &'a [u8],
    pub solve: fn(&[u8]) -> (String, String),
}

pub fn from_option<T: Display>(value: Option<T>) -> String {
    match value {
        Some(x) => format!("{}", x),
        None => String::from("no solution for input"),
    }
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
