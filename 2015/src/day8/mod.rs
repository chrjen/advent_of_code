pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 8: Matchsticks",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        // All characters minus surrounding quotes.
        let mut chars: Vec<char> = line.chars().skip(1).collect();
        chars.pop();
        part1 += 2;
        part2 += 4;

        let mut chars = chars.iter();
        while let Some(ch) = chars.next() {
            if *ch == '\\' {
                match chars.next() {
                    Some('\\') | Some('"') => {
                        part1 += 1;
                        part2 += 2;
                    }
                    Some('x') => {
                        chars.nth(1);
                        part1 += 3;
                        part2 += 1;
                    }
                    Some(_) => {}
                    None => panic!("unexpected end of string!"),
                }
            }
        }
    }

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(p1, p1_example_1, r#""""#, "2");
    example!(p1, p1_example_2, r#""abc""#, "2");
    example!(p1, p1_example_3, r#""aaa\"aaa""#, "3");
    example!(p1, p1_example_4, r#""\x27""#, "5");
    solution!(p1, p1_solution, "1342");

    // Part 2
    example!(p2, p2_example_1, r#""""#, "4");
    example!(p2, p2_example_2, r#""abc""#, "4");
    example!(p2, p2_example_3, r#""aaa\"aaa""#, "6");
    example!(p2, p2_example_4, r#""\x27""#, "5");
    solution!(p2, p2_solution, "2074");
}
