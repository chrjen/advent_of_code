use regex::Regex;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 15: Science for Hungry People",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

#[derive(Debug, Clone, Copy)]
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
    for i in 0..=100_i64 {
        for j in i..=100_i64 {
            for k in j..=100_i64 {
                let s0 = i;
                let s1 = j - i;
                let s2 = k - j;
                let s3 = 100 - k;

                let cookie = ingredients[0] * s0
                    + ingredients[1] * s1
                    + ingredients[2] * s2
                    + ingredients[3] * s3;
                let score = cookie.score();
                max_score = max_score.max(score);

                if cookie.calories == 500 {
                    max_score_calories = max_score_calories.max(score);
                }
            }
        }
    }

    (max_score.to_string(), max_score_calories.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::solution;

    // Part 1
    // example!(
    //     p1,
    //     p1_example_1,
    //     r"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
    // Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
    //     "62842880"
    // );
    solution!(p1, p1_solution, "13882464");

    // Part 2
    //     example!(
    //         p2,
    //         p2_example_1,
    //         r"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
    // Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
    //         "57600000"
    //     );
    solution!(p2, p2_solution, "11171160");
}
