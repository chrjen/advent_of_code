use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
pub enum Monkey<'a> {
    Literal(Option<&'a str>, super::Number),
    Add(Option<&'a str>, &'a str, &'a str),
    Sub(Option<&'a str>, &'a str, &'a str),
    Mul(Option<&'a str>, &'a str, &'a str),
    Div(Option<&'a str>, &'a str, &'a str),
    Eq(Option<&'a str>, &'a str, Option<&'a str>),
}

impl<'a> Monkey<'a> {
    pub fn eval(&self, context: &HashMap<&str, Monkey<'a>>) -> Option<super::Number> {
        match *self {
            Monkey::Literal(_, x) => Some(x),
            Monkey::Add(_, lhs, rhs) => {
                let lhs = context.get(lhs)?.eval(context)?;
                let rhs = context.get(rhs)?.eval(context)?;
                Some(lhs + rhs)
            }
            Monkey::Sub(_, lhs, rhs) => {
                let lhs = context.get(lhs)?.eval(context)?;
                let rhs = context.get(rhs)?.eval(context)?;
                Some(lhs - rhs)
            }
            Monkey::Mul(_, lhs, rhs) => {
                let lhs = context.get(lhs)?.eval(context)?;
                let rhs = context.get(rhs)?.eval(context)?;
                Some(lhs * rhs)
            }
            Monkey::Div(_, lhs, rhs) => {
                let lhs = context.get(lhs)?.eval(context)?;
                let rhs = context.get(rhs)?.eval(context)?;
                Some(lhs / rhs)
            }
            Monkey::Eq(_, lhs, rhs) => {
                if let Some(rhs) = rhs {
                    let lhs = context.get(lhs)?.eval(context)?;
                    let rhs = context.get(rhs)?.eval(context)?;
                    if lhs == rhs {
                        Some(super::Number::from(1))
                    } else {
                        Some(super::Number::from(0))
                    }
                } else {
                    context.get(lhs)?.eval(context)
                }
            }
        }
    }

    pub fn invert(&mut self, child: &'a str) {
        match self {
            Monkey::Literal(_, _) => panic!("tried to invert a literal value"),
            Monkey::Add(Some(p), lhs, rhs) => {
                if child == *lhs {
                    *self = Monkey::Sub(Some(lhs), p, rhs);
                } else if child == *rhs {
                    *self = Monkey::Sub(Some(rhs), p, lhs);
                } else {
                    panic!("child is not found")
                }
            }
            Monkey::Sub(Some(p), lhs, rhs) => {
                if child == *lhs {
                    *self = Monkey::Add(Some(lhs), p, rhs);
                } else if child == *rhs {
                    *self = Monkey::Sub(Some(rhs), lhs, p);
                } else {
                    panic!("child is not found")
                }
            }
            Monkey::Mul(Some(p), lhs, rhs) => {
                if child == *lhs {
                    *self = Monkey::Div(Some(lhs), p, rhs);
                } else if child == *rhs {
                    *self = Monkey::Div(Some(rhs), p, lhs);
                } else {
                    panic!("child is not found")
                }
            }
            Monkey::Div(Some(p), lhs, rhs) => {
                if child == *lhs {
                    *self = Monkey::Mul(Some(lhs), p, rhs);
                } else if child == *rhs {
                    *self = Monkey::Div(Some(rhs), lhs, p);
                } else {
                    panic!("child is not found")
                }
            }
            Monkey::Eq(_, lhs, rhs) => {
                if child == *lhs {
                    *self = Monkey::Eq(Some(lhs), rhs.unwrap(), None);
                } else if child == rhs.unwrap() {
                    *self = Monkey::Eq(Some(rhs.unwrap()), lhs, None);
                } else {
                    panic!("child is not found")
                }
            }
            Monkey::Add(None, _, _)
            | Monkey::Sub(None, _, _)
            | Monkey::Mul(None, _, _)
            | Monkey::Div(None, _, _) => panic!("no parent, needed to be able to invert."),
        }
    }

    #[allow(unused)]
    pub fn lhs(&self) -> Option<&'a str> {
        match self {
            Monkey::Literal(_, _) => None,
            Monkey::Add(_, lhs, _) => Some(lhs),
            Monkey::Sub(_, lhs, _) => Some(lhs),
            Monkey::Mul(_, lhs, _) => Some(lhs),
            Monkey::Div(_, lhs, _) => Some(lhs),
            Monkey::Eq(_, lhs, _) => Some(lhs),
        }
    }

    pub fn lhs_mut(&mut self) -> Option<&mut &'a str> {
        match self {
            Monkey::Literal(_, _) => None,
            Monkey::Add(_, ref mut lhs, _) => Some(lhs),
            Monkey::Sub(_, ref mut lhs, _) => Some(lhs),
            Monkey::Mul(_, ref mut lhs, _) => Some(lhs),
            Monkey::Div(_, ref mut lhs, _) => Some(lhs),
            Monkey::Eq(_, ref mut lhs, _) => Some(lhs),
        }
    }

    pub fn parent(&self) -> &Option<&'a str> {
        match self {
            Monkey::Literal(p, _) => p,
            Monkey::Add(p, _, _) => p,
            Monkey::Sub(p, _, _) => p,
            Monkey::Mul(p, _, _) => p,
            Monkey::Div(p, _, _) => p,
            Monkey::Eq(p, _, _) => p,
        }
    }

    pub fn parent_mut(&mut self) -> &mut Option<&'a str> {
        match self {
            Monkey::Literal(ref mut p, _) => p,
            Monkey::Add(ref mut p, _, _) => p,
            Monkey::Sub(ref mut p, _, _) => p,
            Monkey::Mul(ref mut p, _, _) => p,
            Monkey::Div(ref mut p, _, _) => p,
            Monkey::Eq(ref mut p, _, _) => p,
        }
    }

    #[allow(unused)]
    pub fn rhs(&self) -> Option<&'a str> {
        match self {
            Monkey::Literal(_, _) => None,
            Monkey::Add(_, _, rhs) => Some(rhs),
            Monkey::Sub(_, _, rhs) => Some(rhs),
            Monkey::Mul(_, _, rhs) => Some(rhs),
            Monkey::Div(_, _, rhs) => Some(rhs),
            Monkey::Eq(_, _, rhs) => *rhs,
        }
    }

    pub fn rhs_mut(&mut self) -> Option<&mut &'a str> {
        match self {
            Monkey::Literal(_, _) => None,
            Monkey::Add(_, _, ref mut rhs) => Some(rhs),
            Monkey::Sub(_, _, ref mut rhs) => Some(rhs),
            Monkey::Mul(_, _, ref mut rhs) => Some(rhs),
            Monkey::Div(_, _, ref mut rhs) => Some(rhs),
            Monkey::Eq(_, _, rhs) => rhs.as_mut(),
        }
    }

    pub fn update_parents(root: &'a str, context: &mut HashMap<&'a str, Monkey<'a>>) {
        let mut node = context[root].clone();

        if let Some(lhs) = node.lhs_mut() {
            Self::update_parents(lhs, context);
            let lhs = context.get_mut(lhs).unwrap();
            *lhs.parent_mut() = Some(root);
        }

        if let Some(rhs) = node.rhs_mut() {
            Self::update_parents(rhs, context);
            let rhs = context.get_mut(rhs).unwrap();
            *rhs.parent_mut() = Some(root);
        }
    }

    pub fn solve_for_unknown(
        unknown: &'a str,
        context: &HashMap<&'a str, Monkey<'a>>,
    ) -> Option<super::Number> {
        let mut context = context.clone();

        let Some(new_root) = *context[unknown].parent() else {
            return context[unknown].eval(&context);
        };

        let mut child = unknown;
        let mut current = new_root;
        loop {
            if let Some(parent) = *context[current].parent() {
                context.get_mut(current)?.invert(child);
                child = current;
                current = parent;
            } else {
                context.get_mut(current)?.invert(child);
                break;
            }
        }

        context[new_root].eval(&context)
    }

    #[allow(unused)]
    pub fn print_tree(root: &'a str, context: &HashMap<&'a str, Monkey<'a>>) {
        fn print_tree_<'a>(
            indent: &mut Vec<bool>,
            current: &'a str,
            context: &HashMap<&'a str, Monkey<'a>>,
        ) {
            const SPACES: usize = 2;

            for (j, &continuing) in indent.iter().enumerate() {
                let last = j == indent.len() - 1;

                if !last {
                    for i in 0..SPACES {
                        if i == 0 && continuing {
                            print!("│");
                        } else {
                            print!(" ");
                        }
                    }
                } else {
                    for i in 0..SPACES {
                        if i == 0 && continuing {
                            print!("├");
                        } else if i == 0 && !continuing {
                            print!("└");
                        } else {
                            print!("─");
                        }
                    }
                }
            }

            let mut print_op = |symbol, lhs, rhs| {
                println!("{current}: ({symbol})");
                indent.push(true);
                print_tree_(indent, lhs, context);
                indent.pop();
                indent.push(false);
                print_tree_(indent, rhs, context);
                indent.pop();
            };

            match context[current] {
                Monkey::Literal(_, x) => println!("{current}: {x}"),
                Monkey::Add(_, lhs, rhs) => print_op('+', lhs, rhs),
                Monkey::Sub(_, lhs, rhs) => print_op('-', lhs, rhs),
                Monkey::Mul(_, lhs, rhs) => print_op('*', lhs, rhs),
                Monkey::Div(_, lhs, rhs) => print_op('/', lhs, rhs),
                Monkey::Eq(_, lhs, rhs) => {
                    if let Some(rhs) = rhs {
                        print_op('=', lhs, rhs);
                    } else {
                        println!("{current}: (=)");
                        indent.push(false);
                        print_tree_(indent, lhs, context);
                        indent.pop();
                    }
                }
            }
        }

        print_tree_(&mut Vec::new(), root, context)
    }
}

impl TryFrom<&str> for Monkey<'_> {
    type Error = <super::Number as FromStr>::Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Monkey::Literal(None, value.parse()?))
    }
}

impl<'a> From<(&'a str, char, &'a str)> for Monkey<'a> {
    fn from(value: (&'a str, char, &'a str)) -> Self {
        let (a, op, b) = value;
        match op {
            '+' => Monkey::Add(None, a, b),
            '-' => Monkey::Sub(None, a, b),
            '*' => Monkey::Mul(None, a, b),
            '/' => Monkey::Div(None, a, b),
            _ => unreachable!(),
        }
    }
}
