use nom::{
    IResult,
    bytes::complete::tag_no_case,
    character::complete::{self, newline, space0, space1},
    error::Error,
    multi::{many0, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
};

use super::data::Cpu;

fn program(input: &str) -> IResult<&str, Vec<u8>> {
    preceded(
        tuple((
            tag_no_case("Program"),
            delimited(space0, complete::char(':'), space0),
        )),
        separated_list1(complete::char(','), complete::u8),
    )(input)
}

fn register(input: &str) -> IResult<&str, (char, u64)> {
    separated_pair(
        preceded(
            terminated(tag_no_case("Register"), space1),
            complete::one_of("AaBbCcPp"),
        ),
        delimited(space0, complete::char(':'), space0),
        complete::u64,
    )(input)
}

fn cpu(input: &str) -> IResult<&str, Cpu> {
    let (input, registers) = terminated(many0(terminated(register, newline)), newline)(input)?;
    let mut cpu = Cpu::default();

    for (c, value) in registers {
        match c {
            'a' | 'A' => cpu.a = value,
            'b' | 'B' => cpu.b = value,
            'c' | 'C' => cpu.c = value,
            'p' | 'P' => cpu.pc = value as usize,
            _ => unreachable!("no other cases should be returned by the parser"),
        }
    }

    Ok((input, cpu))
}

pub(super) fn parse_program(input: &str) -> Result<(Cpu, Vec<u8>), nom::Err<Error<&str>>> {
    tuple((cpu, program))(input).map(|(_, v)| v)
}
