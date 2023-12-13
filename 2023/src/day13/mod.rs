use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use itertools::Itertools;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 13: Point of Incidence",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod parse;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let grounds = parse::parse_input(&input);

    let part1: u32 = grounds.iter().map(summarise(0)).sum();
    let part2: u32 = grounds.iter().map(summarise(1)).sum();

    (part1.to_string(), part2.to_string())
}

fn summarise(smudges: usize) -> impl Fn(&HashSet<(u32, u32)>) -> u32 {
    move |ground: &HashSet<(u32, u32)>| -> u32 {
        let (mirror_x, mirror_y) = find_mirror(ground, smudges);
        // _print_ground(&ground, (mirror_x.map(|x| x + 1), mirror_y.map(|y| y + 1)));
        // println!();
        mirror_x.unwrap_or(0) + mirror_y.map(|y| y * 100).unwrap_or(0)
    }
}

fn find_mirror(ground: &HashSet<(u32, u32)>, smudges: usize) -> (Option<u32>, Option<u32>) {
    let (x_bound, y_bound) = get_bounds(ground.iter());
    let empty_set = HashSet::new();

    let cols: HashMap<u32, HashSet<_>> = ground
        .iter()
        .copied()
        .sorted_by_key(|(x, _)| *x)
        .group_by(|(x, _)| *x)
        .into_iter()
        .map(|(key, group)| (key, group.map(|(_, y)| y).collect()))
        .collect();

    let rows: HashMap<u32, HashSet<_>> = ground
        .iter()
        .copied()
        .sorted_by_key(|(_, y)| *y)
        .group_by(|(_, y)| *y)
        .into_iter()
        .map(|(key, group)| (key, group.map(|(x, _)| x).collect()))
        .collect();

    let count_difference_cols = |(x0, x1): (u32, u32)| -> usize {
        let col0 = cols.get(&x0).unwrap_or(&empty_set);
        let col1 = cols.get(&x1).unwrap_or(&empty_set);
        col0.symmetric_difference(col1).count()
    };

    let count_difference_rows = |(y0, y1): (u32, u32)| -> usize {
        let row0 = rows.get(&y0).unwrap_or(&empty_set);
        let row1 = rows.get(&y1).unwrap_or(&empty_set);
        row0.symmetric_difference(row1).count()
    };

    let vertical_candidates: Vec<_> = x_bound
        .clone()
        .tuple_windows()
        .flat_map(|(x0, x1)| (count_difference_cols((x0, x1)) <= smudges).then_some(x1))
        .collect();

    let horizontal_candidates: Vec<_> = y_bound
        .clone()
        .tuple_windows()
        .flat_map(|(y0, y1)| (count_difference_rows((y0, y1)) <= smudges).then_some(y1))
        .collect();

    for x in vertical_candidates {
        let smudge_count: usize = (*x_bound.start()..x)
            .rev()
            .zip(x..=*x_bound.end())
            .map(count_difference_cols)
            .sum();

        if smudge_count == smudges {
            return (Some(x - 1), None);
        }
    }

    for y in horizontal_candidates {
        let smudge_count: usize = (*y_bound.start()..y)
            .rev()
            .zip(y..=*y_bound.end())
            .map(count_difference_rows)
            .sum();

        if smudge_count == smudges {
            return (None, Some(y - 1));
        }
    }

    // No solution.
    (None, None)
}

fn _print_ground(universe: &HashSet<(u32, u32)>, mirror: (Option<u32>, Option<u32>)) {
    let (x_bound, y_bound) = get_bounds(universe.iter());
    let (mirror_x, mirror_y) = mirror;

    for y in y_bound {
        if mirror_y.is_some_and(|my| my == y) {
            x_bound.clone().for_each(|_| print!("\x1b[33m─\x1b[0m"));
            println!();
        }
        for x in x_bound.clone() {
            if mirror_x.is_some_and(|mx| mx == x) {
                print!("\x1b[33m│\x1b[0m");
            }
            let c = match universe.get(&(x, y)) {
                Some(_) => '#',
                None => '.',
            };
            print!("{c}");
        }
        println!()
    }
}

fn get_bounds<'a, T: 'a, I>(coords: I) -> (RangeInclusive<T>, RangeInclusive<T>)
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
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        "405"
    );
    solution!(p1, p1_solution, "32371");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        "400"
    );
    solution!(p2, p2_solution, "37416");
}
