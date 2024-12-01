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
    fn find(
        it: impl Iterator<Item = (u32, u32)>,
        smudges: usize,
        bound: RangeInclusive<u32>,
    ) -> Option<u32> {
        let empty_set = HashSet::new();

        let sets: HashMap<u32, HashSet<_>> = it
            .sorted_by_key(|(u, _)| *u)
            .group_by(|(u, _)| *u)
            .into_iter()
            .map(|(key, group)| (key, group.map(|(_, v)| v).collect()))
            .collect();

        let count_difference_sets = |(v0, v1): (u32, u32)| -> usize {
            let set0 = sets.get(&v0).unwrap_or(&empty_set);
            let set1 = sets.get(&v1).unwrap_or(&empty_set);
            set0.symmetric_difference(set1).count()
        };

        let candidates: Vec<_> = bound
            .clone()
            .tuple_windows()
            .flat_map(|(v0, v1)| (count_difference_sets((v0, v1)) <= smudges).then_some(v1))
            .collect();

        for v in candidates {
            let smudge_count: usize = (*bound.start()..v)
                .rev()
                .zip(v..=*bound.end())
                .map(count_difference_sets)
                .sum();

            if smudge_count == smudges {
                return Some(v - 1);
            }
        }

        None
    }

    let (x_bound, y_bound) = get_bounds(ground.iter());

    // Calls `find()` on columns first, then rows. (x, y) is simply swapped when
    // doing the rows to keep as much logic as possible the same. `find()` is
    // the real implementation.
    let mirror_x = find(ground.iter().copied(), smudges, x_bound);
    let mirror_y = find(
        ground.iter().copied().map(|(x, y)| (y, x)),
        smudges,
        y_bound,
    );

    (mirror_x, mirror_y)
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

fn get_bounds<'a, T, I>(coords: I) -> (RangeInclusive<T>, RangeInclusive<T>)
where
    I: Iterator<Item = &'a (T, T)> + Clone,
    T: Ord + Clone + 'a,
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
