pub const INPUT: &[u8] = std::include_bytes!("day1_input");

pub fn solve(input: &[u8]) {
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

    println!("solution = {} and {:?}", floor, basement_idx);
}
