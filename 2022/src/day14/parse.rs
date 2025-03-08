use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::{
        complete::char,
        complete::{digit1, line_ending, space0},
    },
    combinator::map_res,
    multi::separated_list0,
    sequence::{delimited, separated_pair},
};

pub fn rock_paths_parser<T>(input: &str) -> IResult<&str, Vec<Vec<T>>>
where
    T: From<(i32, i32)>,
{
    separated_list0(line_ending, rock_path_parser)(input)
}

pub fn rock_path_parser<T>(input: &str) -> IResult<&str, Vec<T>>
where
    T: From<(i32, i32)>,
{
    separated_list0(
        delimited(space0, tag("->"), space0),
        separated_pair(
            map_res(digit1, str::parse),
            char(','),
            map_res(digit1, str::parse),
        )
        .map(|t| T::from(t)),
    )(input)
}
