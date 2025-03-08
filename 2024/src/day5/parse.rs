use std::collections::{HashMap, HashSet};

use nom::{
    IResult,
    character::complete::{self, line_ending},
    combinator::{all_consuming, opt},
    multi::{fold_many0, many1, separated_list1},
    sequence::{separated_pair, terminated},
};

fn page_ordering_rule(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(complete::u32, complete::char('|'), complete::u32)(input)
}

fn page_ordering_rules(input: &str) -> IResult<&str, HashMap<u32, HashSet<u32>>> {
    fold_many0(
        terminated(page_ordering_rule, line_ending),
        HashMap::new,
        |mut acc, (lhs, rhs)| -> HashMap<u32, HashSet<u32>> {
            acc.entry(lhs)
                .and_modify(|m| {
                    m.insert(rhs);
                })
                .or_insert(HashSet::from([rhs]));
            acc
        },
    )(input)
}

fn pages_to_produce(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    many1(terminated(
        separated_list1(complete::char(','), complete::u32),
        opt(line_ending),
    ))(input)
}

type PrintQueue<'a> = IResult<&'a str, (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>)>;
pub(super) fn parse_print_queue(input: &str) -> PrintQueue {
    all_consuming(separated_pair(
        page_ordering_rules,
        line_ending,
        pages_to_produce,
    ))(input)
}
