use std::collections::HashSet;

pub(super) fn parse_map(input: &str) -> HashSet<(usize, usize)> {
    let mut galaxies = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    galaxies.insert((x + 1, y + 1));
                }
                '.' => {}
                _ => panic!("found unknown character '{c}' while parsing input"),
            }
        }
    }

    galaxies
}
