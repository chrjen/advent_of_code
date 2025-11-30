mod day1;
mod day2;

pub mod skeleton;

use std::process::Termination;

use skeleton::Solver;

use miette::{Diagnostic, Result};

use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum DataStoreError {
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

fn main() -> Result<()> {
    println!("{}", day1::Solution::title());
    let parts = day1::Solution::initial(day1::Solution::input());

    println!("Part 1: {}", parts.part1());
    println!("Part 2: {}", parts.part2());

    fn kek() -> Result<(), DataStoreError> {
        Err(DataStoreError::Unknown)
    }

    fn foo() -> Result<()> {
        kek()?;
        Ok(())
    }

    foo().report();

    Err(day2::run().unwrap_err())?;
    Ok(())
}

fn kekky() {}
