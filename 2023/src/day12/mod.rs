pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 12: Hot Springs",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod nonogram;
mod parse;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let (_, mut rows) = parse::parse_rows(&input).expect("input should be valid");

    // Part 1
    let part1: usize = rows
        .iter()
        .inspect(|row| print!("{row} => "))
        .map(|row| row.count_combinations())
        .inspect(|count| println!("{count}"))
        .sum();

    println!("\x1b[33mDid recursion {} times.\x1b[0m", unsafe {
        nonogram::COUNT
    });
    unsafe {
        nonogram::COUNT = 0;
    }

    // Part 2
    rows.iter_mut().for_each(|r| r.unfold(5));
    let mut part2: u32 = 0;
    // part2 = rows.iter().map(|row| row.count_combinations()).sum();

    (part1.to_string(), part2.to_string())
    // (0.to_string(), 0.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        "21"
    );
    example!(p1, p1_example_2, "???.### 1,1,3", "1");
    example!(p1, p1_example_3, ".??..??...?##. 1,1,3", "4");
    example!(p1, p1_example_4, "?#?#?#?#?#?#?#? 1,3,1,6", "1");
    example!(p1, p1_example_5, "????.#...#... 4,1,1", "1");
    example!(p1, p1_example_6, "????.######..#####. 1,6,5", "4");
    example!(p1, p1_example_7, "?###???????? 3,2,1", "10");
    example!(p1, p1_example_8, "??? 1,1", "1");
    example!(p1, p1_example_9, "?????#.?????##?##?? 1,1,9", "15");
    example!(p1, p1_example_10, "???????.??####?????? 4,1,1,8,1", "3");
    solution!(p1, p1_solution, "8075");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        "525152"
    );
    example!(p2, p2_example_2, "???.### 1,1,3", "1");
    example!(p2, p2_example_3, ".??..??...?##. 1,1,3", "16384");
    example!(p2, p2_example_4, "?#?#?#?#?#?#?#? 1,3,1,6", "1");
    example!(p2, p2_example_5, "????.#...#... 4,1,1", "16");
    example!(p2, p2_example_6, "????.######..#####. 1,6,5", "2500");
    example!(p2, p2_example_7, "?###???????? 3,2,1", "506250");
    // solution!(p2, p2_solution, "100");
}
