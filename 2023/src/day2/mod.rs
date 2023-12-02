pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 2: Cube Conundrum",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod parse;

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

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    solve_for(input, 12, 13, 14)
}

fn solve_for(input: &str, red: u32, green: u32, blue: u32) -> (String, String) {
    let mut part1_sum = 0;
    let mut part2_sum = 0;
    for line in input.lines() {
        let (_, game) = parse::parse_game(line).unwrap();

        // Part 1
        let possible = game.rounds.iter().fold(true, |acc, round| {
            acc && round.red.map_or(true, |count| count <= red)
                && round.green.map_or(true, |count| count <= green)
                && round.blue.map_or(true, |count| count <= blue)
        });
        if possible {
            part1_sum += game.id;
        }

        // Part 2
        let (min_red, min_green, min_blue) =
            game.rounds.iter().fold((0, 0, 0), |(r, g, b), round| {
                (
                    round.red.map_or(r, |v| v.max(r)),
                    round.green.map_or(g, |v| v.max(g)),
                    round.blue.map_or(b, |v| v.max(b)),
                )
            });
        part2_sum += min_red * min_green * min_blue
    }

    (part1_sum.to_string(), part2_sum.to_string())
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
