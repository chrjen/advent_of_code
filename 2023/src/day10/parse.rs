use super::{Field, Pipe};

pub(super) fn parse_field(input: &str) -> Field {
    let mut field = Field::default();

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                '|' | '-' | 'L' | 'J' | '7' | 'F' => {
                    field
                        .layout
                        .insert((col as i32 + 1, row as i32 + 1), Pipe::try_from(c).unwrap());
                }
                'S' => field.start = (col as i32 + 1, row as i32 + 1),
                '.' => {}
                _ => {
                    panic!("got unexpected tile while parsing, got '{c}'");
                }
            };
        }
    }

    field
}
