pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 3: Mull It Over",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod data;
mod parse;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    // Part 1
    let (_, instructions) = parse::parse_instructions(input).expect("input should be valid");
    let part1: u32 = instructions.iter().map(data::Mul::eval).sum();

    // Part 1
    let (_, instructions) =
        parse::parse_instructions_with_dont(input).expect("input should be valid");
    let part2: u32 = instructions.iter().map(data::Mul::eval).sum();

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
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        "161"
    );
    example!(p1, p1_example_2, "mul(4*", "0");
    example!(p1, p1_example_3, "mul(6,9!", "0");
    example!(p1, p1_example_4, "?(12,34)", "0");
    example!(
        p1,
        p1_example_5,
        "@~don't()mul(683,461) >,~select()what()};<mul(848,589)!#{$$:,#mul(597,936)]!how();)",
        "1373127"
    );
    solution!(p1, p1_solution, "165225049");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        "48"
    );
    example!(
        p2,
        p2_example_2,
        "(mul(232,619)don't()^mul(406,760)mul(400,865)%",
        "143608"
    );
    solution!(p2, p2_solution, "108830766");
}
