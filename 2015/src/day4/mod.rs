pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 4: The Ideal Stocking Stuffer",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input).into_owned();
    let mut hash_input = input.clone();
    let is_zero_bits = |byte: u8, mask: u8| (byte & mask) == 0;

    let mut i: u32 = 0;

    // Part 1.
    let five_zeroes: u32 = loop {
        hash_input.push_str(&i.to_string());
        let digest = md5::compute(hash_input.as_bytes());

        let cond = is_zero_bits(digest[0], 0xff)
            && is_zero_bits(digest[1], 0xff)
            && is_zero_bits(digest[2], 0xf0);
        if cond {
            break i;
        }

        i += 1;
        hash_input.clone_from(&input);
    };

    // Part 2.
    let six_zeroes: u32 = loop {
        hash_input.push_str(&i.to_string());
        let digest = md5::compute(hash_input.as_bytes());

        let cond = is_zero_bits(digest[0], 0xff)
            && is_zero_bits(digest[1], 0xff)
            && is_zero_bits(digest[2], 0xff);
        if cond {
            break i;
        }

        i += 1;
        hash_input.clone_from(&input);
    };

    (five_zeroes.to_string(), six_zeroes.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(p1, p1_example_1, "abcdef", "609043", ignore);
    example!(p1, p1_example_2, "pqrstuv", "1048970", ignore);
    solution!(p1, p1_solution, "117946", ignore);

    // Part 2
    solution!(p2, p2_solution, "3938038", ignore);
}
