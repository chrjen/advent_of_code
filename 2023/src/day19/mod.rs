pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 19: Aplenty",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod data;
mod parse;

use std::collections::HashMap;

use self::data::{Part, PartRange, Workflow};

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let (workflows, parts) = parse::parse_input(&input).expect("input should be valid").1;

    // Part 1
    let mut accepted: Vec<Part> = Vec::new();
    for part in parts.iter() {
        let mut current_workflow = workflows
            .get("in")
            .expect("should be a workflow named \"in\"");

        loop {
            match current_workflow.process(part) {
                data::Action::Accept => {
                    accepted.push(part.clone());
                    break;
                }
                data::Action::Reject => break,
                data::Action::Switch(name) => {
                    current_workflow = workflows.get(name).unwrap_or_else(|| {
                        panic!("can't switch to workflow \"{name}\", not found")
                    });
                }
            }
        }
    }
    let part1: u32 = accepted.iter().map(Part::total_rating).sum();

    // Part 2
    let start_workflow = workflows
        .get("in")
        .expect("should be a workflow named \"in\"");

    let mut accepted: Vec<PartRange> = Vec::new();
    calc_workflow(&workflows, start_workflow, &mut accepted, PartRange::new());
    let part2: usize = accepted.iter().map(PartRange::total_combination).sum();

    (part1.to_string(), part2.to_string())
}

fn calc_workflow(
    workflows: &HashMap<&str, Workflow<'_>>,
    current: &Workflow<'_>,
    accepted: &mut Vec<PartRange>,
    mut part: PartRange,
) {
    // Originally this function would return accepted PartRange with overlap,
    // thus causing some combination to be counted more than once. This could be
    // solved by e.g. thinking of each PartRange as a 4D hypercube and try
    // eliminating overlapping volumes. Alternatively I was thinking to split
    // each cube into 3^4 = 81 other cubes based on the intersection. This way
    // I only had to deal with individual non-overlapping cubes.
    //
    // In any case, this solution is a lot simpler. It just applies the rule to
    // the ranges, splitting the PartRange into two, a matching and non-matching
    // part. The rule action is applied to the matching part and the next rule
    // is matched against the non-matching part, keeping track of which
    // PartRange is accepted.
    for rule in current.rules.iter() {
        if part.total_combination() == 0 {
            break;
        }

        let (matching_part, not_matching_part, action) = rule.match_range(part);
        part = not_matching_part;

        if matching_part.total_combination() == 0 {
            continue;
        }

        match action {
            data::Action::Accept => accepted.push(matching_part),
            data::Action::Reject => {}
            data::Action::Switch(name) => {
                let workflow = workflows
                    .get(name)
                    .unwrap_or_else(|| panic!("can't switch to workflow \"{name}\", not found"));
                calc_workflow(workflows, workflow, accepted, matching_part);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        "19114"
    );
    solution!(p1, p1_solution, "391132");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        "167409079868000"
    );
    solution!(p2, p2_solution, "128163929109524");
}
