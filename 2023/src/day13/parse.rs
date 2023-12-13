use std::collections::HashSet;

pub(super) fn parse_input(input: &str) -> Vec<HashSet<(u32, u32)>> {
    let mut vec = Vec::new();

    for map in input.split("\n\n") {
        let mut set = HashSet::new();
        for (y, line) in map.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        set.insert((x as u32 + 1, y as u32 + 1));
                    }
                    '.' => {}
                    _ => panic!(
                        "got unknown character {c} at {:?}",
                        (x as u32 + 1, y as u32 + 1)
                    ),
                }
            }
        }
        vec.push(set);
    }

    vec
}
