use std::{collections::HashMap, fmt};

use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::{
        complete::char,
        complete::{alpha1, line_ending, one_of, space0},
    },
    multi::separated_list0,
    sequence::{delimited, terminated, tuple},
    IResult, Parser,
};

pub fn monkeys<'a, T>(input: &'a str) -> IResult<&str, HashMap<&str, T>>
where
    T: From<(&'a str, char, &'a str)> + TryFrom<&'a str>,
    <T as TryFrom<&'a str>>::Error: fmt::Debug,
{
    let mut hash_map = HashMap::new();
    let (input, monkeys) = separated_list0(line_ending, monkey::<T>)(input)?;
    for (name, monkey) in monkeys.into_iter() {
        hash_map.insert(name, monkey);
    }
    Ok((input, hash_map))
}

pub fn monkey<'a, T>(input: &'a str) -> IResult<&str, (&str, T)>
where
    T: From<(&'a str, char, &'a str)> + TryFrom<&'a str>,
    <T as TryFrom<&'a str>>::Error: fmt::Debug,
{
    let (input, name) = terminated(alpha1, char(':'))(input)?;
    let (input, _) = space0(input)?;
    let (input, monkey) = alt((
        take_while1(|c: char| c.is_ascii_digit() || matches!(c, '-' | '.' | '/'))
            .map(|n| T::try_from(n).unwrap_or_else(|_| panic!("fuck, got '{n}'"))),
        tuple((alpha1, delimited(space0, one_of("+-*/"), space0), alpha1)).map(|t| T::from(t)),
    ))(input)?;

    Ok((input, (name, monkey)))
}
