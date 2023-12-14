use std::collections::HashMap;

use super::{Platform, Rock};

pub(super) fn parse_input(input: &str) -> Platform {
    let mut max_x = u32::MIN;
    let mut max_y = u32::MIN;
    let mut rocks = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        max_y = max_y.max(y as u32 + 2);
        for (x, c) in line.chars().enumerate() {
            max_x = max_x.max(x as u32 + 2);
            match c {
                '#' => {
                    rocks.insert((x as u32 + 1, y as u32 + 1), Rock::Cubed);
                }
                'O' => {
                    rocks.insert((x as u32 + 1, y as u32 + 1), Rock::Round);
                }
                '.' => {}
                _ => panic!(
                    "got unknown character {c} at {:?}",
                    (x as u32 + 1, y as u32 + 1)
                ),
            }
        }
    }

    Platform {
        x_bound: 1..max_x,
        y_bound: 1..max_y,
        rocks,
    }
}
