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
    /// Return total number of wins using direct calculation.
    ///
    /// Given a race with time `T` and distance `D`, The distance travelled
    /// after holding the button `x` seconds can be described as `f(x) = x(T-x)`,
    /// which again describes a parabola. We want to find for how many discreet
    /// values of `x` the inequality `f(x) > D` holds. Using the quadratic
    /// equation we can calculate the following inequality.
    ///
    /// `T - sqrt(T^2 - 4D) < x < T + sqrt(T^2 - 4D)`
    ///
    /// Applying the floor and ceiling functions and an extra +1.0 gives
    /// discreet bounds that when subtracted gives number of possible ways we
    /// can win.
    fn num_wins(&self) -> u64 {
        let time: f64 = self.time as f64;
        let distance: f64 = self.distance as f64;

        let lower_bound: u64 =
            f64::floor(0.5 * (time - f64::sqrt(time * time - 4.0 * distance)) + 1.0) as u64;
        let upper_bound: u64 =
            f64::ceil(0.5 * (time + f64::sqrt(time * time - 4.0 * distance))) as u64;

        upper_bound - lower_bound
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    // Part 1
    let (_, races) = parse::parse_races(input.as_ref()).expect("should be valid input");
    let part1: u64 = races.iter().map(|v| v.num_wins()).product();

    // Part 2
    let (_, race) = parse::parse_kerning_race(input.as_ref()).expect("should be valid input");
    let part2: u64 = race.num_wins();

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
