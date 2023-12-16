use itertools::Itertools;
use std::{collections::HashMap, ops::Range};

pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 5: If You Give A Seed A Fertilizer",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod parse;

#[derive(Debug)]
struct MapRange {
    /// Start of source category.
    src0: i64,
    /// Start of destination category.
    dst0: i64,
    /// Length of the range.
    len: i64,
}

impl MapRange {
    fn map_to_dst(&self, src: i64) -> Option<i64> {
        self.src_range_contains(src)
            .then_some(src + (self.dst0 - self.src0))
    }

    /// Maps a whole range using this range and returns three new ranges. A left
    /// unmapped range, a mapped range, and a right unmapped range. The returned
    /// ranges can be empty if the `src` range is not a superset of this range.
    /// You can use [`Range::is_empty`] to check if a returned rage is empty.  
    fn map_range_to_dst(&self, src: &Range<i64>) -> (Range<i64>, Range<i64>, Range<i64>) {
        let dst_start = src.start + (self.dst0 - self.src0);
        let dst_end = src.end + (self.dst0 - self.src0);
        (
            src.start..src.end.min(self.src0),
            dst_start.max(self.dst0)..dst_end.min(self.dst0 + self.len),
            src.start.max(self.src0 + self.len)..src.end,
        )
    }

    fn src_range_contains(&self, value: i64) -> bool {
        (self.src0..self.src0 + self.len).contains(&value)
    }
}

#[derive(Debug)]
struct MapRanges {
    ranges: Box<[MapRange]>,
}

impl MapRanges {
    /// Maps the `src` number if the number is in one of the map ranges. If no map
    /// range is found the original value of `src` is returned.
    fn map_to_dst(&self, src: i64) -> i64 {
        self.ranges
            .iter()
            .find_map(|v| v.map_to_dst(src))
            .unwrap_or(src)
    }

    /// Maps the `src` range based on all the map ranges. Overlapping regions between
    /// `src` and map ranges produce new ranges that have been mapped. Any unmapped regions
    /// are also returned.
    ///
    /// Returns an iterator with all the new ranges which must be iterated over.
    fn map_range_to_dst(&self, src: Range<i64>) -> impl Iterator<Item = Range<i64>> + '_ {
        let mut queue = vec![src];
        let mut queue_new = Vec::new();
        let mut mapped = Vec::new();

        // Unmapped ranges are tried again. There should never be an overlap
        // between mappings.
        for range in self.ranges.iter() {
            for item in queue.drain(..) {
                let (a, b, c) = range.map_range_to_dst(&item);
                if !a.is_empty() {
                    queue_new.push(a);
                }
                if !b.is_empty() {
                    mapped.push(b);
                }
                if !c.is_empty() {
                    queue_new.push(c);
                }
            }
            std::mem::swap(&mut queue, &mut queue_new);
        }

        queue.into_iter().chain(mapped)
    }
}

#[derive(Debug)]
struct Almanac<'a> {
    seeds: Box<[i64]>,
    maps: HashMap<(&'a str, &'a str), MapRanges>,
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let (_, almanac) = parse::parse_almanac(input.as_ref()).unwrap();

    let map_order = &[
        "seed",
        "soil",
        "fertilizer",
        "water",
        "light",
        "temperature",
        "humidity",
        "location",
    ];

    // Part 1
    let mut part1_seeds = almanac.seeds.clone();
    for (src_category, dst_category) in map_order.iter().tuple_windows() {
        for seed in part1_seeds.iter_mut() {
            *seed = almanac
                .maps
                .get(&(src_category, dst_category))
                .unwrap_or_else(|| panic!("should have mapping {src_category}->{dst_category}"))
                .map_to_dst(*seed);
        }
    }

    // Part 2
    let mut part2_seeds: Vec<_> = almanac
        .seeds
        .iter()
        .tuples()
        .map(|(&start, &len)| start..start + len)
        .collect();

    for (src_category, dst_category) in map_order.iter().tuple_windows() {
        part2_seeds = part2_seeds
            .into_iter()
            .flat_map(|v| {
                almanac
                    .maps
                    .get(&(src_category, dst_category))
                    .unwrap_or_else(|| panic!("should have mapping {src_category}->{dst_category}"))
                    .map_range_to_dst(v)
            })
            .collect();
    }

    (
        part1_seeds.iter().min().unwrap().to_string(),
        part2_seeds
            .iter()
            .map(|v| v.start)
            .min()
            .unwrap()
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        "35"
    );
    solution!(p1, p1_solution, "535088217");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        "46"
    );
    solution!(p2, p2_solution, "51399228");
}
