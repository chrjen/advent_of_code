use std::fmt::Display;

pub type Solver = fn(&[u8]) -> (String, String);

pub struct Solution<'a> {
    pub name: &'a str,
    pub input: &'a [u8],
    pub solve: Solver,
}

pub fn from_option<T: Display>(value: Option<T>) -> String {
    match value {
        Some(x) => format!("{}", x),
        None => String::from("no solution for input"),
    }
}
