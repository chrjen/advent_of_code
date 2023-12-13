use std::{collections::HashSet, ops::RangeInclusive};

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

    let mut part1 = 0;
    for ground in grounds {
        let (x_bound, y_bound) = get_bounds(ground.iter());

        let vertical_candidates: Vec<_> = x_bound
            .clone()
            .tuple_windows()
            .flat_map(|(x0, x1)| {
                let col0 = y_bound
                    .clone()
                    .flat_map(|y| ground.get(&(x0, y)).map(|(_, y)| y))
                    .collect::<Vec<_>>();
                let col1 = y_bound
                    .clone()
                    .flat_map(|y| ground.get(&(x1, y)).map(|(_, y)| y))
                    .collect::<Vec<_>>();

                if col0 == col1 {
                    Some(x1)
                } else {
                    None
                }
            })
            .collect();

        let horizontal_candidates: Vec<_> = y_bound
            .clone()
            .tuple_windows()
            .flat_map(|(y0, y1)| {
                let row0 = x_bound
                    .clone()
                    .flat_map(|x| ground.get(&(x, y0)).map(|(x, _)| x))
                    .collect::<Vec<_>>();
                let row1 = x_bound
                    .clone()
                    .flat_map(|x| ground.get(&(x, y1)).map(|(x, _)| x))
                    .collect::<Vec<_>>();

                if row0 == row1 {
                    Some(y1)
                } else {
                    None
                }
            })
            .collect();

        for x in vertical_candidates {
            // dbg!(x);
            let is_mirror = (*x_bound.start()..x)
                .rev()
                .zip(x..=*x_bound.end())
                .all(|(x0, x1)| {
                    // dbg!((x0, x1));
                    let col0 = y_bound
                        .clone()
                        .flat_map(|y| ground.get(&(x0, y)).map(|(_, y)| y))
                        .collect::<Vec<_>>();
                    let col1 = y_bound
                        .clone()
                        .flat_map(|y| ground.get(&(x1, y)).map(|(_, y)| y))
                        .collect::<Vec<_>>();

                    col0 == col1
                });

            if is_mirror {
                part1 += x - 1;
                println!("Found mirror at x={}", x - 1);
                _print_ground(&ground);
                println!();
                break;
            }
        }

        for y in horizontal_candidates {
            let is_mirror = (*y_bound.start()..y)
                .rev()
                .zip(y..=*y_bound.end())
                .all(|(y0, y1)| {
                    let row0 = x_bound
                        .clone()
                        .flat_map(|x| ground.get(&(x, y0)).map(|(x, _)| x))
                        .collect::<Vec<_>>();
                    let row1 = x_bound
                        .clone()
                        .flat_map(|x| ground.get(&(x, y1)).map(|(x, _)| x))
                        .collect::<Vec<_>>();

                    row0 == row1
                });

            if is_mirror {
                part1 += (y - 1) * 100;
                println!("Found mirror at y={}", y - 1);
                _print_ground(&ground);
                println!();
                break;
            }
        }
    }

    (part1.to_string(), 0.to_string())
}

fn _print_ground(universe: &HashSet<(u32, u32)>) {
    let (x_bound, y_bound) = get_bounds(universe.iter());

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
    // example!(p2, p2_example_1, "", "0");
    // example!(p2, p2_example_2, "", "0");
    // example!(p2, p2_example_3, "", "0");
    // example!(p2, p2_example_4, "", "0");
    // example!(p2, p2_example_5, "", "0");
    // solution!(p2, p2_solution, "100");
}
