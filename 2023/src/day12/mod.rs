use std::collections::HashMap;

pub const SOLUTION: common::Solver = common::Solver {
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
    let mut cache = HashMap::new();
    let part1: usize = rows
        .iter()
        .map(|row| row.count_combinations(&mut cache))
        .sum();

    // println!("\x1b[33m(Part 1) Did recursion {} times.\x1b[0m", unsafe {
    //     nonogram::COUNT
    // });
    // unsafe {
    //     nonogram::COUNT = 0;
    // }

    // Part 2
    rows.iter_mut().for_each(nonogram::Row::unfold);
    let part2: usize = rows
        .iter()
        .map(|row| row.count_combinations(&mut cache))
        // .enumerate()
        // .inspect(|(i, j)| println!("{i}/1000 : {j}"))
        // .map(|(_, v)| v)
        .sum();

    // println!("\x1b[33m(Part 2) Did recursion {} times.\x1b[0m", unsafe {
    //     nonogram::COUNT
    // });
    // unsafe {
    //     nonogram::COUNT = 0;
    // }

    (part1.to_string(), part2.to_string())
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
    example!(p1, p1_example_11, "??????????? 1,1,3,1", "15");
    solution!(
        p1,
        p1_solution,
        "8075",
        ignore = "too slow in debug release"
    );

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
    solution!(
        p2,
        p2_solution,
        "4232520187524",
        ignore = "too slow in debug release"
    );
}
