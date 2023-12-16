pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 8: Treetop Tree House",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let grid: Vec<u8> = input
        .lines()
        .flat_map(|s| s.chars())
        .map(|c| {
            if !c.is_ascii_digit() {
                panic!("none ascii character given for height '{}'", c);
            }
            c as u8 - b'0'
        })
        .collect();

    let width = input.lines().next().unwrap().len();
    let height = grid.len() / width;

    if grid.len() % width != 0 {
        panic!(
            "grid is not a perfect rectangle got {}x{} grid, but {} trees",
            width,
            height,
            grid.len()
        );
    }

    // Part 1.
    let mut visible = vec![false; width * height];

    // Left to right.
    for y in 0..height {
        let mut highest;

        highest = grid[y * width];
        visible[y * width] = true;

        for x in 1..width {
            if grid[y * width + x] > highest {
                highest = grid[y * width + x];
                visible[y * width + x] = true;
            }
        }
    }

    // Right to left.
    for y in 0..height {
        let mut highest;

        highest = grid[y * width + (width - 1)];
        visible[y * width + (width - 1)] = true;

        for x in (0..width).rev() {
            if grid[y * width + x] > highest {
                highest = grid[y * width + x];
                visible[y * width + x] = true;
            }
        }
    }

    // Top to bottom.
    for x in 0..width {
        let mut highest;

        highest = grid[x];
        visible[x] = true;

        for y in 1..height {
            if grid[y * width + x] > highest {
                highest = grid[y * width + x];
                visible[y * width + x] = true;
            }
        }
    }

    // Bottom to top.
    for x in 0..width {
        let mut highest;

        highest = grid[(height - 1) * width + x];
        visible[(height - 1) * width + x] = true;

        for y in (0..height).rev() {
            if grid[y * width + x] > highest {
                highest = grid[y * width + x];
                visible[y * width + x] = true;
            }
        }
    }

    // Part 2.
    let mut tree_scores = vec![0; width * height];

    for y in 0..height {
        for x in 0..width {
            let tree = grid[y * width + x];
            let mut score = 1;

            // Right.
            let mut tmp_score = 0;
            for xx in x + 1..width {
                tmp_score += 1;
                if tree <= grid[y * width + xx] {
                    break;
                }
            }
            score *= tmp_score;

            // Left.
            let mut tmp_score = 0;
            for xx in (0..x).rev() {
                tmp_score += 1;
                if tree <= grid[y * width + xx] {
                    break;
                }
            }
            score *= tmp_score;

            // Top.
            let mut tmp_score = 0;
            for yy in y + 1..height {
                tmp_score += 1;
                if tree <= grid[yy * width + x] {
                    break;
                }
            }
            score *= tmp_score;

            // Bottom.
            let mut tmp_score = 0;
            for yy in (0..y).rev() {
                tmp_score += 1;
                if tree <= grid[yy * width + x] {
                    break;
                }
            }
            score *= tmp_score;

            tree_scores[y * width + x] = score;
        }
    }

    let (_best_tree_idx, best_tree_score) = tree_scores
        .iter()
        .enumerate()
        .max_by_key(|&(_, x)| x)
        .unwrap();

    // // Uncomment to draw a map of all visible trees, as well as the location of
    // // the best location for a tree house.
    // for y in 0..height {
    //     for x in 0..width {
    //         let idx = y * width + x;
    //         if idx == _best_tree_idx {
    //             print!("@");
    //         } else if visible[y * width + x] {
    //             print!("T");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    (
        visible.iter().filter(|v| **v).count().to_string(),
        best_tree_score.to_string(),
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
        "30373
25512
65332
33549
35390",
        "21"
    );
    solution!(p1, p1_solution, "1814");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "30373
25512
65332
33549
35390",
        "8"
    );
    solution!(p2, p2_solution, "330786");
}
