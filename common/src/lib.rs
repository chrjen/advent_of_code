use std::fmt::Display;

pub fn from_option<T: Display>(value: Option<T>) -> String {
    match value {
        Some(x) => format!("{}", x),
        None => String::from("no solution for input"),
    }
}
