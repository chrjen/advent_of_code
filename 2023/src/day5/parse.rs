use std::collections::HashMap;

use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{self, alpha1, newline, space0, space1},
    combinator::{map, opt},
    multi::{fold_many0, many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
};

use super::{Almanac, MapRange, MapRanges};

pub(super) fn parse_seeds(input: &str) -> IResult<&str, Box<[i64]>> {
    let list = delimited(space0, separated_list1(space1, complete::i64), space0);
    map(preceded(tag("seeds:"), list), |v| v.into_boxed_slice())(input)
}

pub(super) fn parse_map_range(input: &str) -> IResult<&str, MapRange> {
    let (input, (dst0, src0, len)) = preceded(
        newline,
        tuple((
            delimited(space0, complete::i64, space0),
            delimited(space0, complete::i64, space0),
            delimited(space0, complete::i64, space0),
        )),
    )(input)?;

    Ok((input, MapRange { src0, dst0, len }))
}

#[allow(clippy::let_and_return)]
pub(super) fn parse_map_ranges(input: &str) -> IResult<&str, (&str, &str, MapRanges)> {
    let (input, (src_category, dst_category)) = terminated(
        separated_pair(alpha1, tag("-to-"), alpha1),
        tuple((space0, tag("map:"))),
    )(input)?;

    let parsed = map(many1(parse_map_range), |v| {
        (
            src_category,
            dst_category,
            MapRanges {
                ranges: v.into_boxed_slice(),
            },
        )
    })(input);

    parsed
}

pub(super) fn parse_almanac(input: &str) -> IResult<&str, Almanac<'_>> {
    let (input, seeds) = terminated(parse_seeds, tag("\n\n"))(input)?;

    let (input, maps) = fold_many0(
        delimited(opt(newline), parse_map_ranges, opt(newline)),
        HashMap::new,
        |mut acc: HashMap<_, _>, (src, dst, map)| {
            acc.insert((src, dst), map);
            acc
        },
    )(input)?;

    Ok((input, Almanac { seeds, maps }))
}
