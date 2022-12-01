pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 1: Calorie Counting",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let mut top3: [i64; 3] = [i64::MIN, i64::MIN, i64::MIN];
    let mut count = 0;
    for line in input.lines() {
        if line.is_empty() {
            if count > top3[0] {
                if count > top3[1] {
                    top3[0] = top3[1];
                    if count > top3[2] {
                        top3[1] = top3[2];
                        top3[2] = count;
                    } else {
                        top3[1] = count;
                    }
                } else {
                    top3[0] = count;
                }
            }
            count = 0;
            continue;
        }
        count += line.parse::<i64>().unwrap();
    }
    if count > top3[0] {
        if count > top3[1] {
            top3[0] = top3[1];
            if count > top3[2] {
                top3[1] = top3[2];
                top3[2] = count;
            } else {
                top3[1] = count;
            }
        } else {
            top3[0] = count;
        }
    }

    (top3[2].to_string(), top3.iter().sum::<i64>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        "24000"
    );
    solution!(p1, p1_solution, "71506");

    // Part 2
    example!(
        p2,
        p2_example_1,
        r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        "45000"
    );
    solution!(p2, p2_solution, "209603");
}
