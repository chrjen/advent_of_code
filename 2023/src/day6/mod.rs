pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 6: Wait For It",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod parse;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn num_wins(&self) -> u64 {
        let time: f64 = self.time as f64;
        let distance: f64 = self.distance as f64;
        let lower_bound: u64 =
            f64::ceil(0.5 * (time - f64::sqrt(time * time - 4.0 * distance)) + 0.0001) as u64;
        let upper_bound: u64 =
            f64::ceil(0.5 * (f64::sqrt(time * time - 4.0 * distance) + time)) as u64;

        upper_bound - lower_bound
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    // Part 1
    let (_, races) = parse::parse_races(input.as_ref()).expect("should be valid input");
    let part1: u64 = races.iter().map(|v| v.num_wins()).product();

    let (_, race) = parse::parse_kerning_race(input.as_ref()).expect("should be valid input");
    let part2: u64 = race.num_wins();
    dbg!(&race);

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
        "Time:      7  15   30
Distance:  9  40  200",
        "288"
    );
    solution!(p1, p1_solution, "1710720");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "Time:      7  15   30
Distance:  9  40  200",
        "71503"
    );
    solution!(p2, p2_solution, "35349468");
}
