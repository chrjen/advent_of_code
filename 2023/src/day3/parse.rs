use std::rc::Rc;

use nom::{
    IResult,
    branch::alt,
    bytes::complete::take_while,
    character::complete::{digit1, one_of},
    combinator::{all_consuming, map},
};

use super::{Engine, Part};

pub(super) fn take_void(input: &str) -> IResult<&str, usize> {
    map(take_while(|c| c == '.'), |s: &str| s.len())(input)
}

pub(super) fn parse_engine_part(input: &str) -> IResult<&str, (Part, usize)> {
    map(digit1, |s: &str| {
        (Part::Number(s.parse::<u32>().unwrap()), s.len())
    })(input)
}

pub(super) fn parse_engine_symbol(input: &str) -> IResult<&str, (Part, usize)> {
    map(one_of("+-/*@#$=&%"), |c: char| (Part::Symbol(c), 1))(input)
}

fn _parse_engine(mut input: &str) -> IResult<&str, Engine> {
    let mut engine = Engine::new();

    let mut y: usize = 1;
    for line in input.lines() {
        let mut x: usize = 1;
        let mut n;
        let mut part;

        (input, n) = take_void(line)?;
        x += n;

        // println!("LINE: \"{line}\" | INPUT: \"{input}\"");

        while !input.is_empty() {
            (input, (part, n)) = alt((parse_engine_part, parse_engine_symbol))(input)?;
            // println!("\tPART: {part:?} | N: \"{n}\"");
            let part = Rc::new(part);
            for xx in x..x + n {
                engine.layout.insert((xx, y), part.clone());
            }
            x += n;

            (input, n) = take_void(input)?;
            x += n;
        }

        y += 1;
    }

    Ok((input, engine))
}

pub(super) fn parse_engine(input: &str) -> IResult<&str, Engine> {
    all_consuming(_parse_engine)(input)
}
