use std::ops::Range;

use regex::Regex;

pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 15: Science for Hungry People",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

#[derive(Debug, Default, Clone, Copy)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavour: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    fn score(self) -> i64 {
        self.capacity.max(0) * self.durability.max(0) * self.flavour.max(0) * self.texture.max(0)
    }
}

impl std::ops::Add<Self> for Ingredient {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.capacity += rhs.capacity;
        self.durability += rhs.durability;
        self.flavour += rhs.flavour;
        self.texture += rhs.texture;
        self.calories += rhs.calories;
        self
    }
}

impl std::ops::Mul<i64> for Ingredient {
    type Output = Self;

    fn mul(mut self, rhs: i64) -> Self::Output {
        self.capacity *= rhs;
        self.durability *= rhs;
        self.flavour *= rhs;
        self.texture *= rhs;
        self.calories *= rhs;
        self
    }
}

/// SubIntervalGenerator generates all possible sub-intervals of a given
/// interval.
///
/// Example:
/// Given an interval \[0, 2\] divided into two sub-intervals would give your the
/// following intervals, given as the length of each interval: \[0, 3\], \[1, 2\],
/// \[2, 1\] and \[3, 0\].
///
/// Internally this is done by going through all possible cuts of the interval.
/// With N sub-intervals N-1 cuts are needed. Each cut is an index between two
/// elements in the interval.
///
/// Used to generate all possible ways e.g. 100 teaspoons can be divided across
/// N number of ingredients.
struct SubIntervalGenerator {
    interval: Range<i64>,
    cuts: Vec<i64>,
    sub_intervals: Vec<i64>,
}

impl SubIntervalGenerator {
    fn new(interval: Range<i64>, sub_intervals: usize) -> Self {
        let mut v = Vec::with_capacity(sub_intervals);
        let mut w = Vec::with_capacity(sub_intervals - 1);
        v.resize(sub_intervals, interval.start);
        w.resize(sub_intervals - 1, interval.start);
        SubIntervalGenerator {
            interval,
            cuts: w,
            sub_intervals: v,
        }
    }

    fn next(&mut self) -> Option<&[i64]> {
        /// Increments each cut, moving it to the right. If the cut ends past the interval's upper
        /// bound then the previous cut get incremented instead and current cut gets set to the
        /// previous cut.
        ///
        /// Example of all cuts with interval \[0, 1\] and to sub-intervals in order:
        /// \[0, 0\]
        /// \[0, 1\]
        /// \[0, 2\]
        /// \[1, 1\]
        /// \[1, 2\]
        /// \[2, 2\]
        fn update(g: &mut SubIntervalGenerator, idx: usize) {
            g.cuts[idx] += 1;
            if g.cuts[idx] > g.interval.end && idx != 0 {
                update(g, idx - 1);
                g.cuts[idx] = g.cuts[idx - 1];
            }
        }

        if self.cuts.iter().any(|x| *x == self.interval.end + 1) {
            return None;
        }

        // Calculate the length of each sub-interval.
        let mut prev = self.interval.start;
        for (i, c) in self.cuts.iter().enumerate() {
            let int = *c - prev;
            prev = *c;
            self.sub_intervals[i] = int;
        }
        *self.sub_intervals.last_mut().unwrap() = self.interval.end - prev;

        // Increment all the cuts.
        update(self, self.cuts.len() - 1);

        Some(&self.sub_intervals)
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let reg = Regex::new(
        r"\w+: capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)",
    )
    .unwrap();

    let mut ingredients: Vec<Ingredient> = Vec::new();

    for cap in reg.captures_iter(&input) {
        ingredients.push(Ingredient {
            capacity: cap[1].parse().unwrap(),
            durability: cap[2].parse().unwrap(),
            flavour: cap[3].parse().unwrap(),
            texture: cap[4].parse().unwrap(),
            calories: cap[5].parse().unwrap(),
        });
    }

    let mut max_score = 0;
    let mut max_score_calories = 0;

    let mut k = SubIntervalGenerator::new(0..100, ingredients.len());
    while let Some(s) = k.next() {
        let mut cookie: Ingredient = Default::default();

        for (i, teaspoons) in s.iter().enumerate() {
            cookie = cookie + ingredients[i] * *teaspoons;
        }

        let score = cookie.score();
        max_score = max_score.max(score);

        if cookie.calories == 500 {
            max_score_calories = max_score_calories.max(score);
        }
    }

    (max_score.to_string(), max_score_calories.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        r"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
    Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
        "62842880"
    );
    solution!(p1, p1_solution, "13882464");

    // Part 2
    example!(
        p2,
        p2_example_1,
        r"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
    Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
        "57600000"
    );
    solution!(p2, p2_solution, "11171160");
}
