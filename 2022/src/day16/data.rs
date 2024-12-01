use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node<'a> {
    pub name: &'a str,
    pub flow_rate: u32,
}

impl Debug for Node<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.name, self.flow_rate)
    }
}

impl PartialOrd for Node<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ord = self.flow_rate.cmp(&other.flow_rate);
        if ord.is_eq() {
            self.name.cmp(other.name)
        } else {
            ord
        }
    }
}
