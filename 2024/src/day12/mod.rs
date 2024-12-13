pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 12: Garden Groups",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    let mut fences: HashMap<(i32, i32), (bool, bool)> = HashMap::new();
    let mut regions: HashMap<(i32, i32), u32> = HashMap::new();
    let mut region_area: HashMap<u32, u32> = HashMap::new();
    let mut region_perimeter: HashMap<u32, u32> = HashMap::new();
    let mut region_corners: HashMap<u32, u32> = HashMap::new();

    for (y, line) in (1..).zip(input.lines()) {
        for (x, c) in (1..).zip(line.chars()) {
            map.insert((x, y), c);
        }
    }

    let max_x = *map.keys().map(|(x, _)| x).max().unwrap_or(&1);
    let max_y = *map.keys().map(|(_, y)| y).max().unwrap_or(&1);

    // Calculate all the regions.
    let mut region_index = 0;
    for y in 1..=max_y {
        for x in 1..=max_x {
            if regions.contains_key(&(x, y)) {
                continue;
            }

            regions.extend(
                find_region(&map, (x, y))
                    .into_iter()
                    .map(|loc| (loc, region_index)),
            );

            region_index += 1;
        }
    }

    // Calculate all the fences.
    for y in 1..=max_y + 1 {
        for x in 1..=max_x + 1 {
            let region_current = regions.get(&(x, y));
            let region_north = regions.get(&(x, y - 1));
            let region_west = regions.get(&(x - 1, y));

            let horizontal_border = match (region_current, region_north) {
                (Some(a), Some(b)) if a == b => false,
                (None, None) => false,
                _ => true,
            };
            let vertical_border = match (region_current, region_west) {
                (Some(a), Some(b)) if a == b => false,
                (None, None) => false,
                _ => true,
            };
            fences.insert((x, y), (vertical_border, horizontal_border));

            if let Some(region) = region_current {
                region_area
                    .entry(*region)
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
                region_perimeter
                    .entry(*region)
                    .and_modify(|v| *v += horizontal_border as u32 + vertical_border as u32)
                    .or_insert(horizontal_border as u32 + vertical_border as u32);
            }
            if let Some(region) = region_north {
                region_perimeter
                    .entry(*region)
                    .and_modify(|v| *v += horizontal_border as u32)
                    .or_insert(horizontal_border as u32);
            }
            if let Some(region) = region_west {
                region_perimeter
                    .entry(*region)
                    .and_modify(|v| *v += vertical_border as u32)
                    .or_insert(vertical_border as u32);
            }
        }
    }

    // Calculate all the corners.
    for y in 1..=max_y + 1 {
        for x in 1..=max_x + 1 {
            let region_nw = regions.get(&(x - 1, y - 1));
            let region_ne = regions.get(&(x, y - 1));
            let region_sw = regions.get(&(x - 1, y));
            let region_se = regions.get(&(x, y));

            if let Some(current_region) = region_nw {
                match (
                    region_ne.is_some_and(|v| v == current_region),
                    region_sw.is_some_and(|v| v == current_region),
                    region_se.is_some_and(|v| v == current_region),
                ) {
                    (true, true, true) => {}
                    (true, true, false) => {
                        region_corners
                            .entry(*current_region)
                            .and_modify(|v| *v += 1)
                            .or_insert(1);
                    }
                    (true, false, _) => {}
                    (false, true, _) => {}
                    (false, false, _) => {
                        region_corners
                            .entry(*current_region)
                            .and_modify(|v| *v += 1)
                            .or_insert(1);
                    }
                };
            }

            if let Some(current_region) = region_ne {
                match (
                    region_nw.is_some_and(|v| v == current_region),
                    region_se.is_some_and(|v| v == current_region),
                    region_sw.is_some_and(|v| v == current_region),
                ) {
                    (true, true, true) => {}
                    (true, true, false) => {
                        region_corners
                            .entry(*current_region)
                            .and_modify(|v| *v += 1)
                            .or_insert(1);
                    }
                    (true, false, _) => {}
                    (false, true, _) => {}
                    (false, false, _) => {
                        region_corners
                            .entry(*current_region)
                            .and_modify(|v| *v += 1)
                            .or_insert(1);
                    }
                };
            }

            if let Some(current_region) = region_se {
                match (
                    region_ne.is_some_and(|v| v == current_region),
                    region_sw.is_some_and(|v| v == current_region),
                    region_nw.is_some_and(|v| v == current_region),
                ) {
                    (true, true, true) => {}
                    (true, true, false) => {
                        region_corners
                            .entry(*current_region)
                            .and_modify(|v| *v += 1)
                            .or_insert(1);
                    }
                    (true, false, _) => {}
                    (false, true, _) => {}
                    (false, false, _) => {
                        region_corners
                            .entry(*current_region)
                            .and_modify(|v| *v += 1)
                            .or_insert(1);
                    }
                };
            }

            if let Some(current_region) = region_sw {
                match (
                    region_nw.is_some_and(|v| v == current_region),
                    region_se.is_some_and(|v| v == current_region),
                    region_ne.is_some_and(|v| v == current_region),
                ) {
                    (true, true, true) => {}
                    (true, true, false) => {
                        region_corners
                            .entry(*current_region)
                            .and_modify(|v| *v += 1)
                            .or_insert(1);
                    }
                    (true, false, _) => {}
                    (false, true, _) => {}
                    (false, false, _) => {
                        region_corners
                            .entry(*current_region)
                            .and_modify(|v| *v += 1)
                            .or_insert(1);
                    }
                };
            }
        }
    }

    // // Regions printed with fences.
    // println!();
    // for y in 0..=2 * max_y {
    //     for x in 0..=2 * max_x {
    //         if x % 2 != 0 && y % 2 != 0 {
    //             if let Some(c) = map.get(&((x + 1) / 2, (y + 1) / 2)) {
    //                 print!("{c}");
    //             } else {
    //                 print!(".");
    //             }
    //         } else if x % 2 != 0 {
    //             // print!("{}", (x + 2) / 2);
    //             match fences.get(&((x + 2) / 2, (y + 2) / 2)) {
    //                 Some((_, true)) => print!("─"),
    //                 _ => print!(" "),
    //             }
    //         } else if y % 2 != 0 {
    //             match fences.get(&((x + 2) / 2, (y + 2) / 2)) {
    //                 Some((true, _)) => print!("│"),
    //                 _ => print!(" "),
    //             }
    //         } else {
    //             print!("+");
    //         }
    //     }
    //     println!();
    // }

    // // Debug region indices.
    // println!();
    // for y in 1..=max_y {
    //     for x in 1..=max_x {
    //         if let Some(index) = regions.get(&(x, y)) {
    //             print!("{:3x}", index);
    //         } else {
    //             print!("  .");
    //         }
    //     }
    //     println!();
    // }

    let part1: u32 = regions
        .values()
        .unique()
        .sorted_by(|a, b| a.cmp(b))
        .map(|region_index| {
            region_area.get(region_index).unwrap() * region_perimeter.get(region_index).unwrap()
        })
        .sum();

    let part2: u32 = regions
        .values()
        .unique()
        .sorted_by(|a, b| a.cmp(b))
        .map(|region_index| {
            region_area.get(region_index).unwrap() * region_corners.get(region_index).unwrap()
        })
        .sum();

    (part1.to_string(), part2.to_string())
}

/// Returns all the locations that are in the same region as start.
/// Solution uses BFS to locate all connecting areas. If start is outside
/// the map an empty region is returned with no locations inside it.
fn find_region(map: &HashMap<(i32, i32), char>, start: (i32, i32)) -> HashSet<(i32, i32)> {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut visit_queue: VecDeque<(i32, i32)> = VecDeque::new();
    let offsets = [(-1, 0), (0, -1), (1, 0), (0, 1)].into_iter();

    let Some(target_region) = map.get(&start) else {
        return visited;
    };

    visited.insert(start);
    visit_queue.push_back(start);

    while let Some(current) = visit_queue.pop_front() {
        for offset in offsets.clone() {
            let neighbour = (current.0 + offset.0, current.1 + offset.1);
            let is_same_region = map.get(&neighbour).is_some_and(|v| v == target_region);

            if is_same_region && !visited.contains(&neighbour) {
                visited.insert(neighbour);
                visit_queue.push_back(neighbour);
            }
        }
    }

    visited
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "AAAA
BBCD
BBCC
EEEC",
        "140"
    );
    example!(
        p1,
        p1_example_2,
        "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
        "772"
    );
    example!(
        p1,
        p1_example_3,
        "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
        "1930"
    );
    example!(
        p1,
        p1_example_4,
        "AABB
ABBC",
        "68"
    );
    example!(
        p1,
        p1_example_5,
        "AAAAAAAA
AACBBDDA
AACBBAAA
ABBAAAAA
ABBADDDA
AAAADADA
AAAAAAAA",
        "2566"
    );
    solution!(p1, p1_solution, "1431316");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "AAAA
BBCD
BBCC
EEEC",
        "80"
    );
    example!(
        p2,
        p2_example_2,
        "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
        "436"
    );
    example!(
        p2,
        p2_example_3,
        "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
        "236"
    );
    example!(
        p2,
        p2_example_4,
        "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
        "368"
    );
    example!(
        p2,
        p2_example_5,
        "AAAAAAAA
AACBBDDA
AACBBAAA
ABBAAAAA
ABBADDDA
AAAADADA
AAAAAAAA",
        "946"
    );
    example!(
        p2,
        p2_example_6,
        "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
        "1206"
    );
    solution!(p2, p2_solution, "821428");
}
