pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 17: No Such Thing as Too Much",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

/// Recursive algorithm solving part 1 and part 2.
///
/// Example, given the containers `[5, 5, 10, 15, 20]` and target 25: \
/// Solve `[5, 5, 10, 15]` and target 5 \
/// Solve `[5, 5, 10, 15]` and target 25 \
/// Then add solutions together for answer.
///
/// When the target reach zero that means we have reached the original target
/// exactly and we return 1, else return 0 if we reach the end of containers
/// or target goes below zero.
fn count_combinations(containers: &[i32], target: i32) -> (u32, u32) {
    if containers.is_empty() {
        return (0, 0);
    }

    let mut containers = containers.to_owned();
    containers.sort_unstable();
    let mut n_count = vec![0; containers.len() + 1];

    fn count(
        n_count: &mut Vec<u32>,
        containers: &[i32],
        idx: usize,
        target: i32,
        used: usize,
    ) -> u32 {
        if idx == 0 && target == containers[idx] {
            n_count[used + 1] += 1;
            return 1;
        }

        if target == 0 {
            n_count[used] += 1;
            return 1;
        }

        if target.is_negative() || idx == 0 {
            return 0;
        }

        let mut total = 0;

        // Use current container.
        total += count(
            n_count,
            containers,
            idx - 1,
            target - containers[idx],
            used + 1,
        );
        // Don't use current container.
        total += count(n_count, containers, idx - 1, target, used);

        total
    }

    let c = count(
        &mut n_count,
        containers.as_slice(),
        containers.len() - 1,
        target,
        0,
    );
    let f = *n_count.iter().find(|x| **x != 0).unwrap();
    (c, f)
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let containers: Vec<i32> = input.lines().map(|x| x.parse().unwrap()).collect();

    let (part1, part2) = count_combinations(&containers, 150);

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::solution;

    // Part 1
    #[test]
    fn p1_example_1() {
        let v = vec![5, 5, 10, 15, 20];
        assert_eq!(count_combinations(&v, 25).0, 4);
    }
    solution!(p1, p1_solution, "654");

    // Part 2
    #[test]
    fn p2_example_1() {
        let v = vec![5, 5, 10, 15, 20];
        assert_eq!(count_combinations(&v, 25).1, 3);
    }
    solution!(p2, p2_solution, "57");
}
