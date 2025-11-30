use std::collections::HashMap;

use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{self, alpha1, line_ending, one_of},
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair, tuple},
};

use super::data::{Action, Category, Part, Rule, Workflow};

pub(super) fn part(input: &str) -> IResult<&str, Part> {
    let attribute = separated_pair(complete::one_of("xmas"), complete::char('='), complete::u32);
    let part_parser = separated_list0(complete::char(','), attribute).map(|attributes| Part {
        cool: attributes
            .iter()
            .find_map(|&(c, value)| (c == 'x').then_some(value))
            .expect("part has 'x'"),
        musical: attributes
            .iter()
            .find_map(|&(c, value)| (c == 'm').then_some(value))
            .expect("part has 'm'"),
        aerodynamic: attributes
            .iter()
            .find_map(|&(c, value)| (c == 'a').then_some(value))
            .expect("part has 'a'"),
        shiny: attributes
            .iter()
            .find_map(|&(c, value)| (c == 's').then_some(value))
            .expect("part has 's'"),
    });

    delimited(complete::char('{'), part_parser, complete::char('}'))(input)
}

pub(super) fn parts(input: &str) -> IResult<&str, Box<[Part]>> {
    map(separated_list0(line_ending, part), |v| v.into_boxed_slice())(input)
}

pub(super) fn action(input: &str) -> IResult<&str, Action<'_>> {
    map(alpha1, |action| match action {
        "A" => Action::Accept,
        "R" => Action::Reject,
        workflow_name => Action::Switch(workflow_name),
    })(input)
}

pub(super) fn rule(input: &str) -> IResult<&str, Rule<'_>> {
    let compare_rule = tuple((
        one_of("xmas"),
        one_of("><"),
        complete::u32,
        preceded(complete::char(':'), action),
    ))
    .map(|(category, op, value, action)| {
        let category = match category {
            'x' => Category::CoolLooking,
            'm' => Category::Musical,
            'a' => Category::Aerodynamic,
            's' => Category::Shiny,
            _ => unreachable!(),
        };
        match op {
            '>' => Rule::Greater(category, value, action),
            '<' => Rule::Less(category, value, action),
            _ => unreachable!(),
        }
    });

    let default_rule = action.map(Rule::Default);

    alt((compare_rule, default_rule))(input)
}

pub(super) fn rules(input: &str) -> IResult<&str, Box<[Rule<'_>]>> {
    map(separated_list0(complete::char(','), rule), |v| {
        v.into_boxed_slice()
    })(input)
}

pub(super) fn workflow(input: &str) -> IResult<&str, Workflow<'_>> {
    let rules = delimited(complete::char('{'), rules, complete::char('}'));
    map(tuple((alpha1, rules)), |(name, rules)| Workflow {
        name,
        rules,
    })(input)
}

pub(super) fn workflows(input: &str) -> IResult<&str, HashMap<&str, Workflow<'_>>> {
    map(separated_list0(line_ending, workflow), |v| {
        v.into_iter()
            .map(|workflow| (workflow.name, workflow))
            .collect()
    })(input)
}

#[allow(clippy::type_complexity)]
pub(super) fn parse_input(
    input: &str,
) -> IResult<&str, (HashMap<&str, Workflow<'_>>, Box<[Part]>)> {
    all_consuming(separated_pair(
        workflows,
        tuple((line_ending, line_ending)),
        parts,
    ))(input)
}
