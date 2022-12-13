mod data;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 13: Distress Signal",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(mut input: &[u8]) -> (String, String) {
    use data::Data;

    let mut packets: Vec<Data> = Vec::new();

    while let (Some(packet), input_) = data::parse_packet(input) {
        input = input_;
        packets.push(packet);
    }

    // Part 1.
    let mut part1 = 0;
    for (i, chunk) in packets.chunks(2).enumerate() {
        let (i, left, right) = (i + 1, &chunk[0], &chunk[1]);
        if left < right {
            part1 += i;
        }
    }

    // Part 2.
    let div1: Data = vec![(vec![2.into()].into())].into();
    let div2: Data = vec![(vec![6.into()].into())].into();
    packets.push(div1.clone());
    packets.push(div2.clone());

    packets.sort_unstable();

    let idx1 = packets.iter().position(|p| p == &div1).unwrap() + 1;
    let idx2 = packets.iter().position(|p| p == &div2).unwrap() + 1;
    let part2 = idx1 * idx2;

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]",
        "13"
    );
    solution!(p1, p1_solution, "5808");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]",
        "140"
    );
    solution!(p2, p2_solution, "22713");
}
