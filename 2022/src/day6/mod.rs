use std::collections::HashMap;

pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 6: Tuning Trouble",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let input: Vec<_> = String::from_utf8_lossy(input).chars().collect();

    /// Iterates over the input, one character at the time. A `HashMap` is used to keep track
    /// how many unique characters there are in the last `packet_len`
    ///
    /// Imagine a sliding window over the input of length `packet_len`. As characters enters the
    /// window the counter for that character gets incremented/inserted. Then decremented/removed
    /// when leaving the window.
    ///
    /// If the `map` and `packet_len` has the same length then all characters must have been unique and
    /// we return the index + 1, for the index of the character after the packet.
    fn find_packet(input: &[char], packet_len: usize) -> Option<usize> {
        let mut map = HashMap::new();

        for (i, c) in input.iter().enumerate() {
            *map.entry(c).or_insert(0) += 1;

            if i >= packet_len {
                let old_c = &input[i - packet_len];

                let count = map.get_mut(old_c).unwrap();
                *count -= 1;

                if *count == 0 {
                    map.remove(old_c);
                }
            }

            if map.len() == packet_len {
                return Some(i + 1);
            }
        }

        None
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
