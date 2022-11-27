use std::collections::HashMap;

use regex::Regex;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 16: Aunt Sue",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

#[derive(Debug)]
struct Sue {
    id: usize,
    compounds: HashMap<String, i32>,
}

impl Sue {
    fn new(id: usize) -> Self {
        Sue {
            id,
            compounds: HashMap::new(),
        }
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let sue_reg = Regex::new(r"Sue (\d+):(.*)").unwrap();
    let compound_reg = Regex::new(r"\s*(\w+):\s*(\d+),?").unwrap();

    let mut sues: Vec<Sue> = Vec::new();

    for cap in sue_reg.captures_iter(&input) {
        let mut sue = Sue::new(cap[1].parse().unwrap());

        for cap in compound_reg.captures_iter(&cap[2]) {
            sue.compounds
                .insert(cap[1].to_owned(), cap[2].parse::<i32>().unwrap());
        }

        sues.push(sue);
    }

    let targets = vec![
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ];

    // Part 1.
    // Goes through each Sue and checks if the compound match the target compounds
    // if they are present. If a Sue whose compounds all match the target, then
    // that Sue is the aunt Sue we are looking for and is returned.
    let aunt_sue = sues.iter().find(|s| {
        for target in targets.iter() {
            let (name, target_value) = *target;
            if let Some(value) = s.compounds.get(name) {
                let condition = *value == target_value;

                if !condition {
                    return false;
                }
            }
        }
        true
    });
    let part1 = aunt_sue.map(|aunt_sue| aunt_sue.id.to_string());

    // Part 2.
    // Same as part 1 except the condition are a little different.
    let real_sue = sues.iter().find(|s| {
        for target in targets.iter() {
            let (name, target_value) = *target;
            if let Some(value) = s.compounds.get(name) {
                let condition = match name {
                    "cats" | "trees" => *value > target_value,
                    "pomeranians" | "goldfish" => *value < target_value,
                    _ => *value == target_value,
                };

                if !condition {
                    return false;
                }
            }
        }
        true
    });
    let part2 = real_sue.map(|real_sue| real_sue.id.to_string());

    (common::from_option(part1), common::from_option(part2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::solution;

    // Part 1
    solution!(p1, p1_solution, "103");

    // Part 2
    solution!(p2, p2_solution, "405");
}
