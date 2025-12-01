#![allow(unused)]
use serde_json::Value;

use common_v2::prelude::*;

pub struct Solution;

pub struct State {
    data: Value,
}

impl Solver for Solution {
    fn title() -> &'static str {
        return "Day X: Snowy Mountain Monkeys";
    }

    fn input() -> &'static [u8] {
        static INPUT: &'static [u8] = std::include_bytes!("input");
        return &INPUT;
    }

    fn initial(input: &[u8]) -> Box<dyn PartSolver + 'static> {
        let data = serde_json::from_slice::<Value>(input)
            .unwrap_or_else(|err| panic!("unable to parse json: {}", err));

        Box::new(State { data })
    }
}

impl PartSolver for State {
    fn part1(&self) -> Output {
        let mut total: i64 = 0;

        fn process_json(value: &Value, total: &mut i64) {
            match value {
                Value::Null => {}
                Value::Bool(_) => {}
                Value::Number(n) => *total += n.as_i64().unwrap_or(0),
                Value::String(_) => {}
                Value::Array(arr) => {
                    for v in arr.iter() {
                        process_json(v, total);
                    }
                }
                Value::Object(obj) => {
                    for (_, v) in obj.iter() {
                        process_json(v, total);
                    }
                }
            }
        }

        process_json(&self.data, &mut total);

        total.into()
    }

    fn part2(&self) -> Output {
        let mut total: i64 = 0;

        fn process_json(value: &Value, total: &mut i64) -> bool {
            match value {
                Value::Null => {}
                Value::Bool(_) => {}
                Value::Number(n) => *total += n.as_i64().unwrap_or(0),
                Value::String(s) => {
                    if s == "red" {
                        return false;
                    }
                }
                Value::Array(arr) => {
                    for v in arr.iter() {
                        process_json(v, total);
                    }
                }
                Value::Object(obj) => {
                    let mut total_ = 0;
                    for (_, v) in obj.iter() {
                        if !process_json(v, &mut total_) {
                            return true;
                        }
                    }
                    *total += total_;
                }
            }
            true
        }

        process_json(&self.data, &mut total);

        total.into()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use common_v2::{example, solution};

//     #[test]
//     fn p1_example_1() {
//         println!("input: {}", "1abc2");
//         let result = Solution::initial(str::as_bytes("1abc2")).part1();
//         assert_eq!(result, Output::from(142));
//     }

//     example!(p1, p1_example_2, "1abc3", 300);
//     solution!(p1, p1_solution, 200);

//     #[test]
//     fn p2_solution() {
//         let result = Solution::initial(Solution::input()).part1();
//         assert_eq!(result, Output::from("5227"));
//     }

//     example!(p2, p2_example_1, "1abc3", "200");
// }
