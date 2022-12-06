use std::collections::HashSet;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 6: Tuning Trouble",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let input: Vec<_> = String::from_utf8_lossy(input).chars().collect();

    /// Sliding window with length `package_len` over the input.
    /// Processing `package_len` characters at the time.
    ///
    /// A `HashSet` is used to remove any duplicates in each window.
    ///
    /// We clear previous set of characters each iteration. Could probably be made more
    /// efficient by reusing `set` rather than clearing it for each window. E.g. using
    /// a `HashMap<char, u32>` instead. Adding and removing `char`s as the window slides along.
    ///
    /// If the set and window has the same length then all characters must have been unique and
    /// we return the index of the window plus its length.
    fn find_packet(input: &[char], package_len: usize) -> Option<usize> {
        let mut result = None;
        let mut set = HashSet::new();

        for (i, window) in input.windows(package_len).enumerate() {
            set.clear();

            for c in window {
                if !set.insert(c) {
                    break;
                }
            }

            if set.len() == window.len() {
                result = Some(i + package_len);
                break;
            }
        }

        result
    }

    (
        common::from_option(find_packet(&input, 4)),
        common::from_option(find_packet(&input, 14)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(p1, p1_example_1, "bvwbjplbgvbhsrlpgdmjqwftvncz", "5");
    example!(p1, p1_example_2, "nppdvjthqldpwncqszvftbrmjlhg", "6");
    example!(p1, p1_example_3, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "10");
    example!(p1, p1_example_4, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "11");
    solution!(p1, p1_solution, "1300");

    // Part 2
    example!(p2, p2_example_1, "mjqjpqmgbljsphdztnvjfqwrcgsmlb", "19");
    example!(p2, p2_example_2, "bvwbjplbgvbhsrlpgdmjqwftvncz", "23");
    example!(p2, p2_example_3, "nppdvjthqldpwncqszvftbrmjlhg", "23");
    example!(p2, p2_example_4, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "29");
    example!(p2, p2_example_5, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "26");
    solution!(p2, p2_solution, "3986");
}
