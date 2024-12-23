pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 23: LAN Party",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

// The Clique problem
// https://en.wikipedia.org/wiki/Clique_problem
pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let mut network: FxHashMap<&str, FxHashSet<&str>> = FxHashMap::default();
    let mut network_pair: FxHashSet<(&str, &str)> = FxHashSet::default();
    for line in input.lines() {
        let (comp1, comp2) = line.split('-').next_tuple().expect("input should be valid");
        network
            .entry(comp1)
            .and_modify(|v| {
                v.insert(comp2);
            })
            .or_insert_with(|| {
                let mut v = FxHashSet::default();
                v.insert(comp2);
                v
            });
        network
            .entry(comp2)
            .and_modify(|v| {
                v.insert(comp1);
            })
            .or_insert_with(|| {
                let mut v = FxHashSet::default();
                v.insert(comp1);
                v
            });
        network_pair.insert((comp1, comp2));
        network_pair.insert((comp2, comp1));
    }
    let network = network;
    let network_pair = network_pair;

    // Part 1
    let mut seen: FxHashSet<String> = FxHashSet::default();
    let mut part1 = 0;

    for (&comp, connected) in network.iter().filter(|(v, _)| v.starts_with('t')) {
        for (&other1, &other2) in connected.iter().tuple_combinations() {
            if network_pair.contains(&(other1, other2)) {
                let clique = &mut [comp, other1, other2];
                clique.sort_unstable();
                let clique = clique.join(",");

                if !seen.contains(&clique) {
                    // println!("Found set: {},{},{}", comp, other1, other2);
                    seen.insert(clique);
                    part1 += 1;
                }
            }
        }
    }

    // Part 2
    let part2 = bron_kerbosch(
        &network,
        FxHashSet::default(),
        network.keys().copied().collect(),
        FxHashSet::default(),
    )
    .into_iter()
    .sorted()
    .join(",");

    (part1.to_string(), part2.to_string())
}

/// Implementation of the Bron–Kerbosch algorithm for finding all maximal
/// cliques. This implementation returns the biggest of all maximal cliques.
///
/// [Wikipedia](en.wikipedia.org/wiki/Bron–Kerbosch_algorithm)
fn bron_kerbosch<'a>(
    neighbours: &FxHashMap<&'a str, FxHashSet<&'a str>>,
    r: FxHashSet<&'a str>,
    mut p: FxHashSet<&'a str>,
    mut x: FxHashSet<&'a str>,
) -> FxHashSet<&'a str> {
    if p.is_empty() && x.is_empty() {
        return r;
    }

    let mut biggest: FxHashSet<&'a str> = FxHashSet::default();

    while !p.is_empty() {
        let v = *p.iter().next().unwrap();

        let mut rr = r.clone();
        rr.insert(v);

        let pp: FxHashSet<&str> = p
            .intersection(neighbours.get(v).unwrap())
            .copied()
            .collect();

        let xx: FxHashSet<&str> = x
            .intersection(neighbours.get(v).unwrap())
            .copied()
            .collect();

        let found = bron_kerbosch(neighbours, rr, pp, xx);
        if found.len() > biggest.len() {
            biggest = found;
        }

        p.remove(v);
        x.insert(v);
    }

    biggest
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn",
        "7"
    );
    solution!(p1, p1_solution, "1064");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn",
        "co,de,ka,ta"
    );
    solution!(p2, p2_solution, "aq,cc,ea,gc,jo,od,pa,rg,rv,ub,ul,vr,yy");
}
