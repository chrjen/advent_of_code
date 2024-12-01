use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Hasher {
    pub value: u32,
}

impl Hasher {
    pub fn new() -> Self {
        Hasher { value: 0 }
    }

    pub fn hash(&mut self, c: char) {
        self.value += c as u32;
        self.value *= 17;
        self.value %= 256;
    }

    pub fn hash_str(s: &str) -> u32 {
        let mut hasher = Hasher::new();
        for c in s.chars() {
            match c {
                '\n' => {}
                c => hasher.hash(c),
            }
        }
        hasher.value
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Dash,
    Equal(u8),
}

#[derive(Debug, Clone, Copy)]
pub struct Step<'a> {
    pub label: &'a str,
    pub op: Operation,
}

impl<'a> Step<'a> {
    pub fn new(label: &'a str, op: Operation) -> Self {
        Step { label, op }
    }
}

impl Display for Step<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.op {
            Operation::Dash => write!(f, "{}-", self.label)?,
            Operation::Equal(focal_len) => write!(f, "{}={}", self.label, focal_len)?,
        }
        Ok(())
    }
}
