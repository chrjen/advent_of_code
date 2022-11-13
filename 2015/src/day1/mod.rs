pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 1: Not Quite Lisp",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let mut floor: isize = 0;
    let mut basement_idx: Option<usize> = None;

    for (i, c) in input.iter().enumerate() {
        match *c as char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }

        if basement_idx.is_none() && floor < 0 {
            basement_idx = Some(i + 1);
        }
    }

    (floor.to_string(), common::from_option(basement_idx))
}
