use std::collections::HashMap;

use regex::Regex;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 13: Knights of the Dinner Table",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

/// Based on the solution for 2015 — Day 9.
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

    let mut guest_happiness: HashMap<String, HashMap<String, i32>> = HashMap::new();

    let reg =
        Regex::new(r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).")
            .unwrap();
    for cap in reg.captures_iter(&input) {
        let (from, to, action, mut happiness) =
            (&cap[1], &cap[4], &cap[2], cap[3].parse::<i32>().unwrap());

        match action {
            "gain" => {}
            "lose" => happiness = -happiness,
            a => panic!("unknown action '{}'", a),
        }

        if let Some(guest) = guest_happiness.get_mut(from) {
            guest.insert(to.to_owned(), happiness);
        } else {
            let mut guest = HashMap::new();
            guest.insert(to.to_owned(), happiness);
            guest_happiness.insert(from.to_owned(), guest);
        }
    }

    let get_dist = |v: &Vec<String>, happiness: &HashMap<String, HashMap<String, i32>>| -> i32 {
        let len = v.len();
        let mut total = 0;

        for i in 0..len {
            let (from, to) = (&v[i], &v[(i + 1) % len]);
            total += happiness.get(from).unwrap().get(to).unwrap();
            total += happiness.get(to).unwrap().get(from).unwrap();
        }

        // Print all routes.
        // for c in v {
        //     print!("{c} -> ");
        // }
        // println!("{}", total);

        total
    };

    // Uses brute force to go through all possible permutations and stores the largest
    // and smallest happiness.
    // Part 1.
    let mut guests: Vec<String> = guest_happiness.keys().cloned().collect();
    let mut max_happy = i32::MIN;
    permute(&mut guests, &mut |v| {
        let dist = get_dist(v, &guest_happiness);
        max_happy = max_happy.max(dist);
    });

    // Add me to the list of guests.
    let me = String::from("me");
    let mut me_happiness = HashMap::<String, i32>::new();
    for guest in guests.iter() {
        me_happiness.insert(guest.clone(), 0);
        guest_happiness
            .get_mut(guest)
            .unwrap()
            .insert(me.clone(), 0);
    }
    guest_happiness.insert(me.clone(), me_happiness);
    guests.push(me);

    // Part 2.
    let mut max_happy_me = i32::MIN;
    permute(&mut guests, &mut |v| {
        let dist = get_dist(v, &guest_happiness);
        max_happy_me = max_happy_me.max(dist);
    });

    (max_happy.to_string(), max_happy_me.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."#,
        "330"
    );
    solution!(p1, p1_solution, "664", ignore = "takes too long to run");

    // Part 2
    solution!(p2, p2_solution, "640", ignore = "takes too long to run");
}
