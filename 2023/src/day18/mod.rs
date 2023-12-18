pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 18: Lavaduct Lagoon",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod data;
mod parse;

use self::data::{Colour, Coord, DigStep};
use crossterm::style::Stylize;
use std::collections::HashMap;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let (_, dig_plan) = parse::parse_dig_plan(&input).expect("input should be valid");

    // println!();
    // _print_dig_plan(&dig_plan);
    // println!();

    // Part 1
    let part1 = calc_area(dig_plan.iter().cloned());

    // Part 2
    let dig_steps = dig_plan
        .iter()
        .map(|step| DigStep::from_colour(step.colour));
    let part2 = calc_area(dig_steps);

    (part1.to_string(), part2.to_string())
}

/// Calculate the area based on the shoelace formula.
/// https://en.wikipedia.org/wiki/Shoelace_formula
fn calc_area(dig_steps: impl Iterator<Item = DigStep>) -> usize {
    let (mut x, mut y) = (0, 0);
    let mut area: i64 = 0;
    let mut dist_total = 0;

    for step in dig_steps {
        let (x_offset, y_offset) = step.dir.offset();
        let (x2, y2) = (
            x + x_offset * step.dist as i64,
            y + y_offset * step.dist as i64,
        );
        area += x * y2 - x2 * y;
        dist_total += step.dist;
        (x, y) = (x2, y2);
    }

    // The area does not seem to include the border so we also add the total distance
    // to the area calculation. Lastly we add plus one to fix an off-by-one error
    // that I don't know where comes from, but it works.
    (area.unsigned_abs() as usize + dist_total) / 2 + 1
}

fn _print_dig_plan(dig_plan: &[DigStep]) {
    let mut terrain: HashMap<Coord, Colour> = HashMap::new();
    let (mut x, mut y) = (0, 0);
    for step in dig_plan.iter() {
        let (x_offset, y_offset) = step.dir.offset();
        for _ in 0..step.dist {
            terrain.insert((x, y), step.colour);
            (x, y) = (x + x_offset, y + y_offset);
        }
    }

    let x_min = terrain.keys().map(|(x, _)| *x).min().unwrap_or_default();
    let y_min = terrain.keys().map(|(_, y)| *y).min().unwrap_or_default();
    let x_max = terrain.keys().map(|(x, _)| *x).max().unwrap_or_default();
    let y_max = terrain.keys().map(|(_, y)| *y).max().unwrap_or_default();

    for y in (y_min..=y_max).rev() {
        for x in x_min..=x_max {
            if let Some(&colour) = terrain.get(&(x, y)) {
                print!("{}", "#".with(colour.into()));
            } else {
                print!("{}", ".".dark_grey());
            }
        }
        println!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
        "62"
    );
    solution!(p1, p1_solution, "36679");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
        "952408144115"
    );
    solution!(p2, p2_solution, "88007104020978");
}
