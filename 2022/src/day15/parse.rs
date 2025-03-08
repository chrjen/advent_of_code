use nom::{
    IResult,
    bytes::complete::tag_no_case,
    character::{
        self,
        complete::char,
        complete::{line_ending, one_of, space0},
    },
    multi::separated_list0,
    sequence::{delimited, separated_pair, tuple},
};

/// Parses multiple lines of sensor outputs into a `Vec`.
pub fn sensor_outputs_parser<T>(input: &str) -> IResult<&str, Vec<(T, T)>>
where
    T: From<(i32, i32)>,
{
    separated_list0(line_ending, sensor_output_parser)(input)
}

/// Parses a single line of sensor output.
pub fn sensor_output_parser<T>(input: &str) -> IResult<&str, (T, T)>
where
    T: From<(i32, i32)>,
{
    let (input, _) = tag_no_case("sensor at ")(input)?;
    let (input, sensor) = coordinates_parser(input).map(|(input, v)| (input, T::from(v)))?;
    let (input, _) = tuple((char(':'), space0, tag_no_case("closest beacon is at ")))(input)?;
    let (input, beacon) = coordinates_parser(input).map(|(input, v)| (input, T::from(v)))?;
    Ok((input, (sensor, beacon)))
}

/// Parses two key-value pairs separated by a comma representing coordinates.
/// Possible keys are `x` and `y`. The values are returned in as tuple `(x, y)`.
fn coordinates_parser(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, ((a, a_val), (b, b_val))) = separated_pair(
        coord_parser,
        delimited(space0, char(','), space0),
        coord_parser,
    )(input)?;
    if a == b {
        return Err(nom::Err::Error(nom::error::ParseError::from_error_kind(
            input,
            nom::error::ErrorKind::Fail,
        )));
    }

    if a == 'x' {
        Ok((input, (a_val, b_val)))
    } else {
        Ok((input, (b_val, a_val)))
    }
}

/// Parses a single key-value pair. Possible keys are `x` and `y`.
fn coord_parser(input: &str) -> IResult<&str, (char, i32)> {
    separated_pair(one_of("xy"), char('='), character::complete::i32)(input)
}
