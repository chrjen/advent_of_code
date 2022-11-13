use std::collections::HashSet;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 5: Doesn't He Have Intern-Elves For This?",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let mut nice_count = 0;
    let mut nicer_count = 0;
    let mut map: HashSet<(char, char)> = HashSet::new();

    for line in String::from_utf8_lossy(input).lines() {
        let mut chars = line.chars();
        let mut prev2 = chars.next().unwrap();
        let mut prev = chars.next().unwrap();
        let is_vowel = |c: char| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u');
        map.clear();

        // Line has at least three vowels. (aeiou)
        let mut vowel = 0;
        // Line has at least a single double letter. (aa, kk, …)
        let mut double = false;
        // Line does not have a banned two-letter sequence. (ab, cd, pq, or xy)
        let mut not = true;
        // Line has at least one pair of non-overlaping two-letter sequences.
        let mut pair = false;
        // Line has any letter sandwiched between two equal letters. (xyx, efe, aaa, …)
        let mut sand = false;

        // Pre-loop. Handling the first to characters seperately.
        {
            if is_vowel(prev2) {
                vowel += 1;
            }

            if is_vowel(prev) {
                vowel += 1;
            }

            if prev2 == prev {
                double = true;
            }
            if matches!(
                (prev2, prev),
                ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')
            ) {
                not = false;
            }
        }

        // Loop.
        let mut skip = false;
        for c in chars {
            if is_vowel(c) {
                vowel += 1;
            }
            if prev == c {
                double = true;
            }
            if matches!((prev, c), ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')) {
                not = false;
            }
            if prev2 == c {
                sand = true;
            }
            if !skip {
                if !map.insert((prev2, prev)) {
                    pair = true;
                }
                if prev2 == prev && prev == c {
                    skip = true;
                }
            } else {
                skip = false;
            }
            prev2 = prev;
            prev = c;
        }

        // Post-loop. Handling last characters.
        {
            if !skip {
                if !map.insert((prev2, prev)) {
                    pair = true;
                }
            }
        }

        // Part1.
        if vowel >= 3 && double && not {
            nice_count += 1;
        }

        // Part2.
        if pair && sand {
            nicer_count += 1;
        }
    }

    (nice_count.to_string(), nicer_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_example_1() {
        println!("input: ugknbfddgicrmopn");
        let (result, _) = solve(b"ugknbfddgicrmopn");
        assert_eq!(result, "1");
    }

    #[test]
    fn p1_example_2() {
        println!("input: aaa");
        let (result, _) = solve(b"aaa");
        assert_eq!(result, "1");
    }

    #[test]
    fn p1_example_3() {
        println!("input: jchzalrnumimnmhp");
        let (result, _) = solve(b"jchzalrnumimnmhp");
        assert_eq!(result, "0");
    }

    #[test]
    fn p1_example_4() {
        println!("input: haegwjzuvuyypxyu");
        let (result, _) = solve(b"haegwjzuvuyypxyu");
        assert_eq!(result, "0");
    }

    #[test]
    fn p1_example_5() {
        println!("input: dvszwmarrgswjxmb");
        let (result, _) = solve(b"dvszwmarrgswjxmb");
        assert_eq!(result, "0");
    }

    #[test]
    fn p1_solution() {
        let (result, _) = solve(SOLUTION.input);
        assert_eq!(result, "238");
    }

    #[test]
    fn p2_example_1() {
        println!("input: qjhvhtzxzqqjkmpb");
        let (_, result) = solve(b"qjhvhtzxzqqjkmpb");
        assert_eq!(result, "1");
    }

    #[test]
    fn p2_example_2() {
        println!("input: xxyxx");
        let (_, result) = solve(b"xxyxx");
        assert_eq!(result, "1");
    }

    #[test]
    fn p2_example_3() {
        println!("input: uurcxstgmygtbstg");
        let (_, result) = solve(b"uurcxstgmygtbstg");
        assert_eq!(result, "0");
    }

    #[test]
    fn p2_example_4() {
        println!("input: ieodomkazucvgmuy");
        let (_, result) = solve(b"ieodomkazucvgmuy");
        assert_eq!(result, "0");
    }

    #[test]
    fn p2_example_5() {
        println!("input: aaa");
        let (_, result) = solve(b"aaa");
        assert_eq!(result, "0");
    }

    #[test]
    fn p2_solution() {
        let (_, result) = solve(SOLUTION.input);
        assert_eq!(result, "69");
    }
}
