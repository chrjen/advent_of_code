use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Output {
    UInt(u128),
    Int(i128),
    String(Box<str>),
}

impl From<u128> for Output {
    fn from(value: u128) -> Self {
        Self::UInt(value)
    }
}

impl From<u64> for Output {
    fn from(value: u64) -> Self {
        Self::UInt(value.into())
    }
}

impl From<u32> for Output {
    fn from(value: u32) -> Self {
        Self::UInt(value.into())
    }
}

impl From<u16> for Output {
    fn from(value: u16) -> Self {
        Self::UInt(value.into())
    }
}

impl From<u8> for Output {
    fn from(value: u8) -> Self {
        Self::UInt(value.into())
    }
}

impl From<i128> for Output {
    fn from(value: i128) -> Self {
        Self::Int(value)
    }
}

impl From<i64> for Output {
    fn from(value: i64) -> Self {
        Self::Int(value.into())
    }
}

impl From<i32> for Output {
    fn from(value: i32) -> Self {
        Self::Int(value.into())
    }
}

impl From<i16> for Output {
    fn from(value: i16) -> Self {
        Self::Int(value.into())
    }
}

impl From<i8> for Output {
    fn from(value: i8) -> Self {
        Self::Int(value.into())
    }
}

impl From<Box<str>> for Output {
    fn from(value: Box<str>) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Output {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl From<String> for Output {
    fn from(value: String) -> Self {
        Self::String(value.into())
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Output::UInt(x) => write!(f, "{}", x),
            Output::Int(x) => write!(f, "{}", x),
            Output::String(x) => write!(f, "{}", x),
        }
    }
}

pub trait Solver {
    fn title() -> &'static str;
    fn input() -> &'static [u8];
    fn initial(input: &[u8]) -> Box<dyn PartSolver>;
}

pub trait PartSolver {
    fn part1(&self) -> Output;
    fn part2(&self) -> Output;
}
