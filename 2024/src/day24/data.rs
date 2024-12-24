#[derive(Debug, Clone)]
pub enum LogicGate<'a> {
    And(&'a str, &'a str, &'a str),
    Or(&'a str, &'a str, &'a str),
    Xor(&'a str, &'a str, &'a str),
}

impl LogicGate<'_> {
    pub fn left_input_node(&self) -> &str {
        match self {
            LogicGate::And(input, _, _) => input,
            LogicGate::Or(input, _, _) => input,
            LogicGate::Xor(input, _, _) => input,
        }
    }

    pub fn right_input_node(&self) -> &str {
        match self {
            LogicGate::And(_, input, _) => input,
            LogicGate::Or(_, input, _) => input,
            LogicGate::Xor(_, input, _) => input,
        }
    }

    pub fn output_node(&self) -> &str {
        match self {
            LogicGate::And(_, _, output) => output,
            LogicGate::Or(_, _, output) => output,
            LogicGate::Xor(_, _, output) => output,
        }
    }
}
