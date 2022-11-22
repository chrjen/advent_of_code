use std::collections::HashMap;

use regex::Regex;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 9: All in a Single Night",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    // Heap's algorithm, for going through all permutations.
    // https://en.wikipedia.org/wiki/Heap%27s_algorithm
    fn permute<F, T>(v: &mut Vec<T>, f: &mut F)
    where
        F: FnMut(&Vec<T>),
    {
        if v.is_empty() {
            return;
        }

        fn permute_<F, T>(k: usize, v: &mut Vec<T>, f: &mut F)
        where
            F: FnMut(&Vec<T>),
        {
            if k == 1 {
                f(&*v);
                return;
            }

            permute_(k - 1, v, f);

            for i in 0..k - 1 {
                if k % 2 == 0 {
                    v.swap(i, k - 1);
                } else {
                    v.swap(0, k - 1);
                }
                permute_(k - 1, v, f)
            }
        }
        permute_(v.len(), v, f);
    }

    let mut distances: HashMap<String, HashMap<String, u32>> = HashMap::new();

    let reg = Regex::new(r"(\w+) to (\w+)\s*=\s*(\d+)").unwrap();
    for cap in reg.captures_iter(&input) {
        let (from, to, dist) = (&cap[1], &cap[2], &cap[3].parse().unwrap());

        if let Some(dests) = distances.get_mut(from) {
            dests.insert(to.to_owned(), *dist);
        } else {
            let mut dests = HashMap::new();
            dests.insert(to.to_owned(), *dist);
            distances.insert(from.to_owned(), dests);
        }

        if let Some(dests) = distances.get_mut(to) {
            dests.insert(from.to_owned(), *dist);
        } else {
            let mut dests = HashMap::new();
            dests.insert(from.to_owned(), *dist);
            distances.insert(to.to_owned(), dests);
        }
    }

    let mut cities: Vec<&String> = distances.keys().collect();

    let get_dist = |v: &Vec<&String>| -> u32 {
        let mut total = 0;
        for pair in v.windows(2) {
            let (from, to) = (pair[0], pair[1]);
            total += distances.get(from).unwrap().get(to).unwrap();
        }

        // // Print all routes.
        // for c in v {
        //     print!("{c} -> ");
        // }
        // println!("{}", total);

        total
    };

    // Main body.
    // Uses brute force to go through all possible paths and stores the largest
    // and smallest distances.
    let mut min_dist = u32::MAX;
    let mut max_dist = 0;
    permute(&mut cities, &mut |v| {
        let dist = get_dist(v);
        min_dist = min_dist.min(dist);
        max_dist = max_dist.max(dist);
    });

    (min_dist.to_string(), max_dist.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        r#"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"#,
        "605"
    );
    solution!(p1, p1_solution, "207", ignore = "takes too long to run");

    // Part 2
    example!(
        p2,
        p2_example_1,
        r#"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"#,
        "982"
    );
    solution!(p2, p2_solution, "804", ignore = "takes too long to run");
}
