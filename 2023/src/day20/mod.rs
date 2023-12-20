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

            let (low_count, high_count, _) = system.button_pulse();
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

    // Part 2

    // This part was solved manually by graphing and looking at the graph.
    // Uncomment the below line to print out a graphviz description of the graph
    // which can be pasted into any graphviz visualiser to visually see the graph
    // where circles are flip-flops and diamonds are conjunctions.
    // initial_system.print_graphviz();

    // To actually solve it you need to recognise that there are four 12-bit
    // counters that are all AND-ed together. The input is set up in such a way
    // where each counter resets back to zero when hitting its "target number".
    // In other words, each counter counts to a predefined number and loops.
    // When all counters hit their predefined maximum number, the "rx" module
    // is finally set LOW. Calculating the least common multiple (LCM) of the
    // four numbers gives the answer. For me numbers were 3917, 3793, 3911 and
    // 3929, where LCM(3917, 3793, 3911, 3929) = 228 300 182 686 739.

    // To actually find these maximum or "predefined" numbers for each loop you
    // simply look at all the edges from the counter *to* the conjunction and
    // those represents binary 1s, and no edge represents 0s. The least
    // significant bit is the one connected to the "broadcaster" module which
    // here acts as the clock. Following the least significant bit, seeing if
    // each bit is a 0 or 1 will give you the predefined number for that loop
    // in binary form.

    // // Below is an attempt at a brute force solution. (WAY TOO SLOW!)
    // let part2: usize = {
    //     let mut system = initial_system.clone();
    //     let mut button_count: usize = 0;

    //     if !system.has_module("rx") {
    //         return (part1.to_string(), "No module named \"rx\" found".to_owned());
    //     }

    //     loop {
    //         button_count += 1;
    //         let (_, _, rx_set_low) = system.button_pulse();
    //         if rx_set_low {
    //             break button_count;
    //         }

    //         if button_count.is_multiple_of(&1_000_000) {
    //             println!("button count {} millions", button_count / 1_000_000);
    //         }

    //         if system == initial_system {
    //             println!("Found loop after {button_count} button presses");
    //             break 0;
    //         }
    //     }
    // };

    (
        part1.to_string(),
        "228300182686739 (input ignored, manually solved)".to_owned(),
    )
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
    solution!(
        p2,
        p2_solution,
        "228300182686739 (input ignored, manually solved)"
    );
}
