use std::collections::HashMap;

use super::data::{Contraption, Mirror};

pub(super) fn parse_contraption(input: &str) -> Contraption {
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;
    let mut mirrors = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        max_y = max_y.max(y + 2);
        for (x, c) in line.chars().enumerate() {
            max_x = max_x.max(x + 2);
            let mirror: Option<_> = match c {
                '\\' => Some(Mirror::Left),
                '/' => Some(Mirror::Right),
                '|' => Some(Mirror::SplitV),
                '-' => Some(Mirror::SplitH),
                '.' => None,
                _ => panic!("got unknown character {c} at {:?}", (x + 1, y + 1)),
            };
            if let Some(mirror) = mirror {
                mirrors.insert((x + 1, y + 1), mirror);
            }
        }
    }

    Contraption {
        x_bound: 1..max_x,
        y_bound: 1..max_y,
        mirrors,
    }
}
