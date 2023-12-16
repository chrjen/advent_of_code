pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 10: Elves Look, Elves Say",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

/// Creates the look-and-say string appending it to the `dst` string.
/// Returns an immutable reference to the `dst` string.
fn look_and_say<'a>(dst: &'a mut String, src: &str) -> &'a str {
    let mut chars = src.chars();
    let Some(mut prev) = chars.next() else {
        return dst;
    };
    let mut count: u32 = 1;

    for ch in chars {
        if prev == ch {
            count += 1;
            continue;
        }

        dst.push_str(&format!("{}{}", count, prev));

        prev = ch;
        count = 1;
    }

    dst.push_str(&format!("{}{}", count, prev));

    dst
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let src = &mut input.into_owned();
    let dst = &mut String::new();

    for _ in 0..40 {
        dst.clear();
        look_and_say(dst, src);
        std::mem::swap(dst, src);
    }
    let part1 = src.len();

    for _ in 0..10 {
        dst.clear();
        look_and_say(dst, src);
        std::mem::swap(dst, src);
    }
    let part2 = src.len();

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::solution;

    // Part 1
    #[test]
    fn p1_example_1() {
        assert_eq!(look_and_say(&mut String::new(), "1"), "11");
    }
    #[test]
    fn p1_example_2() {
        assert_eq!(look_and_say(&mut String::new(), "11"), "21");
    }
    #[test]
    fn p1_example_3() {
        assert_eq!(look_and_say(&mut String::new(), "21"), "1211");
    }
    #[test]
    fn p1_example_4() {
        assert_eq!(look_and_say(&mut String::new(), "1211"), "111221");
    }
    #[test]
    fn p1_example_5() {
        assert_eq!(look_and_say(&mut String::new(), "111221"), "312211");
    }
    solution!(p1, p1_solution, "492982", ignore = "takes too long to run");

    // Part 2
    solution!(p2, p2_solution, "6989950", ignore = "takes too long to run");
}
