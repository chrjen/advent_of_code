mod data;
mod parse;

use data::Monkey;

type Number = num::Rational64;

pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 21: Monkey Math",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let (_, mut monkeys) = parse::monkeys::<Monkey>(&input).unwrap();
    Monkey::update_parents("root", &mut monkeys);

    // // Uncomment to print a pretty tree representation of the monkeys.
    // Monkey::print_tree("root", &monkeys);

    // Part 1.
    let root = monkeys
        .get("root")
        .unwrap_or_else(|| panic!("root monkey not found"));
    let part1 = root.eval(&monkeys);

    // Part 2.
    // Change "root" monkey's operation to be `Monkey::Eq`.
    monkeys.insert(
        "root",
        match root {
            Monkey::Add(_, lhs, rhs)
            | Monkey::Sub(_, lhs, rhs)
            | Monkey::Mul(_, lhs, rhs)
            | Monkey::Div(_, lhs, rhs) => Monkey::Eq(None, lhs, Some(rhs)),
            Monkey::Eq(_, _, _) => root.clone(),
            Monkey::Literal(_, _) => {
                panic!("root cannot be a literal, need left and right hand sides.")
            }
        },
    );

    let part2 = Monkey::solve_for_unknown("humn", &monkeys);

    (common::from_option(part1), common::from_option(part2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32",
        "152"
    );
    solution!(p1, p1_solution, "364367103397416");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32",
        "301"
    );
    example!(
        p2,
        p2_example_2,
        "root: a - b
a: c + d
b: h - j
c: e / f
d: g * humn
e: 9
f: 3
g: m + n
h: k * l
j: 10
k: 4
l: 5
m: o * p
n: 1
o: 2
p: 3
humn: 4",
        "1"
    );
    example!(
        p2,
        p2_example_3,
        "root: a - b
a: c * d
b: 1
c: humn / f
d: 2
f: 2
humn: 2",
        "1"
    );
    example!(
        p2,
        p2_example_4,
        "root: a - b
a: c / humn
b: 4
c: 36
humn: 2",
        "9"
    );
    example!(
        p2,
        p2_example_5,
        "root: a - b
a: c - humn
b: 4
c: 36
humn: 2",
        "32"
    );
    solution!(p2, p2_solution, "3782852515583");
}
