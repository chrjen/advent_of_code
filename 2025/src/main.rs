mod day1;
mod day2;

use day1::skeleton::Solver;

use miette::Result;

fn main() -> Result<()> {
    println!("{}", day1::Solution::title());
    let parts = day1::Solution::initial(day1::Solution::input());

    println!("Part 1: {}", parts.part1());
    println!("Part 2: {}", parts.part2());

    Err(day2::run().unwrap_err())?;
    Ok(())
}
