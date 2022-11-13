use regex::Regex;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 2: I Was Told There Would Be No Math",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let input: &str = std::str::from_utf8(input).unwrap();
    let reg = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
    let mut total_area = 0;
    let mut total_length = 0;

    for line in input.lines() {
        for grp in reg.captures_iter(line) {
            let l: u32 = grp[1].parse().expect("integer");
            let w: u32 = grp[2].parse().expect("integer");
            let h: u32 = grp[3].parse().expect("integer");

            let lw = l * w;
            let wh = w * h;
            let hl = h * l;

            let min_side = lw.min(wh).min(hl);
            let area = 2 * (lw + wh + hl) + min_side;
            total_area += area;

            let length = 2 * (l + w + h) + l * w * h;
            let length = length - 2 * l.max(w).max(h);
            total_length += length;
        }
    }

    (total_area.to_string(), total_length.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(p1, p1_example_1, "2x3x4", "58");
    example!(p1, p1_example_2, "1x1x10", "43");
    example!(p1, p1_example_3, "jchzalrnumimnmhp", "0");
    example!(p1, p1_example_4, "haegwjzuvuyypxyu", "0");
    example!(p1, p1_example_5, "dvszwmarrgswjxmb", "0");
    solution!(p1, p1_solution, "1588178");

    // Part 2
    example!(p2, p2_example_1, "2x3x4", "34");
    example!(p2, p2_example_2, "1x1x10", "14");
    solution!(p2, p2_solution, "3783758");
}
