use std::collections::HashMap;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 7: Some Assembly Required",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod tokens {
    use std::str::Chars;

    #[derive(Debug, Clone, Copy)]
    pub enum BinaryOp {
        And,
        Or,
        LShift,
        RShift,
    }

    #[derive(Debug, Clone)]
    pub enum Token {
        Invalid(String),
        Ident(String),
        Signal(u16),
        Connect,
        BinOp(BinaryOp),
        Not,
    }

    pub struct Tokeniser<'a> {
        source: Chars<'a>,
        cur: Option<char>,
    }

    impl<'a> Tokeniser<'a> {
        pub fn new(mut source: Chars) -> Tokeniser {
            let c = source.next();
            Tokeniser { source, cur: c }
        }

        fn advance(&mut self) {
            self.cur = self.source.next();
        }

        fn read_whitespace(&mut self) {
            while self.cur.is_some() && matches!(self.cur.unwrap(), ' ' | '\n' | '\t') {
                self.advance();
            }
        }

        fn read_ident(&mut self) -> String {
            let mut buf = String::new();

            while let Some(c) = self.cur {
                if !c.is_ascii_lowercase() {
                    break;
                }
                buf.push(c);
                self.advance();
            }

            buf
        }

        fn read_digit(&mut self) -> u16 {
            let mut buf = String::new();

            while let Some(c) = self.cur {
                if !c.is_ascii_digit() {
                    break;
                }
                buf.push(c);
                self.advance();
            }

            let result = buf.parse();
            match result {
                Ok(n) => n,
                Err(_) => panic!("failed to parse '{}' to u16", &buf),
            }
        }

        fn read_keyword(&mut self) -> Token {
            let mut buf = String::new();

            while let Some(c) = self.cur {
                if !c.is_ascii_uppercase() {
                    break;
                }
                buf.push(c);
                self.advance();
            }

            match buf.as_str() {
                "AND" => Token::BinOp(BinaryOp::And),
                "OR" => Token::BinOp(BinaryOp::Or),
                "NOT" => Token::Not,
                "LSHIFT" => Token::BinOp(BinaryOp::LShift),
                "RSHIFT" => Token::BinOp(BinaryOp::RShift),
                _ => Token::Invalid(buf),
            }
        }
    }

    impl<'a> Iterator for Tokeniser<'a> {
        type Item = Token;

        fn next(&mut self) -> Option<Self::Item> {
            self.cur?;

            self.read_whitespace();

            let token = match self.cur {
                Some('-') => {
                    self.advance();
                    if matches!(self.cur, Some('>')) {
                        self.advance();
                        Token::Connect
                    } else {
                        Token::Invalid("-".to_owned())
                    }
                }
                Some(c) if c.is_ascii_digit() => Token::Signal(self.read_digit()),
                Some(c) if c.is_ascii_lowercase() => Token::Ident(self.read_ident()),
                Some(c) if c.is_ascii_uppercase() => self.read_keyword(),
                Some(c) => Token::Invalid(c.to_string()),
                x => panic!("failed to tokenise, got {:?}", x),
            };

            Some(token)
        }
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let mut wires: HashMap<String, u16> = HashMap::new();

    // Sorry for the awful parser below.
    let mut tokeniser = tokens::Tokeniser::new(input.chars());
    let mut cur_token = tokeniser.next();
    while cur_token.is_some() {
        use tokens::BinaryOp::*;
        use tokens::Token::{self, *};

        fn get_signal(
            tokeniser: &mut tokens::Tokeniser,
            cur_token: &mut Option<Token>,
            wires: &HashMap<String, u16>,
        ) -> u16 {
            let result = match (*cur_token).clone() {
                Some(Ident(name)) => {
                    *cur_token = tokeniser.next();
                    if let Some(&value) = wires.get(name.as_str()) {
                        value
                    } else {
                        panic!("variable used before decleared: {}", name);
                    }
                }
                Some(Signal(value)) => {
                    *cur_token = tokeniser.next();
                    value
                }
                Some(Not) => {
                    *cur_token = tokeniser.next();
                    let right = get_signal(tokeniser, cur_token, wires);
                    !right
                }
                Some(x) => panic!("failed to parse instruction got token: {:?}", x),
                None => panic!("unexpected end of input"),
            };

            result
        }

        let left: u16 = get_signal(&mut tokeniser, &mut cur_token, &wires);

        let left = match cur_token {
            Some(BinOp(op)) => {
                cur_token = tokeniser.next();
                let right = get_signal(&mut tokeniser, &mut cur_token, &wires);
                match op {
                    And => left & right,
                    Or => left | right,
                    LShift => left << right,
                    RShift => left >> right,
                }
            }
            _ => left,
        };

        if !matches!(cur_token, Some(Connect)) {
            panic!("expected '->' operator, got {:?}", cur_token);
        }

        cur_token = tokeniser.next();
        if let Some(Ident(name)) = cur_token {
            if wires.insert(name.clone(), left).is_some() {
                panic!("double assigment: {}", name);
            }
        } else {
            panic!("expected identifier, got: {:?}", cur_token);
        }

        cur_token = tokeniser.next();
    }

    let mut output = String::new();
    let mut keys = wires.keys().collect::<Vec<_>>();
    keys.sort_unstable();
    let mut first = true;

    for key in keys.into_iter() {
        if first {
            first = false;
        } else {
            output.push('\n');
        }

        let tmp = format!("{}: {}", key, wires.get(key).unwrap());
        output.push_str(&tmp);
    }

    (output, 0.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i",
        "d: 72
e: 507
f: 492
g: 114
h: 65412
i: 65079
x: 123
y: 456"
    );
    // example!(p1, p1_example_3, "", "0");
    // example!(p1, p1_example_4, "", "0");
    // example!(p1, p1_example_5, "", "0");
    // solution!(p1, p1_solution, "100");

    // Part 2
    // example!(p2, p2_example_1, "", "0");
    // example!(p2, p2_example_2, "", "0");
    // example!(p2, p2_example_3, "", "0");
    // example!(p2, p2_example_4, "", "0");
    // example!(p2, p2_example_5, "", "0");
    // solution!(p2, p2_solution, "100");
}
