use std::collections::HashSet;
use std::str;

#[derive(Clone, Copy, Default, Debug, Hash, PartialEq, Eq)]
struct House {
    x: i16,
    y: i16,
}

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 3: Perfectly Spherical Houses in a Vacuum",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = str::from_utf8(input).unwrap();

    let mut houses = HashSet::new();
    let mut current_house = House::default();
    houses.insert(current_house);

    for c in input.chars() {
        match c {
            '^' => current_house.y += 1,
            '>' => current_house.x += 1,
            '<' => current_house.x -= 1,
            'v' => current_house.y -= 1,
            _ => {}
        }
        houses.insert(current_house);
    }

    let part1 = houses.len();

    houses.clear();
    let mut current_santa_house = House::default();
    let mut current_robot_house = House::default();
    houses.insert(current_santa_house);

    for (i, c) in input.chars().enumerate() {
        let current_house = if i % 2 == 0 {
            &mut current_santa_house
        } else {
            &mut current_robot_house
        };

        match c {
            '^' => current_house.y += 1,
            '>' => current_house.x += 1,
            '<' => current_house.x -= 1,
            'v' => current_house.y -= 1,
            _ => {}
        }
        houses.insert(*current_house);
    }

    (part1.to_string(), houses.len().to_string())
}
