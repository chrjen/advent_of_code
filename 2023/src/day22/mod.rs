pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 22: Sand Slabs",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod data;
mod parse;

use self::data::Block;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let mut blocks: Vec<Block> = parse::blocks(&input).expect("input should be valid").1;

    // Part 1
    let part1 = Block::fall_all(blocks.as_mut_slice());

    // for block in blocks {
    //     println!("{},", block._to_python_value());
    // }

    (part1.to_string(), 0.to_string())
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9",
        "5"
    );
    example!(
        p1,
        p1_example_2,
        "\
0,0,2~0,0,4
0,0,5~0,0,7
0,0,8~0,0,10
0,0,11~2,0,11
0,0,12~0,0,14
1,0,6~1,2,6
1,0,13~1,2,13",
        "3"
    );
    example!(
        p1,
        p1_example_3,
        "\
9,0,1~9,2,1
1,5,1~1,5,3
9,5,1~9,5,3
6,6,1~8,6,1
1,8,1~4,8,1
7,8,1~8,8,1
1,9,1~2,9,1
0,0,2~3,0,2
6,0,2~7,0,2
1,2,2~4,2,2
7,2,2~7,2,2
0,4,2~0,6,2
3,6,2~5,6,2
2,7,2~2,9,2
0,0,3~0,0,4
1,0,3~3,0,3
9,0,3~9,2,3
7,1,3~8,1,3
8,2,3~8,2,6
6,3,3~6,5,3
7,3,3~7,3,5
7,4,3~9,4,3
5,7,3~7,7,3
8,8,3~8,9,3
9,8,3~9,8,4
2,9,3~5,9,3
6,0,4~6,0,4
3,2,4~3,5,4
1,8,4~1,8,6
3,0,5~5,0,5
4,1,5~7,1,5
0,6,5~2,6,5
1,0,6~4,0,6
4,1,6~4,2,6
9,3,6~9,7,6
8,4,6~8,7,6
2,5,6~5,5,6
6,5,6~6,6,6
0,6,6~0,8,6
3,7,6~6,7,6
4,0,7~5,0,7
5,1,7~7,1,7
2,4,7~4,4,7
7,5,7~8,5,7
6,0,8~6,1,8
2,2,8~2,4,8
9,4,8~9,6,8
0,5,8~3,5,8
4,5,8~4,7,8
6,5,8~6,8,8
0,6,8~0,8,8
8,0,9~8,2,9
2,2,9~4,2,9
0,3,9~2,3,9
8,3,9~8,4,9
4,4,9~7,4,9
1,5,9~1,7,9
5,7,9~5,7,12",
        "31"
    );
    solution!(p1, p1_solution, "386");

    // Part 2
    // example!(p2, p2_example_1, "", "0");
    // example!(p2, p2_example_2, "", "0");
    // example!(p2, p2_example_3, "", "0");
    // example!(p2, p2_example_4, "", "0");
    // example!(p2, p2_example_5, "", "0");
    // solution!(p2, p2_solution, "100");
}
