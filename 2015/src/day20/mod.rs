pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 20: Infinite Elves and Infinite Houses",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let target: u32 = String::from_utf8_lossy(input).parse().unwrap();

    fn calc_num_present_infinite(house: u32) -> u32 {
        [1, house]
            .into_iter()
            .chain(divisors::get_divisors(house))
            .sum::<u32>()
    }

    fn calc_num_present_50(house: u32) -> u32 {
        [1, house]
            .into_iter()
            .chain(divisors::get_divisors(house))
            .filter(|&x| house <= x * 50)
            .sum::<u32>()
    }

    let part1 = (1..)
        .find(|&house| 10 * calc_num_present_infinite(house) >= target)
        .unwrap();

    let part2 = (1..)
        .find(|&house| 11 * calc_num_present_50(house) >= target)
        .unwrap();

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(p1, p1_example_1, "10", "1");
    example!(p1, p1_example_2, "100", "6");
    example!(p1, p1_example_3, "130", "8");
    solution!(p1, p1_solution, "831600", ignore = "takes too long");

    // Part 2
    solution!(p2, p2_solution, "884520", ignore = "takes too long");
}
