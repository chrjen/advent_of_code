pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 11: Cosmic Expansion",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

use std::collections::HashSet;

use itertools::Itertools;
use std::ops::RangeInclusive;

mod parse;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let galaxies = parse::parse_map(&input);

    // All rows and columns with no galaxies in them.
    let x_gaps = galaxies
        .iter()
        .map(|(x, _)| x)
        .sorted_unstable()
        .unique()
        .tuple_windows()
        .flat_map(|(x0, x1)| (x0 + 1..*x1))
        .collect::<Vec<_>>();

    let y_gaps = galaxies
        .iter()
        .map(|(_, y)| y)
        .sorted_unstable()
        .unique()
        .tuple_windows()
        .flat_map(|(y0, y1)| (y0 + 1..*y1))
        .collect::<Vec<_>>();

    let expand_by = |expansion_rate: usize| {
        let x_gaps = &x_gaps;
        let y_gaps = &y_gaps;
        return move |(x, y): &(usize, usize)| {
            let x_expansion: usize = x_gaps
                .iter()
                .map(|gap_x| ((x > gap_x) as usize) * (expansion_rate - 1))
                .sum();
            let y_expansion: usize = y_gaps
                .iter()
                .map(|gap_y| ((y > gap_y) as usize) * (expansion_rate - 1))
                .sum();
            (x + x_expansion, y + y_expansion)
        };
    };

    // Part 1
    let expanded_universe = galaxies.iter().map(expand_by(2)).collect::<HashSet<_>>();
    // _print_universe(&expanded_universe);

    let part1: usize = expanded_universe
        .iter()
        .tuple_combinations()
        .map(|(&(x0, y0), &(x1, y1))| x0.abs_diff(x1) + y0.abs_diff(y1))
        .sum();

    // Part 2
    let part2: usize = galaxies
        .iter()
        .map(expand_by(1_000_000))
        .collect::<Vec<_>>()
        .iter()
        .tuple_combinations()
        .map(|(&(x0, y0), &(x1, y1))| x0.abs_diff(x1) + y0.abs_diff(y1))
        .sum();

    (part1.to_string(), part2.to_string())
}

fn _print_universe(universe: &HashSet<(usize, usize)>) {
    let (x_bound, y_bound) = _get_bounds(universe.iter());

    for y in y_bound {
        for x in x_bound.clone() {
            let c = match universe.get(&(x, y)) {
                Some(_) => '#',
                None => '.',
            };
            print!("{c}");
        }
        println!()
    }
}

fn _get_bounds<'a, T: 'a, I>(coords: I) -> (RangeInclusive<T>, RangeInclusive<T>)
where
    I: Iterator<Item = &'a (T, T)> + Clone,
    T: Ord + Clone,
{
    let min_x = coords.clone().map(|(x, _)| x).min().unwrap();
    let min_y = coords.clone().map(|(_, y)| y).min().unwrap();
    let max_x = coords.clone().map(|(x, _)| x).max().unwrap();
    let max_y = coords.map(|(_, y)| y).max().unwrap();

    (min_x.clone()..=max_x.clone(), min_y.clone()..=max_y.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        "374"
    );
    example!(
        p1,
        p1_example_2,
        "#...
...#
....
....
.#..",
        "20"
    );
    solution!(p1, p1_solution, "9947476");

    // Part 2
    solution!(p2, p2_solution, "519939907614");
}
