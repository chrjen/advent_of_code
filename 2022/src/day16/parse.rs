// Valve XL has flow rate=0; tunnels lead to valves AA, FA

use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag_no_case,
    character::{
        self,
        complete::char,
        complete::{alpha1, line_ending, space0},
    },
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded},
};

pub type Valve<'a> = (&'a str, u32, Vec<&'a str>);

/// Parses multiple lines of valve specifications into a `Vec`.
pub fn valve_specs_parser(input: &str) -> IResult<&str, Vec<Valve<'_>>> {
    separated_list0(line_ending, valve_spec_parser)(input)
}

/// Parses a single line of valve specification.
pub fn valve_spec_parser(input: &str) -> IResult<&str, Valve<'_>> {
    let (input, name) = preceded(tag_no_case("valve "), alpha1)(input)?;
    let (input, flow_rate) =
        preceded(tag_no_case(" has flow rate="), character::complete::u32)(input)?;
    let (input, _) = char(';')(input)?;
    let (input, connections) = preceded(
        alt((
            tag_no_case(" tunnel leads to valve "),
            tag_no_case(" tunnels lead to valves "),
        )),
        separated_list1(delimited(space0, char(','), space0), alpha1),
    )(input)?;

    Ok((input, (name, flow_rate, connections)))
}
