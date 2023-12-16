mod world;

use world::World;

pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 24: Blizzard Basin",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let world = World::from_str(&input);
    let mut step = 0;

    // Part 1.
    world.walk(&mut step, world.start(), world.end());
    let part1 = step;

    // Part 2.
    world.walk(&mut step, world.end(), world.start());
    world.walk(&mut step, world.start(), world.end());
    let part2 = step;

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#",
        "18"
    );
    solution!(p1, p1_solution, "251", ignore = "takes too long");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#",
        "54"
    );
    solution!(p2, p2_solution, "758", ignore = "takes too long");
}
