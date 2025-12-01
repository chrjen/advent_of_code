mod day1;
mod day_ex1;
mod day_ex2;

use common_v2::prelude::*;

use miette::Result;

fn main() -> Result<()> {
    println!("{}", day1::Solution::title());
    let parts = day1::Solution::initial(day1::Solution::input());

    println!("Part 1: {}", parts.part1());
    println!("Part 2: {}", parts.part2());

    Ok(())
}
