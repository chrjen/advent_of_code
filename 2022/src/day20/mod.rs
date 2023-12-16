pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 20: Grove Positioning System",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

const SECRET_DECRYPTION_KEY: isize = 811589153;

/// Mixes the original numbers in a circular way. The original `Vec` is left
/// intact, instead a new `Vec` is returned containing the indices of the
/// numbers in the original `Vec`. So if the original was `[1, -2, 0, 3]` and
/// would after mixing becomes `[3, -2, 0, 1]` then this function returns
/// `[3, 1, 2, 0]` where `0` represents the zeroth number in the original,
/// e.i. `1`; `1` is `-2`, `2` is `0` and `3` is `3`.
fn mix(original: &[isize], times: usize) -> Vec<usize> {
    let mut sequence: Vec<usize> = (0..original.len()).collect();

    for _ in 0..times {
        for (i, &n) in original.iter().enumerate() {
            let idx = sequence.iter().position(|&v| v == i).unwrap();
            sequence.remove(idx);

            let new_idx = (idx as isize + n).rem_euclid(sequence.len() as isize) as usize;
            sequence.insert(new_idx, i);
        }
    }

    sequence
}

/// Decrypts a sequence of numbers given a `key` and `times`. `times` is the
/// number of times to mix the input sequence. Returns the original coordinates
/// before being encrypted.
fn decrypt(original: &[isize], key: isize, times: usize) -> isize {
    let original: Vec<isize> = original.iter().map(|v| key * v).collect();
    let sequence = mix(&original, times)
        .iter()
        .map(|&v| original[v])
        .collect::<Vec<_>>();

    let idx_zero = sequence.iter().position(|&v| v == 0).unwrap();
    sequence[(idx_zero + 1000) % sequence.len()]
        + sequence[(idx_zero + 2000) % sequence.len()]
        + sequence[(idx_zero + 3000) % sequence.len()]
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let original: Vec<isize> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.parse()
                .unwrap_or_else(|_| panic!("failed to parse line {i}: \"{line}\""))
        })
        .collect();

    (
        decrypt(&original, 1, 1).to_string(),
        decrypt(&original, SECRET_DECRYPTION_KEY, 10).to_string(),
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
        "1
2
-3
3
-2
0
4",
        "3"
    );
    example!(
        p1,
        p1_example_2,
        "7
14
-21
21
-14
0
28",
        "21"
    );
    solution!(p1, p1_solution, "13883", ignore = "takes too long");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "1
2
-3
3
-2
0
4",
        "1623178306"
    );
    solution!(p2, p2_solution, "19185967576920", ignore = "takes too long");
}
