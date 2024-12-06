use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Page<'a> {
    rules: &'a HashMap<u32, HashSet<u32>>,
    pub page_number: u32,
}

impl<'a> Page<'a> {
    pub fn new(rules: &'a HashMap<u32, HashSet<u32>>, page_number: u32) -> Self {
        Page { rules, page_number }
    }
}

impl Debug for Page<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Page {:?}", self.page_number)
    }
}

impl PartialOrd for Page<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Page<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_lt = self
            .rules
            .get(&self.page_number)
            .is_some_and(|set| set.contains(&other.page_number));
        let other_lt = self
            .rules
            .get(&other.page_number)
            .is_some_and(|set| set.contains(&self.page_number));

        match (self_lt, other_lt) {
            (false, false) => std::cmp::Ordering::Equal,
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            (true, true) => panic!("circular ordering, both pages ordered before the other"),
        }
    }
}
