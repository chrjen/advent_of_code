pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 2: Cube Conundrum",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod parse;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.rounds.iter().fold(true, |acc, round| {
            acc && round.red.map_or(true, |count| count <= MAX_RED)
                && round.green.map_or(true, |count| count <= MAX_GREEN)
                && round.blue.map_or(true, |count| count <= MAX_BLUE)
        })
    }

    fn power(&self) -> u32 {
        let (min_red, min_green, min_blue) =
            self.rounds.iter().fold((0, 0, 0), |(r, g, b), round| {
                (
                    round.red.map_or(r, |v| v.max(r)),
                    round.green.map_or(g, |v| v.max(g)),
                    round.blue.map_or(b, |v| v.max(b)),
                )
            });
        min_red * min_green * min_blue
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let (_, games) = parse::parse_game0(input.as_ref()).unwrap();
    (
        games
            .iter()
            .filter(|game| game.is_possible())
            .map(|game| game.id)
            .sum::<u32>()
            .to_string(),
        games
            .iter()
            .map(|game| game.power())
            .sum::<u32>()
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        "8"
    );
    solution!(p1, p1_solution, "2776");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        "2286"
    );
    solution!(p2, p2_solution, "68638");
}
