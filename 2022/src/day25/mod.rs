use self::snafu::Snafu;

mod snafu;

pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 25: Full of Hot Air",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let part1: Snafu<u64> = input
        .lines()
        .map(|line| line.parse::<Snafu<u64>>().unwrap())
        .sum();

    (part1.to_string(), "no yet implemented".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122",
        "2=-1=0"
    );
    solution!(p1, p1_solution, "2-=12=2-2-2-=0012==2");

    // Part 2
    // example!(p2, p2_example_1, "", "0");
    // example!(p2, p2_example_2, "", "0");
    // example!(p2, p2_example_3, "", "0");
    // example!(p2, p2_example_4, "", "0");
    // example!(p2, p2_example_5, "", "0");
    // solution!(p2, p2_solution, "100");
}
