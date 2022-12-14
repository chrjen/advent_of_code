mod parse;
mod sand;

use std::collections::HashMap;

use sand::Tile;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 14: Regolith Reservoir",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let paths: Vec<Vec<sand::Loc>> = parse::rock_paths_parser(&input)
        .unwrap_or_else(|err| panic!("parsing error: {err}"))
        .1;

    // Setup the world tiles from the paths in the input.
    let mut tiles = HashMap::new();
    for mut path in paths.iter().map(|p| p.iter()) {
        let start = *path.next().unwrap();
        tiles.insert(start, Tile::Rock);

        let mut rock = start;
        for &end in path.by_ref() {
            let (dx, dy) = ((end.x - rock.x).signum(), (end.y - rock.y).signum());

            // x-direction.
            while rock.x != end.x {
                rock.x += dx;
                tiles.insert(rock, Tile::Rock);
            }

            // y-direction.
            while rock.y != end.y {
                rock.y += dy;
                tiles.insert(rock, Tile::Rock);
            }

            rock = end;
            tiles.insert(rock, Tile::Rock);
        }
    }

    let mut world = sand::World::new(
        sand::Loc { x: 500, y: 0 },
        tiles.keys().map(|loc| loc.y).max().unwrap() + 2,
        tiles,
    );

    // Part 1.
    while world.add_sand_grain().y < world.floor() - 1 {}
    let part1 = world.count() - 1;

    // // Uncomment to print out world.
    // println!("{}", world);

    // Part 2.
    while world.add_sand_grain() != world.source() {}
    let part2 = world.count();

    // // Uncomment to print out world using ANSI colours.
    // println!("{}", world.display_colour());

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
        "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
        "24"
    );
    solution!(p1, p1_solution, "768");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
        "93"
    );
    solution!(p2, p2_solution, "26686");
}
