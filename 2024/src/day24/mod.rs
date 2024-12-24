pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 24: Crossed Wires",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod data;
mod parse;

use data::LogicGate;
use fxhash::FxHashMap;
use std::{fs::File, io::Write};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let (logic_inputs_original, logic_gates) =
        parse::parse_input(input).expect("input should be valid");
    let mut logic_inputs = logic_inputs_original.clone();

    // Part 1
    let mut part1: u64 = 0;
    for (i, node) in (0..).map(|i| (i, format!("z{:02}", i))) {
        let Some(output) = recursive_eval(&logic_gates, &mut logic_inputs, &node) else {
            break;
        };

        part1 |= (output as u64) << i;
    }

    // Part 2
    // This part was solved manually by drawing the logical circuit and inspecting it as well
    // as running the circuit and comparing the output with the expected output.

    // let x: u64 = {
    //     let mut x = 0;
    //     for (i, node) in (0..).map(|i| (i, format!("x{:02}", i))) {
    //         let Some(output) = recursive_eval(&logic_gates, &mut logic_inputs, &node) else {
    //             break;
    //         };

    //         x |= (output as u64) << i;
    //     }
    //     x
    // };

    // let y: u64 = {
    //     let mut y = 0;
    //     for (i, node) in (0..).map(|i| (i, format!("y{:02}", i))) {
    //         let Some(output) = recursive_eval(&logic_gates, &mut logic_inputs, &node) else {
    //             break;
    //         };

    //         y |= (output as u64) << i;
    //     }
    //     y
    // };

    // if (x + y) != part1 {
    //     println!("!!EXPECT!! {} + {} = {}", x, y, x + y);
    //     println!("  GOT      {} + {} = {}", x, y, part1);
    // }

    // write_graphviz_dot_file(&logic_gates, &logic_inputs);

    (
        part1.to_string(),
        "dwp,ffj,gjh,jdr,kfm,z08,z22,z31".to_string(), // Manually solved.
    )
}

fn recursive_eval(
    gates: &[LogicGate],
    nodes: &mut FxHashMap<&str, bool>,
    node: &str,
) -> Option<bool> {
    if let Some(&state) = nodes.get(node) {
        return Some(state);
    }

    let gate = gates.iter().find(|v| v.output_node() == node)?;

    let left = nodes
        .get(gate.left_input_node())
        .copied()
        .or_else(|| recursive_eval(gates, nodes, gate.left_input_node()))?;
    let right = nodes
        .get(gate.right_input_node())
        .copied()
        .or_else(|| recursive_eval(gates, nodes, gate.right_input_node()))?;

    match gate {
        LogicGate::And(_, _, _) => Some(left && right),
        LogicGate::Or(_, _, _) => Some(left || right),
        LogicGate::Xor(_, _, _) => Some(left ^ right),
    }
}

/// Writes out a graph file in graphviz format that can be used to  visualise the
/// logic circuit and aid in manually finding the solution.
///
/// Run the following command to convert the .dot file to a .pdf file. (You need
/// to have graphviz CLI utilities installed.)
/// `dot -Tpdf -o 2024/src/day24/input.pdf 2024/src/day24/input.dot`
#[allow(dead_code)]
fn write_graphviz_dot_file(gates: &[LogicGate], logic_inputs: &FxHashMap<&str, bool>) {
    let mut dot_file = File::create("2024/src/day24/input.dot").unwrap();

    writeln!(dot_file, "digraph G {{").unwrap();

    for &input in logic_inputs.keys() {
        if input.starts_with('x') {
            writeln!(dot_file, "{} [color = \"#0000ff\"];", input).unwrap();
        } else {
            writeln!(dot_file, "{} [color = \"#00ff00\"];", input).unwrap();
        }
    }

    let swapped = &["dwp", "ffj", "gjh", "jdr", "kfm", "z08", "z22", "z31"];
    for gate in gates.iter() {
        let output = gate.output_node();
        match (swapped.contains(&output), output.starts_with('z')) {
            (true, true) => writeln!(
                dot_file,
                "{} [style=filled color=\"#ffff00\" fillcolor=\"#ff0000\"];",
                output
            )
            .unwrap(),
            (true, false) => {
                writeln!(dot_file, "{} [style=filled fillcolor=\"#ff0000\"];", output).unwrap()
            }
            (false, true) => writeln!(dot_file, "{} [color = \"#ffff00\"];", output).unwrap(),
            (false, false) => {}
        }
    }

    for (i, gate) in gates.iter().enumerate() {
        match gate {
            LogicGate::And(left, right, output) => {
                writeln!(
                    dot_file,
                    "GATE{i} [ label=\"AND\" shape=box style=filled color=\"#9bb6e0\" ];"
                )
                .unwrap();
                writeln!(dot_file, "{left} -> GATE{i};").unwrap();
                writeln!(dot_file, "{right} -> GATE{i};").unwrap();
                writeln!(dot_file, "GATE{i} -> {output};").unwrap();
            }
            LogicGate::Or(left, right, output) => {
                writeln!(
                    dot_file,
                    "GATE{i} [ label=\"OR\" shape=box style=filled color=\"#9bb6e0\" ];"
                )
                .unwrap();
                writeln!(dot_file, "{left} -> GATE{i};").unwrap();
                writeln!(dot_file, "{right} -> GATE{i};").unwrap();
                writeln!(dot_file, "GATE{i} -> {output};").unwrap();
            }
            LogicGate::Xor(left, right, output) => {
                writeln!(
                    dot_file,
                    "GATE{i} [ label=\"XOR\" shape=box style=filled color=\"#9bb6e0\" ];"
                )
                .unwrap();
                writeln!(dot_file, "{left} -> GATE{i};").unwrap();
                writeln!(dot_file, "{right} -> GATE{i};").unwrap();
                writeln!(dot_file, "GATE{i} -> {output};").unwrap();
            }
        }
    }

    let zs = (0..45)
        .map(|i| format!("z{i:02}"))
        .collect::<Vec<_>>()
        .join(" ");

    writeln!(dot_file, "{{ rank=same; {zs} }}").unwrap();

    writeln!(dot_file, "}}").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02",
        "4"
    );
    example!(
        p1,
        p1_example_2,
        "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj",
        "2024"
    );
    solution!(p1, p1_solution, "63168299811048");

    // Part 2
    solution!(p2, p2_solution, "dwp,ffj,gjh,jdr,kfm,z08,z22,z31");
}
