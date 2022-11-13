pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 1: Not Quite Lisp",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let mut floor: isize = 0;
    let mut basement_idx: Option<usize> = None;

    for (i, c) in input.iter().enumerate() {
        match *c as char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }

        if basement_idx.is_none() && floor < 0 {
            basement_idx = Some(i + 1);
        }
    }

    (floor.to_string(), common::from_option(basement_idx))
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(p1, p1_example_1, "(())", "0");
    example!(p1, p1_example_2, "()()", "0");
    example!(p1, p1_example_3, "))(((((", "3");
    example!(p1, p1_example_4, "())", "-1");
    example!(p1, p1_example_5, "))(", "-1");
    example!(p1, p1_example_6, ")))", "-3");
    example!(p1, p1_example_7, ")())())", "-3");
    solution!(p1, p1_solution, "232");

    // Part 2
    example!(p2, p2_example_1, ")", "1");
    example!(p2, p2_example_2, "()())", "5");
    example!(p2, p2_example_3, ")()", "1");
    solution!(p2, p2_solution, "1783");
}
