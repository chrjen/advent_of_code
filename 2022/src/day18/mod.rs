use std::collections::HashSet;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 18: Boiling Boulders",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let coords = input.lines().map(|line| {
        let mut numbers = line.split(',');
        let mut get_next_num = || numbers.next().unwrap().parse().unwrap();
        (get_next_num(), get_next_num(), get_next_num())
    });

    let mut count = 0;
    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();
    for coord in coords {
        let (x, y, z) = coord;
        count += 6;

        if cubes.contains(&(x + 1, y, z)) {
            count -= 2;
        }
        if cubes.contains(&(x - 1, y, z)) {
            count -= 2;
        }
        if cubes.contains(&(x, y + 1, z)) {
            count -= 2;
        }
        if cubes.contains(&(x, y - 1, z)) {
            count -= 2;
        }
        if cubes.contains(&(x, y, z + 1)) {
            count -= 2;
        }
        if cubes.contains(&(x, y, z - 1)) {
            count -= 2;
        }

        cubes.insert(coord);
    }

    // Part 1.
    let part1 = count;

    // Part 2.
    // Explorers the outside of the shape using a flood filling algorithm.

    // Add padding of one around the cubes to allow the filling to go around the
    // outside shape.
    let min_x = cubes.iter().min_by_key(|(x, _, _)| x).unwrap().0 - 1;
    let max_x = cubes.iter().max_by_key(|(x, _, _)| x).unwrap().0 + 1;
    let min_y = cubes.iter().min_by_key(|(_, y, _)| y).unwrap().1 - 1;
    let max_y = cubes.iter().max_by_key(|(_, y, _)| y).unwrap().1 + 1;
    let min_z = cubes.iter().min_by_key(|(_, _, z)| z).unwrap().2 - 1;
    let max_z = cubes.iter().max_by_key(|(_, _, z)| z).unwrap().2 + 1;

    let mut unvisited: Vec<(i32, i32, i32)> = Vec::new();
    let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();
    unvisited.push((min_x, min_y, min_z));

    // For each unvisited voxel we check its neighbours. If it contains a cube
    // then we increment `count`, else we add the neighbour to the list of
    // unvisited voxels, unless if has already been visited or on the unvisited
    // list. `count` is the number of faces discovered.
    let mut count = 0;
    while !unvisited.is_empty() {
        let (x, y, z) = unvisited.pop().unwrap();
        visited.insert((x, y, z));

        let neighbours = &[
            (x + 1, y, z),
            (x - 1, y, z),
            (x, y + 1, z),
            (x, y - 1, z),
            (x, y, z + 1),
            (x, y, z - 1),
        ];

        for neighbour in neighbours.iter() {
            let &(nx, ny, nz) = neighbour;
            if nx < min_x || nx > max_x {
                continue;
            }
            if ny < min_y || ny > max_y {
                continue;
            }
            if nz < min_z || nz > max_z {
                continue;
            }

            if cubes.contains(neighbour) {
                count += 1;
            } else if !visited.contains(neighbour) && !unvisited.contains(neighbour) {
                unvisited.push(*neighbour);
            }
        }
    }
    let part2 = count;

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
        "1,1,1
2,1,1",
        "10"
    );
    example!(
        p1,
        p1_example_2,
        "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5",
        "64"
    );
    solution!(p1, p1_solution, "4604");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5",
        "58"
    );
    solution!(p2, p2_solution, "2604");
}
