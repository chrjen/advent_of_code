pub struct Solution();

pub const INPUT: &[u8] = std::include_bytes!("input");

impl common::Solver for Solution {
    type Part1 = isize;
    type Part2 = String;

    fn solve(input: &[u8]) -> (isize, String) {
        let mut floor: isize = 0;
        let mut basement_idx: Option<usize> = None;

        for (i, c) in input.into_iter().enumerate() {
            match *c as char {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => {}
            }

            if basement_idx.is_none() && floor < 0 {
                basement_idx = Some(i + 1);
            }
        }

        (floor, common::from_option(basement_idx))
    }
}
