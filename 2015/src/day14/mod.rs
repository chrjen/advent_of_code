use regex::Regex;

pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 14: Reindeer Olympics",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

#[derive(Debug)]
struct Reindeer {
    _name: String,
    speed: i32,
    duration: i32,
    rest: i32,
    dist: i32,
    cool_down: i32,
    is_resting: bool,
    score: i32,
}

impl Reindeer {
    fn new(name: String, speed: i32, duration: i32, rest: i32) -> Reindeer {
        Reindeer {
            _name: name,
            speed,
            duration,
            rest,
            dist: 0,
            cool_down: duration,
            is_resting: false,
            score: 0,
        }
    }

    fn get_distance(&self) -> i32 {
        self.dist
    }

    fn tick(&mut self) {
        if !self.is_resting {
            self.dist += self.speed;
        }

        self.cool_down -= 1;
        if self.cool_down <= 0 {
            self.is_resting = !self.is_resting;
            if self.is_resting {
                self.cool_down = self.rest;
            } else {
                self.cool_down = self.duration;
            }
        }
    }

    fn score(&mut self) {
        self.score += 1;
    }

    fn get_score(&self) -> i32 {
        self.score
    }
}

fn solve_with_seconds(input: &[u8], seconds: i32) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let reg = Regex::new(
        r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
    )
    .unwrap();

    let mut reindeers: Vec<Reindeer> = Vec::new();

    for cap in reg.captures_iter(&input) {
        let (name, speed, duration, rest) = (
            &cap[1],
            cap[2].parse::<i32>().unwrap(),
            cap[3].parse::<i32>().unwrap(),
            cap[4].parse::<i32>().unwrap(),
        );

        reindeers.push(Reindeer::new(name.to_string(), speed, duration, rest));
    }

    for _ in 0..seconds {
        let mut cur_max_dist = 0;

        for reindeer in reindeers.iter_mut() {
            reindeer.tick();
            cur_max_dist = cur_max_dist.max(reindeer.get_distance());
        }

        for reindeer in reindeers
            .iter_mut()
            .filter(|x| x.get_distance() == cur_max_dist)
        {
            reindeer.score();
        }
    }

    let (max_dist, max_score) = reindeers.iter().fold((0, 0), |mut max, reindeer| {
        max.0 = max.0.max(reindeer.get_distance());
        max.1 = max.1.max(reindeer.get_score());
        max
    });

    (max_dist.to_string(), max_score.to_string())
}

pub fn solve(input: &[u8]) -> (String, String) {
    solve_with_seconds(input, 2503)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::solution;

    // Part 1
    #[test]
    fn p1_example_1() {
        let (result, _) = solve_with_seconds(
            r"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."
                .as_bytes(),
            1000,
        );
        assert_eq!(result, "1120");
    }

    solution!(p1, p1_solution, "2655");

    // Part 2
    #[test]
    fn p2_example_1() {
        let (_, result) = solve_with_seconds(
            r"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."
                .as_bytes(),
            1000,
        );
        assert_eq!(result, "689");
    }
    solution!(p2, p2_solution, "1059");
}
