pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 16: The Floor Will Be Lava",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod data;
mod parse;

use std::collections::HashMap;

use rayon::prelude::*;

use self::data::{Contraption, Direction, Mirror};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    // Part 1
    let contraption = parse::parse_contraption(&input);
    let beam = contraption.fire_beam(((1, 1), Direction::East));
    let part1 = beam.len();

    // println!();
    // _print_contraption(&contraption);
    // println!();

    // Part 2
    let top_edge = contraption.x_bound.clone().into_par_iter().map(|x| {
        let y = contraption.y_bound.start;
        let beam = contraption.fire_beam(((x, y), Direction::South));
        beam.len()
    });
    let bottom_edge = contraption.x_bound.clone().into_par_iter().map(|x| {
        let y = contraption.y_bound.end - 1;
        let beam = contraption.fire_beam(((x, y), Direction::North));
        beam.len()
    });
    let left_edge = contraption.y_bound.clone().into_par_iter().map(|y| {
        let x = contraption.x_bound.start;
        let beam = contraption.fire_beam(((x, y), Direction::East));
        beam.len()
    });
    let right_edge = contraption.y_bound.clone().into_par_iter().map(|y| {
        let x = contraption.x_bound.end - 1;
        let beam = contraption.fire_beam(((x, y), Direction::West));
        beam.len()
    });

    let part2 = top_edge
        .chain(bottom_edge)
        .chain(left_edge)
        .chain(right_edge)
        .max()
        .expect("should return at least one value");

    (part1.to_string(), part2.to_string())
}

fn _print_contraption(ctt: &Contraption) {
    for y in ctt.y_bound.clone() {
        for x in ctt.x_bound.clone() {
            let c = match ctt.mirrors.get(&(x, y)) {
                Some(Mirror::Left) => '\\',
                Some(Mirror::Right) => '/',
                Some(Mirror::SplitV) => '|',
                Some(Mirror::SplitH) => '-',
                None => '.',
            };
            print!("{c}");
        }
        println!()
    }
}

fn _print_contraption_coords(ctt: &Contraption, coords: &HashMap<data::Coord, data::Direction>) {
    for y in ctt.y_bound.clone() {
        for x in ctt.x_bound.clone() {
            if let Some(dir) = coords.get(&(x, y)) {
                match dir {
                    data::Direction::North => print!("^"),
                    data::Direction::West => print!("<"),
                    data::Direction::South => print!("V"),
                    data::Direction::East => print!(">"),
                }
                continue;
            }
            let c = match ctt.mirrors.get(&(x, y)) {
                Some(Mirror::Left) => '\\',
                Some(Mirror::Right) => '/',
                Some(Mirror::SplitV) => '|',
                Some(Mirror::SplitH) => '-',
                None => '.',
            };
            print!("{c}");
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
        r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#,
        "46"
    );
    solution!(p1, p1_solution, "7951");

    // Part 2
    example!(
        p2,
        p2_example_1,
        r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#,
        "51"
    );
    solution!(p2, p2_solution, "8148");
}
