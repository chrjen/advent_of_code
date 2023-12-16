use serde_json::Value;

pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 12: JSAbacusFramework.io",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

pub fn solve(input: &[u8]) -> (String, String) {
    let data = serde_json::from_slice::<Value>(input)
        .unwrap_or_else(|err| panic!("unable to parse json: {}", err));

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

    fn process_json2(value: &Value, total: &mut i64) -> bool {
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
                    process_json2(v, total);
                }
            }
            Value::Object(obj) => {
                let mut total_ = 0;
                for (_, v) in obj.iter() {
                    if !process_json2(v, &mut total_) {
                        return true;
                    }
                }
                *total += total_;
            }
        }
        true
    }

    let mut part1 = 0;
    process_json(&data, &mut part1);

    let mut part2 = 0;
    process_json2(&data, &mut part2);

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(p1, p1_example_1, r#"[1,2,3]"#, "6");
    example!(p1, p1_example_2, r#"{"a":2,"b":4}"#, "6");
    example!(p1, p1_example_3, r#"[[[3]]]"#, "3");
    example!(p1, p1_example_4, r#"{"a":{"b":4},"c":-1}"#, "3");
    example!(p1, p1_example_5, r#"{"a":[-1,1]}"#, "0");
    example!(p1, p1_example_6, r#"[-1,{"a":1}]"#, "0");
    example!(p1, p1_example_7, r#"[]"#, "0");
    example!(p1, p1_example_8, r#"{}"#, "0");
    solution!(p1, p1_solution, "156366");

    // Part 2
    example!(p2, p2_example_1, r#"[1,2,3]"#, "6");
    example!(p2, p2_example_2, r#"[1,{"c":"red","b":2},3]"#, "4");
    example!(p2, p2_example_3, r#"{"d":"red","e":[1,2,3,4],"f":5}"#, "0");
    example!(p2, p2_example_4, r#"[1,"red",5]"#, "6");
    solution!(p2, p2_solution, "96852");
}
