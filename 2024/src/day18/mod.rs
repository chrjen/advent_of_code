pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 18: RAM Run",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use nalgebra::Vector2;

pub fn solve(input: &[u8]) -> (String, String) {
    solve_(input, 70, 1024)
}

pub fn solve_(input: &[u8], size: i32, count_fallen: usize) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let mut fall_locations: Vec<Vector2<i32>> = Vec::new();
    let start: Vector2<i32> = Vector2::new(0, 0);
    let end: Vector2<i32> = Vector2::new(size, size);

    for line in input.lines() {
        fall_locations.push(
            line.split(',')
                .map(|v| v.parse().expect("coordinate should be a number"))
                .tuples()
                .map(|(x, y)| Vector2::new(x, y))
                .exactly_one()
                .expect("line should only contain two numbers"),
        );
    }

    // Part 1
    let mut next: VecDeque<(u32, Vector2<i32>)> = VecDeque::new();
    let mut visited: HashSet<Vector2<i32>> = HashSet::new();
    next.push_back((0, start));
    visited.insert(start);

    const OFFSETS: &[Vector2<i32>] = &[
        Vector2::new(1, 0),
        Vector2::new(0, 1),
        Vector2::new(-1, 0),
        Vector2::new(0, -1),
    ];

    // Part 1
    'outer: while let Some((cost, current)) = next.pop_front() {
        for offset in OFFSETS {
            let neighbour = current + offset;

            if visited.contains(&neighbour)
                || fall_locations[..count_fallen].contains(&neighbour)
                || !(0..=size).contains(&neighbour.x)
                || !(0..=size).contains(&neighbour.y)
            {
                continue;
            }

            visited.insert(neighbour);
            next.push_back((cost + 1, neighbour));

            if neighbour == end {
                break 'outer;
            }
        }
    }

    let part1 = next.pop_back().map(|v| v.0);

    // Part 2
    // Go backwards and remove one byte at the time trying to find a path.
    // When we find a path then the next byte to fall must have been the one to
    // finally block the exit.
    let mut blocking_location = None;
    for i in (0..fall_locations.len()).rev() {
        let mut next: VecDeque<(u32, Vector2<i32>)> = VecDeque::new();
        let mut visited: HashSet<Vector2<i32>> = HashSet::new();
        next.push_back((0, start));
        visited.insert(start);

        const OFFSETS: &[Vector2<i32>] = &[
            Vector2::new(1, 0),
            Vector2::new(0, 1),
            Vector2::new(-1, 0),
            Vector2::new(0, -1),
        ];

        // Part 1
        'outer: while let Some((cost, current)) = next.pop_front() {
            for offset in OFFSETS {
                let neighbour = current + offset;

                if visited.contains(&neighbour)
                    || fall_locations[..=i].contains(&neighbour)
                    || !(0..=size).contains(&neighbour.x)
                    || !(0..=size).contains(&neighbour.y)
                {
                    continue;
                }

                visited.insert(neighbour);
                next.push_back((cost + 1, neighbour));

                if neighbour == end {
                    break 'outer;
                }
            }
        }

        if next.pop_back().is_some() {
            // We found a path so the next byte must have been the blocking one.
            blocking_location = fall_locations.get(i + 1);
            break;
        }
    }

    let part2 = blocking_location.map(|pos| pos.iter().map(|v| v.to_string()).join(","));

    // Draw a map.
    // for y in 0..=size {
    //     for x in 0..=size {
    //         if visited.contains(&Vector2::new(x, y)) {
    //             print!("O");
    //         } else if fall_locations.contains(&Vector2::new(x, y)) {
    //             print!("#");
    //         } else {
    //             print!(".")
    //         }
    //     }
    //     println!();
    // }

    (common::from_option(part1), common::from_option(part2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::solution;

    #[test]
    fn p1_example_1() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";
        println!("input: {}", input);
        let (result, _) = solve_(str::as_bytes(input), 6, 12);
        assert_eq!(result, "22");
    }
    solution!(p1, p1_solution, "348", ignore = "too slow in debug release");

    // Part 2
    #[test]
    fn p2_example_1() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";
        println!("input: {}", input);
        let (_, result) = solve_(str::as_bytes(input), 6, 12);
        assert_eq!(result, "6,1");
    }
    solution!(
        p2,
        p2_solution,
        "54,44",
        ignore = "too slow in debug release"
    );
}
