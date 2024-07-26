use std::fmt::Display;

pub type Solver = dyn Fn(&[u8]) -> (Box<dyn Display>, Box<dyn Display>);

pub struct Solution<'a> {
    pub name: &'a str,
    pub input: &'a [u8],
    pub solve: Box<Solver>,
}

pub fn to_solver<O1, O2>(f: fn(&[u8]) -> (O1, O2)) -> Box<Solver>
where
    O1: Display + 'static,
    O2: Display + 'static,
{
    Box::new(
        move |input: &[u8]| -> (Box<dyn Display>, Box<dyn Display>) {
            let (part1, part2) = f(input);
            (Box::new(part1), Box::new(part2))
        },
    )
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
