use std::collections::HashMap;

use crate::day7::tokens::BinaryOp;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 7: Some Assembly Required",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

mod tokens {
    use std::{ops::Range, str::Chars};

    #[derive(Debug, Clone, Copy)]
    pub enum BinaryOp {
        And,
        Or,
        LShift,
        RShift,
    }

    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    pub enum TokenType {
        Invalid(String),
        Ident(String),
        Signal(u16),
        Connect,
        BinOp(BinaryOp),
        Not,
    }

    #[derive(Debug, Clone)]
    pub struct Token {
        pub row: usize,
        pub col: usize,
        pub offset: Range<usize>,
        pub data: TokenType,
    }

    pub struct Tokeniser<'a> {
        source: Chars<'a>,
        cur: Option<char>,
        cur_row: usize,
        cur_col: usize,
        cur_offset: usize,
    }

    impl<'a> Tokeniser<'a> {
        pub fn new(mut source: Chars) -> Tokeniser {
            let c = source.next();
            Tokeniser {
                source,
                cur: c,
                cur_row: 1,
                cur_col: 1,
                cur_offset: 0,
            }
        }

        fn produce_token<F>(&mut self, f: F) -> Token
        where
            F: FnOnce(&mut Self) -> TokenType,
        {
            let row = self.cur_row;
            let col = self.cur_col;
            let start_offset = self.cur_offset;

            let data = f(self);

            Token {
                row,
                col,
                offset: start_offset..self.cur_offset,
                data,
            }
        }

        fn advance(&mut self) {
            self.cur = self.source.next();
            self.cur_offset += 1;
            self.cur_col += 1;
        }

        fn new_line(&mut self) {
            self.cur_row += 1;
            self.cur_col = 1;
        }

        fn read_whitespace(&mut self) {
            while let Some(c) = self.cur {
                match c {
                    '\n' => {
                        self.advance();
                        self.new_line();
                    }
                    ' ' | '\t' => self.advance(),
                    _ => return,
                }
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

        fn read_keyword(&mut self) -> TokenType {
            let mut buf = String::new();

            while let Some(c) = self.cur {
                if !c.is_ascii_uppercase() {
                    break;
                }
                buf.push(c);
                self.advance();
            }

            match buf.as_str() {
                "AND" => TokenType::BinOp(BinaryOp::And),
                "OR" => TokenType::BinOp(BinaryOp::Or),
                "NOT" => TokenType::Not,
                "LSHIFT" => TokenType::BinOp(BinaryOp::LShift),
                "RSHIFT" => TokenType::BinOp(BinaryOp::RShift),
                _ => TokenType::Invalid(buf),
            }
        }
    }

    impl<'a> Iterator for Tokeniser<'a> {
        type Item = Token;

        fn next(&mut self) -> Option<Self::Item> {
            self.cur?;

            self.read_whitespace();

            let token = match self.cur {
                Some('-') => self.produce_token(|tokeniser| {
                    tokeniser.advance();
                    if matches!(tokeniser.cur, Some('>')) {
                        tokeniser.advance();
                        TokenType::Connect
                    } else {
                        TokenType::Invalid("-".to_owned())
                    }
                }),
                Some(c) if c.is_ascii_digit() => {
                    self.produce_token(|tokeniser| TokenType::Signal(tokeniser.read_digit()))
                }
                Some(c) if c.is_ascii_lowercase() => {
                    self.produce_token(|tokeniser| TokenType::Ident(tokeniser.read_ident()))
                }
                Some(c) if c.is_ascii_uppercase() => {
                    self.produce_token(|tokeniser| tokeniser.read_keyword())
                }
                Some(c) => self.produce_token(|tokeniser| {
                    tokeniser.advance();
                    TokenType::Invalid(c.to_string())
                }),
                x => panic!("failed to tokenise, got {:?}", x),
            };

            Some(token)
        }
    }
}

/// Language grammar:
///
/// statement: expr CONNECT IDENT
///
/// expr: value
///     | NOT value
///     | value binary_op value
///
/// value: IDENT | SIGNAL
///
/// binary_op: AND | OR | LSHIFT | RSHIFT
mod parser {
    use std::{error, fmt::Display};

    use super::tokens::*;

    pub type Ast = Vec<Statement>;

    #[derive(Debug, Clone)]
    pub struct Statement(pub Expr, pub String);

    #[derive(Debug, Clone)]
    pub enum Expr {
        Value(Value),
        Not(Value),
        Binary(Value, BinaryOp, Value),
    }

    #[derive(Debug, Clone)]
    pub enum Value {
        Ident(String),
        Signal(u16),
    }

    #[derive(Debug, Clone)]
    pub struct ParseError {
        pub msg: String,
        pub err: ParseErrorType,
    }

    #[derive(Debug, Clone)]
    pub enum ParseErrorType {
        IncorrectToken(Token),
        EndOfStream,
    }

    impl ParseError {
        pub fn new(msg: String, err: ParseErrorType) -> ParseError {
            ParseError { msg, err }
        }
    }

    impl Display for ParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.msg)
        }
    }

    impl error::Error for ParseError {}

    pub struct Parser<'a> {
        tokeniser: Tokeniser<'a>,
        cur_token: Option<Token>,
        next_token: Option<Token>,
    }

    impl<'a> Parser<'a> {
        pub fn new(mut tokeniser: Tokeniser<'a>) -> Parser {
            let cur_token = tokeniser.next();
            let next_token = tokeniser.next();
            Parser {
                tokeniser,
                cur_token,
                next_token,
            }
        }

        pub fn parse(mut self) -> Result<Ast, ParseError> {
            let mut statements: Ast = Vec::new();
            while self.cur_token.is_some() {
                statements.push(self.parse_statement()?);
            }
            Ok(statements)
        }

        fn advance(&mut self) {
            self.cur_token.clone_from(&self.next_token);
            self.next_token = self.tokeniser.next();
        }

        fn parse_statement(&mut self) -> Result<Statement, ParseError> {
            let expr = self.parse_expr()?;
            match &self.cur_token {
                Some(token) => match &token.data {
                    TokenType::Connect => { /* Do nothing. */ }
                    _ => {
                        return Err(ParseError::new(
                            "unexpected token when parsing statement".to_owned(),
                            ParseErrorType::IncorrectToken(token.clone()),
                        ))
                    }
                },
                None => {
                    return Err(ParseError::new(
                        "unexpected end of stream while parsing statement".to_owned(),
                        ParseErrorType::EndOfStream,
                    ))
                }
            }
            self.advance();

            let right = match &self.cur_token {
                Some(token) => match &token.data {
                    TokenType::Ident(name) => Ok(name.clone()),
                    _ => Err(ParseError::new(
                        "unexpected token when parsing statement".to_owned(),
                        ParseErrorType::IncorrectToken(token.clone()),
                    )),
                },
                None => Err(ParseError::new(
                    "unexpected end of stream while parsing statement".to_owned(),
                    ParseErrorType::EndOfStream,
                )),
            }?;
            self.advance();

            Ok(Statement(expr, right))
        }

        fn parse_expr(&mut self) -> Result<Expr, ParseError> {
            match &self.cur_token {
                Some(token) => match &token.data {
                    TokenType::Not => {
                        self.advance();
                        let right = self.parse_value()?;
                        Ok(Expr::Not(right))
                    }
                    _ => match &self.next_token {
                        Some(next_token) => match next_token.data {
                            TokenType::BinOp(op) => {
                                let left = self.parse_value()?;
                                self.advance();
                                let right = self.parse_value()?;
                                Ok(Expr::Binary(left, op, right))
                            }
                            _ => Ok(Expr::Value(self.parse_value()?)),
                        },
                        None => Err(ParseError::new(
                            "unexpected end of stream while parsing expression".to_owned(),
                            ParseErrorType::EndOfStream,
                        )),
                    },
                },
                None => Err(ParseError::new(
                    "unexpected end of stream while parsing expression".to_owned(),
                    ParseErrorType::EndOfStream,
                )),
            }
        }

        fn parse_value(&mut self) -> Result<Value, ParseError> {
            let result = match &self.cur_token {
                Some(token) => match &token.data {
                    TokenType::Signal(value) => Ok(Value::Signal(*value)),
                    TokenType::Ident(name) => Ok(Value::Ident(name.clone())),
                    _ => Err(ParseError::new(
                        "unexpected token when parsing value".to_owned(),
                        ParseErrorType::IncorrectToken(token.clone()),
                    )),
                },
                None => Err(ParseError::new(
                    "unexpected end of stream while parsing expression".to_owned(),
                    ParseErrorType::EndOfStream,
                )),
            };
            self.advance();
            result
        }
    }
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let mut wires: HashMap<String, u16> = HashMap::new();

    let result = parser::Parser::new(tokens::Tokeniser::new(input.chars())).parse();
    let statements = match result {
        Ok(ast) => ast,
        Err(err) => match err.err {
            parser::ParseErrorType::IncorrectToken(tokens) => {
                println!("Error Line {} Col {}: {}", tokens.row, tokens.col, err.msg);
                for _ in 0..tokens.col - 1 {
                    print!(" ");
                }
                for _ in tokens.offset {
                    print!("v")
                }
                println!();
                let line = input.lines().nth(tokens.row - 1).unwrap();
                println!("{line}");

                return (0.to_string(), 0.to_string());
            }
            parser::ParseErrorType::EndOfStream => {
                println!("{}", err.msg);
                return (0.to_string(), 0.to_string());
            }
        },
    };

    let mut wire_statements: HashMap<String, parser::Statement> = HashMap::new();
    for statement in statements {
        let parser::Statement(_, name) = &statement;
        if wire_statements.insert(name.clone(), statement).is_some() {
            panic!("two different statements for the same wire");
        }
    }

    fn eval_statement(
        statement: &parser::Statement,
        statements: &HashMap<String, parser::Statement>,
        wires: &mut HashMap<String, u16>,
    ) {
        let parser::Statement(expr, ident) = statement;

        if wires.get(ident).is_some() {
            return;
        }

        let signal = {
            let mut get_signal = |value: &parser::Value| -> u16 {
                match value {
                    parser::Value::Ident(ident) => {
                        if let Some(signal) = wires.get(ident) {
                            return *signal;
                        }
                        let Some(s) = statements.get(ident) else {
                            panic!("no statements evaluates to {}", &ident)
                        };
                        eval_statement(s, statements, wires);
                        *wires.get(ident).unwrap()
                    }
                    parser::Value::Signal(signal) => *signal,
                }
            };
            match expr {
                parser::Expr::Value(v) => get_signal(v),
                parser::Expr::Not(v) => !get_signal(v),
                parser::Expr::Binary(left, op, right) => match op {
                    BinaryOp::And => {
                        let left = get_signal(left);
                        let right = get_signal(right);
                        left & right
                    }
                    BinaryOp::Or => {
                        let left = get_signal(left);
                        let right = get_signal(right);
                        left | right
                    }
                    BinaryOp::LShift => {
                        let left = get_signal(left);
                        let right = get_signal(right);
                        left << right
                    }
                    BinaryOp::RShift => {
                        let left = get_signal(left);
                        let right = get_signal(right);
                        left >> right
                    }
                },
            }
        };
        wires.insert(ident.clone(), signal);
    }

    // Part 1
    let s = wire_statements.get("a").unwrap();
    eval_statement(s, &wire_statements, &mut wires);
    let part1 = *wires.get("a").unwrap();

    // Part 2
    wires.clear();
    wires.insert("b".to_owned(), part1);
    eval_statement(s, &wire_statements, &mut wires);
    let part2 = *wires.get("a").unwrap();

    // // Print all wires
    // for (_, statement) in wire_statements.iter() {
    //     eval_statement(statement, &wire_statements, &mut wires);
    // }

    // let mut output = String::new();
    // let mut keys = wires.keys().collect::<Vec<_>>();
    // keys.sort_unstable();
    // let mut first = true;

    // for key in keys.into_iter() {
    //     if first {
    //         first = false;
    //     } else {
    //         output.push('\n');
    //     }

    //     let tmp = format!("{}: {}", key, wires.get(key).unwrap());
    //     output.push_str(&tmp);
    // }

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "i AND h -> a
123 -> x
456 -> b
x AND b -> d
d OR b -> e
x LSHIFT 2 -> f
f RSHIFT 2 -> g
NOT g -> h
NOT e -> i",
        "65028"
    );
    solution!(p1, p1_solution, "956");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "i AND h -> a
123 -> x
456 -> b
x AND b -> d
d OR b -> e
x LSHIFT 2 -> f
f RSHIFT 2 -> g
NOT g -> h
NOT e -> i",
        "384"
    );
    solution!(p2, p2_solution, "40149");
}
