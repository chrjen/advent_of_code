pub const SOLUTION: common::Solution = common::Solution {
    name: "Day X: Template",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    (0.to_string(), 0.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    // example!(p1, p1_example_1, "", "0");
    // example!(p1, p1_example_2, "", "0");
    // example!(p1, p1_example_3, "", "0");
    // example!(p1, p1_example_4, "", "0");
    // example!(p1, p1_example_5, "", "0");
    // solution!(p1, p1_solution, "100");

    // Part 2
    // example!(p2, p2_example_1, "", "0");
    // example!(p2, p2_example_2, "", "0");
    // example!(p2, p2_example_3, "", "0");
    // example!(p2, p2_example_4, "", "0");
    // example!(p2, p2_example_5, "", "0");
    // solution!(p2, p2_solution, "100");
}
