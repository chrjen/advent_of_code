pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 15: Warehouse Woes",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod data;
mod parse;

use std::collections::{HashMap, HashSet};

use data::{Direction, Tile};
use nalgebra::Vector2;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let (map_original, directions) = parse::parse_input(input).expect("input should be valid");

    // Part 1
    let mut map = map_original.clone();
    let max_x = map.keys().map(|v| v.x).max().unwrap_or(0);
    let max_y = map.keys().map(|v| v.y).max().unwrap_or(0);

    let (robot_pos, _) = map
        .iter()
        .find(|(_, tile)| **tile == Tile::Robot)
        .expect("input should contain at least one robot");
    let mut robot_pos = *robot_pos;

    for dir in directions.iter().copied() {
        let mut current_pos = robot_pos;

        let is_free_space = loop {
            current_pos += dir.to_vec();
            match map.get(&current_pos) {
                Some(Tile::Wall) => break false,
                Some(Tile::Box) | Some(Tile::BoxLeft) | Some(Tile::BoxRight) => continue,
                Some(Tile::Robot) => panic!("robot got destroyed by a second robot"),
                None => break true,
            }
        };

        if is_free_space {
            loop {
                let prev_pos = current_pos - dir.to_vec();

                if let Some(prev_pos) = map.remove(&prev_pos) {
                    map.insert(current_pos, prev_pos);
                }

                if prev_pos == robot_pos {
                    robot_pos = current_pos;
                    break;
                } else {
                    current_pos = prev_pos;
                }
            }
        }
    }

    let mut part1 = 0;
    for y in 0..=max_y {
        for x in 0..=max_x {
            match map.get(&Vector2::new(x, y)) {
                Some(Tile::Box) | Some(Tile::BoxLeft) => part1 += 100 * y + x,
                _ => {}
            }
        }
    }

    // Part 2
    let scale = |pos| Vector2::new(2, 1).component_mul(pos);
    let mut map: HashMap<Vector2<i32>, Tile> = map_original
        .iter()
        .flat_map(|(pos, tile)| match tile {
            Tile::Wall => [
                (scale(pos), Tile::Wall),
                (scale(pos) + Direction::East.to_vec(), Tile::Wall),
            ]
            .into_iter(),
            Tile::Box => [
                (scale(pos), Tile::BoxLeft),
                (scale(pos) + Direction::East.to_vec(), Tile::BoxRight),
            ]
            .into_iter(),
            Tile::BoxLeft | Tile::BoxRight => [
                (scale(pos), Tile::BoxLeft),
                (scale(pos) + Direction::East.to_vec(), Tile::BoxRight),
            ]
            .into_iter(),
            Tile::Robot => [(scale(pos), Tile::Robot), (scale(pos), Tile::Robot)].into_iter(),
        })
        .collect();
    let max_x = map.keys().map(|v| v.x).max().unwrap_or(0);
    let max_y = map.keys().map(|v| v.y).max().unwrap_or(0);

    let (robot_pos, _) = map
        .iter()
        .find(|(_, tile)| **tile == Tile::Robot)
        .expect("input should contain at least one robot");
    let mut robot_pos = *robot_pos;

    for dir in directions.iter().copied() {
        if let Some(group) = find_group_to_move(&map, robot_pos, dir) {
            for (pos, _) in group.iter() {
                map.remove(pos);
            }
            for (pos, tile) in group.iter() {
                map.insert(pos + dir.to_vec(), *tile);
            }

            robot_pos += dir.to_vec();
        }
    }

    let mut part2 = 0;
    for y in 0..=max_y {
        for x in 0..=max_x {
            match map.get(&Vector2::new(x, y)) {
                Some(Tile::Box) | Some(Tile::BoxLeft) => part2 += 100 * y + x,
                _ => {}
            }
        }
    }

    // for y in 0..=max_y {
    //     for x in 0..=max_x {
    //         match map.get(&Vector2::new(x, y)) {
    //             Some(tile) => print!("{}", tile.into_char()),
    //             None => print!("."),
    //         }
    //     }
    //     println!();
    // }

    (part1.to_string(), part2.to_string())
}

// Uses basic path finding to locate all connected boxes that need to move.
// If a wall is encountered and the group is unable to move then `None` is
// returned instead.
fn find_group_to_move(
    map: &HashMap<Vector2<i32>, Tile>,
    start: Vector2<i32>,
    dir: Direction,
) -> Option<HashSet<(Vector2<i32>, Tile)>> {
    let mut visited: HashSet<(Vector2<i32>, Tile)> = HashSet::new();
    let mut visit: Vec<Vector2<i32>> = Vec::new();

    visited.insert((start, map.get(&start).copied()?));
    visit.push(start);

    while let Some(current) = visit.pop() {
        let current = current + dir.to_vec();

        match map.get(&current) {
            Some(Tile::Wall) => return None,
            Some(Tile::BoxLeft) => {
                let other = current + Direction::East.to_vec();
                visited.insert((other, Tile::BoxRight));
                visited.insert((current, Tile::BoxLeft));
                visit.push(other);
                if dir == Direction::North || dir == Direction::South {
                    visit.push(current);
                }
            }
            Some(Tile::BoxRight) => {
                let other = current + Direction::West.to_vec();
                visited.insert((other, Tile::BoxLeft));
                visited.insert((current, Tile::BoxRight));
                visit.push(other);
                if dir == Direction::North || dir == Direction::South {
                    visit.push(current);
                }
            }
            Some(Tile::Robot) => panic!("robot got destroyed by a second robot"),
            Some(_) => unreachable!(),
            None => {}
        }
    }

    Some(visited)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
        "10092"
    );
    example!(
        p1,
        p1_example_2,
        "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
        "2028"
    );
    solution!(p1, p1_solution, "1563092");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
        "9021"
    );
    example!(
        p2,
        p2_example_2,
        "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^",
        "618"
    );
    solution!(p2, p2_solution, "1582688");
}
