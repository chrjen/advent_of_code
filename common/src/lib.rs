use std::fmt::Display;

pub type Solver = fn(&[u8]) -> (String, String);

pub struct Solution {
    pub name: &'static str,
    pub input: &'static [u8],
    pub solve: Solver,
}

#[macro_export]
macro_rules! wrap_solve_fn {
    ($func_name:path) => {
        |input: &[u8]| -> (String, String) {
            let (part1, part2) = $func_name(input);
            (
                <dyn std::fmt::Display>::to_string(&part1),
                <dyn std::fmt::Display>::to_string(&part2),
            )
        }
    };
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

// Re-export so that macros using this macro does not have to also add `paste`
// as a dependency.
pub use paste::paste;

#[macro_export]
macro_rules! part1_tests {
    ($($any:tt)*) => { $crate::part_tests![p1; test; $($any)*]; }
}

#[macro_export]
macro_rules! part2_tests {
    ($($any:tt)*) => { $crate::part_tests![p2; test; $($any)*]; }
}

#[macro_export]
macro_rules! part_tests {
    ($part:ident; $prefix:ident;) => {};
    (p1; $prefix:ident; ($name:ident, $input:literal, $output:literal $(, $attr:meta)*) $(, $rest:tt)* $(,)?) => {
        $crate::paste! {
            #[test]
            $(#[$attr])*
            fn [<p1_ $prefix _ $name>]() {
                println!("input: {}", $input);
                let (result, _) = solve(str::as_bytes($input));
                assert_eq!(result, $output);
            }
        }

        $crate::part_tests!(p1; $prefix; $($rest),*);
    };
    (p2; $prefix:ident; ($name:ident, $input:literal, $output:literal $(, $attr:meta)*) $(, $rest:tt)* $(,)?) => {
        $crate::paste! {
            #[test]
            $(#[$attr])*
            fn [<p2_ $prefix _ $name>]() {
                println!("input: {}", $input);
                let (_, result) = solve(str::as_bytes($input));
                assert_eq!(result, $output);
            }
        }

        $crate::part_tests!(p2; $prefix; $($rest),*);
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
