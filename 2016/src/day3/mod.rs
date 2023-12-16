pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 3: Squares With Three Sides",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

use itertools::Itertools;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    // Part 1.
    let part1 = input
        .lines()
        .map(|line| {
            let mut col = line.split_whitespace();
            (
                col.next().unwrap().parse::<u32>().unwrap(),
                col.next().unwrap().parse::<u32>().unwrap(),
                col.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .filter(|&(a, b, c)| a + b > c && a + c > b && b + c > a)
        .count();

    // Part 2.
    let mut cols = [Vec::new(), Vec::new(), Vec::new()];
    for line in input.lines() {
        let mut col = line.split_whitespace();
        cols[0].push(col.next().unwrap().parse::<u32>().unwrap());
        cols[1].push(col.next().unwrap().parse::<u32>().unwrap());
        cols[2].push(col.next().unwrap().parse::<u32>().unwrap());
    }

    let part2 = cols[0]
        .iter()
        .chain(cols[1].iter())
        .chain(cols[2].iter())
        .tuples::<(_, _, _)>()
        .filter(|&(&a, &b, &c)| a + b > c && a + c > b && b + c > a)
        .count();

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(p1, p1_example_1, "  5  10  25", "0");
    example!(p1, p1_example_2, "  5  10  12", "1");
    example!(
        p1,
        p1_example_3,
        "  5  10  25
      5  10  12",
        "1"
    );
    solution!(p1, p1_solution, "862");

    // Part 2
    solution!(p2, p2_solution, "1577");
}
