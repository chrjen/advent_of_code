pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 15: Lens Library",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod data;
mod parse;

use data::Hasher;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    // Part 1
    let part1: u32 = input.split(',').map(Hasher::hash_str).sum();

    // Part 2
    let steps = parse::parse_steps(&input).expect("input should be valid").1;
    let mut boxes: Box<[Vec<(&str, u8)>]> = vec![Vec::new(); 256].into_boxed_slice();

    // Do all the steps, modifying the boxes and any lenses inside.
    for step in steps.iter() {
        let box_idx = Hasher::hash_str(step.label) as usize;

        let label_idx = boxes[box_idx]
            .iter()
            .position(|(label, _)| *label == step.label);

        match (step.op, label_idx) {
            (data::Operation::Dash, Some(label_idx)) => {
                boxes[box_idx].remove(label_idx);
            }
            (data::Operation::Dash, None) => {}
            (data::Operation::Equal(focal_len), Some(label_idx)) => {
                boxes[box_idx][label_idx].1 = focal_len;
            }
            (data::Operation::Equal(focal_len), None) => {
                boxes[box_idx].push((step.label, focal_len));
            }
        }

        // println!("After \"{step}\":");
        // for (i, b) in boxes.iter().enumerate().filter(|(_, v)| !v.is_empty()) {
        //     print!("Box {i}:");
        //     for (label, focal_len) in b {
        //         print!(" [{label} {focal_len}]");
        //     }
        //     println!();
        // }
        // println!();
    }

    let mut part2: usize = 0;
    for (box_idx, slots) in boxes.iter().enumerate() {
        for (slot_idx, (_, focal_len)) in slots.iter().enumerate() {
            part2 += (box_idx + 1) * (slot_idx + 1) * (*focal_len as usize);
        }
    }

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
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
        "1320"
    );
    solution!(p1, p1_solution, "514394");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
        "145"
    );
    solution!(p2, p2_solution, "236358");
}
