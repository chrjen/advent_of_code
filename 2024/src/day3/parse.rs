use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{self},
    combinator::map,
    multi::many0,
    sequence::{delimited, pair, separated_pair},
    IResult,
};

use super::data::Mul;

pub(super) fn parse_args_two(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(complete::u32, complete::char(','), complete::u32)(input)
}

pub(super) fn parse_mul_instr(input: &str) -> IResult<&str, Mul> {
    map(
        delimited(complete::char('('), parse_args_two, complete::char(')')),
        |(lhs, rhs)| Mul(lhs, rhs),
    )(input)
}

pub(super) fn parse_next_instr(mut input: &str) -> IResult<&str, Mul> {
    while !input.is_empty() {
        (input, _) = pair(take_until("mul"), tag("mul"))(input)?;

        match parse_mul_instr(input) {
            Ok((input, instr)) => return Ok((input, instr)),
            Err(nom::Err::Error(kek)) if kek.code != nom::error::ErrorKind::Eof => continue,
            Err(err) => return Err(err),
        }
    }
    Err(nom::Err::Error(nom::error::Error::new(
        "",
        nom::error::ErrorKind::Eof,
    )))
}

pub(super) fn parse_next_instr_with_dont(mut input: &str) -> IResult<&str, Mul> {
    while !input.is_empty() {
        let (input_mul, _) = take_until("mul")(input)?;
        let res: IResult<&str, &str> = take_until("don't()")(input);

        match res {
            Ok((input_dont, _)) if input_mul.len() >= input_dont.len() => {
                (input, _) = tag("mul")(input_mul)?;
                match parse_mul_instr(input) {
                    Ok((input, instr)) => return Ok((input, instr)),
                    Err(nom::Err::Error(kek)) if kek.code != nom::error::ErrorKind::Eof => continue,
                    Err(err) => return Err(err),
                }
            }
            Err(_) => {
                (input, _) = tag("mul")(input_mul)?;
                match parse_mul_instr(input) {
                    Ok((input, instr)) => return Ok((input, instr)),
                    Err(nom::Err::Error(kek)) if kek.code != nom::error::ErrorKind::Eof => continue,
                    Err(err) => return Err(err),
                }
            }
            Ok(_) => {
                (input, _) = take_until("do()")(input)?;
            }
        }
    }
    Err(nom::Err::Error(nom::error::Error::new(
        "",
        nom::error::ErrorKind::Eof,
    )))
}

pub(super) fn parse_instructions(input: &str) -> IResult<&str, Vec<Mul>> {
    many0(parse_next_instr)(input)
}

pub(super) fn parse_instructions_with_dont(input: &str) -> IResult<&str, Vec<Mul>> {
    many0(parse_next_instr_with_dont)(input)
}
