pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 20: Pulse Propagation",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod data;
mod parse;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let initial_system = parse::parse_system(&input)
        .expect("input should be valid")
        .1;

    // Part 1
    let part1 = {
        let mut system = initial_system.clone();
        let mut total_high_count = 0;
        let mut total_low_count = 0;
        let mut button_count = 0;

        while button_count < 1000 {
            // println!("button_count {}", button_count);
            button_count += 1;

            let (low_count, high_count) = system.button_pulse();
            total_low_count += low_count;
            total_high_count += high_count;

            // We are back at the initial state so we know the state loops and
            // can skip ahead using the values we have so far.
            if system == initial_system {
                let loop_len = button_count;
                let remaining_loops = 1000 / loop_len;
                total_low_count *= remaining_loops;
                total_high_count *= remaining_loops;
                button_count *= remaining_loops;
            }
        }

        total_low_count * total_high_count
    };

    (part1.to_string(), 0.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
        "32000000"
    );
    example!(
        p1,
        p1_example_2,
        "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
        "11687500"
    );
    solution!(p1, p1_solution, "788848550");

    // Part 2
    // solution!(p2, p2_solution, "100");
}
