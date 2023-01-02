mod data;
mod parse;

use itertools::{Either, Itertools};
use petgraph::{algo::dijkstra::dijkstra, prelude::*};
use rayon::prelude::*;

use std::collections::{HashMap, VecDeque};

use data::*;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 16: Proboscidea Volcanium",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

const START: &str = "AA";
const TIME_OPEN_VALVE: i32 = 1;
const TIME_BETWEEN_NODES: i32 = 1;

/// Takes a start node, a set of other nodes to use, time left not to exceed and
/// a graph with weights between all the nodes. Returns the max pressure possible
/// to release with the given input.
///
/// Uses Dynamic Programming to reduce the problem into sub-problems,
/// recursively calling itself with one fewer nodes. With the nodes A, B, C and
/// `A` as the start node, the first call to this function would be `(A, {B, C})`.
/// This function would then recursively call itself with each node in `others`
/// as the start node, removing itself from the set `others`, e.i. `(B, {C})`
/// and `(C, {B})`. Lastly each of those call `(C, {})` and `(B, {})`,
/// respectively. On each return only the biggest value is saved and added
/// to the start node
fn max_pressure(
    start: Node,
    others: &mut VecDeque<Node>,
    time_left: i32,
    graph: &GraphMap<Node, i32, Undirected>,
) -> i32 {
    let mut pressure = 0;

    for _ in 0..others.len() {
        let next = others.pop_front().unwrap();

        let time_left = time_left - *graph.edge_weight(start, next).unwrap() - TIME_OPEN_VALVE;
        if !time_left.is_negative() {
            pressure = pressure.max(max_pressure(next, others, time_left, graph));
        }

        others.push_back(next);
    }

    pressure + time_left * start.flow_rate as i32
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let (_, result) = parse::valve_specs_parser(&input).unwrap();

    let mut graph: UnGraphMap<Node, i32> = UnGraphMap::new();
    let nodes: HashMap<&str, Node> = result
        .iter()
        .map(|&(name, flow_rate, _)| (name, Node { name, flow_rate }))
        .collect();

    // Populate the graph with the parsed input.
    for (name, _, neighbours) in result.iter() {
        for &neighbour in neighbours.iter() {
            graph.add_edge(nodes[name], nodes[neighbour], TIME_BETWEEN_NODES);
        }
    }

    // Collapse and remove/dissolve nodes with flow_rate == 0, except the start node.
    for (_, &node) in nodes
        .iter()
        .filter(|(_, n)| n.flow_rate == 0 && n.name != START)
    {
        for (a, b) in graph
            .neighbors(node)
            .collect::<Vec<_>>()
            .into_iter()
            .combinations(2)
            .map(|v| (v[0], v[1]))
        {
            let weight = graph.edge_weight(node, a).unwrap() + graph.edge_weight(node, b).unwrap();
            graph.add_edge(a, b, weight);
        }
        graph.remove_node(node);
    }

    // Create a complete graph connecting all the nodes in `graph` with the time
    // in minutes it takes to go between any node and any other nodes.
    let mut complete_graph: UnGraphMap<Node, i32> = UnGraphMap::new();
    for start in graph.nodes() {
        for (node, len) in dijkstra(&graph, start, None, |(_, _, &weight)| weight) {
            if node != start {
                complete_graph.add_edge(start, node, len);
            }
        }
    }

    let complete_graph = complete_graph;
    let start = nodes[START];
    let others = complete_graph
        .nodes()
        .filter(|n| n != &start)
        .collect::<VecDeque<_>>();

    // // Uncomment to print out a Graphvis graph, in DOT language, of complete_graph.
    // println!(
    //     "{:?}",
    //     petgraph::dot::Dot::with_config(&complete_graph, &[/* Config::EdgeNoLabel */])
    // );

    // Part 1.
    let part1 = max_pressure(start, &mut others.clone(), 30, &complete_graph);

    // Part 2.
    // With two persons all nodes are partitioned in two sets and each person
    // independently calculates `max_pressure` on only those nodes. The total
    // pressure released is the sum of what the two persons released individually.
    //
    // A partition is represented as a binary number with bits showing which set
    // a node belongs to. Example, with four nodes and the binary number `0110`
    // then first node belongs to the elf, second and third to the elephant, and
    // the fourth to the elf. Since we don't care about who gets each set, e.i.
    // `0110` is the same as the bitwise inverse `1001`, we get 2^(n-1) total
    // possible partitions for `n` nodes. We then iterate through all possible
    // partitions taking the result from the partition with highest result.
    let num_partitions = 1 << (others.len() - 1);
    let part2 = (0..num_partitions)
        .into_par_iter()
        .map(|partition| {
            // println!("partition: {partition}/{num_partitions}");
            let (mut elf, mut elephant): (VecDeque<_>, VecDeque<_>) =
                others.iter().copied().enumerate().partition_map(|(i, n)| {
                    if partition & 1 << i == 0 {
                        Either::Left(n)
                    } else {
                        Either::Right(n)
                    }
                });

            let elf_max = max_pressure(start, &mut elf, 26, &complete_graph);
            let elephant_max = max_pressure(start, &mut elephant, 26, &complete_graph);

            elf_max + elephant_max
        })
        .max()
        .unwrap();

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
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II",
        "1651"
    );
    solution!(p1, p1_solution, "1845", ignore = "takes too long");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II",
        "1707"
    );
    solution!(p2, p2_solution, "2286", ignore = "takes too long");
}
