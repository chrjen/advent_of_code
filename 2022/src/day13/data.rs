use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Data {
    Integer(u32),
    List(Vec<Data>),
}

impl Data {
    pub fn to_list(&self) -> Self {
        match self {
            Data::Integer(_) => Data::List(vec![self.clone()]),
            Data::List(_) => self.clone(),
        }
    }
}

impl From<u32> for Data {
    fn from(x: u32) -> Self {
        Data::Integer(x)
    }
}

impl From<Vec<Data>> for Data {
    fn from(v: Vec<Data>) -> Self {
        Data::List(v)
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        match (self, other) {
            (Data::Integer(a), Data::Integer(b)) => a.cmp(b),
            (Data::Integer(_), Data::List(_)) => self.to_list().cmp(other),
            (Data::List(_), Data::Integer(_)) => self.cmp(&other.to_list()),
            (Data::List(a), Data::List(b)) => {
                let (mut a_it, mut b_it) = (a.iter(), b.iter());
                loop {
                    let (a, b) = (a_it.next(), b_it.next());
                    match (a, b) {
                        (None, None) => return Ordering::Equal,
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                        (Some(c), Some(d)) => match c.cmp(d) {
                            Ordering::Less => return Ordering::Less,
                            Ordering::Equal => continue,
                            Ordering::Greater => return Ordering::Greater,
                        },
                    }
                }
            }
        }
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Integer(x) => write!(f, "{}", x)?,
            Data::List(v) => {
                let mut it = v.iter();
                write!(f, "[")?;
                if let Some(first) = it.next() {
                    write!(f, "{}", first)?;
                    for data in it {
                        write!(f, ",{}", data)?;
                    }
                }
                write!(f, "]")?;
            }
        }
        Ok(())
    }
}

pub fn parse_packet(input: &[u8]) -> (Option<Data>, &[u8]) {
    for i in 0..input.len() {
        match input[i] {
            b'[' => {
                let (data, input) = parse_vec(&input[i + 1..]);
                return (Some(data), input);
            }
            _ => continue,
        }
    }
    (None, &[])
}

fn parse_vec(mut input: &[u8]) -> (Data, &[u8]) {
    use Data::*;

    let mut v = Vec::new();
    let mut it = input.iter().enumerate();
    while let Some((i, b)) = it.next() {
        match b {
            b'0'..=b'9' => {
                let data;
                (data, input) = parse_number(&input[i..]);
                v.push(Integer(data));
                if input[0] == b',' {
                    input = &input[1..];
                }
                it = input.iter().enumerate();
            }
            b'[' => {
                let data;
                (data, input) = parse_vec(&input[i + 1..]);
                v.push(data);
                it = input.iter().enumerate();
            }
            b']' | b'\n' => break,
            b',' => continue,
            _ => panic!("unknown character read while parsing vec '{}'", b),
        }
    }
    (List(v), &input[1..])
}

fn parse_number(input: &[u8]) -> (u32, &[u8]) {
    for i in 0..input.len() {
        if input[i].is_ascii_digit() {
            continue;
        }
        let number = std::str::from_utf8(&input[..i]).unwrap().parse().unwrap();
        return (number, &input[i..]);
    }
    panic!("failed to parser number, end of input")
}
