pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 11: Corporate Policy",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

fn is_valid_password(password: &Vec<char>) -> bool {
    if password.len() != 8 {
        return false;
    }

    let mut count_inc = 1;
    let mut count_pair = 0;

    let mut chars = password.iter();
    let mut prev = *chars.next().unwrap();
    let mut prev_pair = '\0';

    if matches!(prev, 'i' | 'o' | 'l') {
        return false;
    }

    for ch in chars {
        let ch = *ch;

        if matches!(ch, 'i' | 'o' | 'l') {
            return false;
        }

        if count_inc < 3 {
            if ch == (prev as u8 + 1) as char {
                count_inc += 1;
            } else {
                count_inc = 1;
            }
        }

        if ch == prev && ch != prev_pair {
            count_pair += 1;
            prev_pair = ch;
        }

        prev = ch;
    }

    count_inc >= 3 && count_pair >= 2
}

pub fn solve(input: &[u8]) -> (String, String) {
    let mut input: Vec<char> = String::from_utf8_lossy(input)
        .chars()
        .filter(|x| x.is_ascii_lowercase())
        .collect();

    fn inc_password(password: &mut Vec<char>) {
        if password.is_empty() {
            return;
        }

        fn inc_password_(idx: usize, password: &mut Vec<char>) {
            let ch = password[idx];
            if ch == 'z' {
                password[idx] = 'a';
                if idx != 0 {
                    inc_password_(idx - 1, password)
                }
            } else {
                password[idx] = (ch as u8 + 1) as char
            }
        }

        inc_password_(password.len() - 1, password)
    }

    fn chars_to_string(chars: &Vec<char>) -> String {
        let mut s = String::new();
        for ch in chars {
            s.push(*ch);
        }
        s
    }

    // Part1.
    while !is_valid_password(&input) {
        inc_password(&mut input);
    }
    let part1 = chars_to_string(&input);

    // Part2.
    inc_password(&mut input);
    while !is_valid_password(&input) {
        inc_password(&mut input);
    }
    let part2 = chars_to_string(&input);

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    #[test]
    fn p1_example_1() {
        assert!(!is_valid_password(&"hijklmmn".chars().collect()));
    }
    #[test]
    fn p1_example_2() {
        assert!(!is_valid_password(&"abbceffg".chars().collect()));
    }
    #[test]
    fn p1_example_3() {
        assert!(!is_valid_password(&"abbcegjk".chars().collect()));
    }
    #[test]
    fn p1_example_4() {
        assert!(is_valid_password(&"abcdffaa".chars().collect()));
    }
    #[test]
    fn p1_example_5() {
        assert!(is_valid_password(&"ghjaabcc".chars().collect()));
    }
    example!(p1, p1_example_6, "abcdefgh", "abcdffaa");
    example!(p1, p1_example_7, "ghijklmn", "ghjaabcc");
    solution!(p1, p1_solution, "cqjxxyzz");

    // Part 2
    solution!(p2, p2_solution, "cqkaabcc");
}
