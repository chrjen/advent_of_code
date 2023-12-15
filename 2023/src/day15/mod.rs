pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 15: Lens Library",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

#[derive(Debug, Clone, Copy)]
struct Hasher {
    value: u32,
}

impl Hasher {
    fn new() -> Self {
        Hasher { value: 0 }
    }

    fn hash(&mut self, c: char) {
        self.value += c as u32;
        self.value *= 17;
        self.value %= 256;
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    // Part 1
    let part1: u32 = input
        .split(',')
        .map(|seq| {
            let mut hasher = Hasher::new();
            for c in seq.chars() {
                match c {
                    '\n' => {}
                    c => hasher.hash(c),
                }
            }
            hasher.value
        })
        .sum();

    (part1.to_string(), 0.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
        "1320"
    );
    solution!(p1, p1_solution, "514394");

    // Part 2
    // example!(p2, p2_example_1, "", "0");
    // example!(p2, p2_example_2, "", "0");
    // example!(p2, p2_example_3, "", "0");
    // example!(p2, p2_example_4, "", "0");
    // example!(p2, p2_example_5, "", "0");
    // solution!(p2, p2_solution, "100");
}
