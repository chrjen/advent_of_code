use std::collections::HashMap;

use super::Coord;

pub(super) fn parse_contraption(input: &str) -> HashMap<Coord, u32> {
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;
    let mut heat_loss = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        max_y = max_y.max(y + 2);
        for (x, c) in line.chars().enumerate() {
            max_x = max_x.max(x + 2);
            heat_loss.insert(
                (x + 1, y + 1),
                c.to_digit(10)
                    .unwrap_or_else(|| panic!("got non-digit {c} in input")),
            );
        }
    }

    heat_loss
}
