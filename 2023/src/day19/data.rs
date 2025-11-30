use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action<'a> {
    Accept,
    Reject,
    Switch(&'a str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    CoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rule<'a> {
    Greater(Category, u32, Action<'a>),
    Less(Category, u32, Action<'a>),
    Default(Action<'a>),
}

#[derive(Debug, Clone)]
pub struct Workflow<'a> {
    pub name: &'a str,
    pub rules: Box<[Rule<'a>]>,
}

#[derive(Debug, Clone)]
pub struct Part {
    pub cool: u32,
    pub musical: u32,
    pub aerodynamic: u32,
    pub shiny: u32,
}

#[derive(Debug, Clone)]
pub struct PartRange {
    pub cool: Range<u32>,
    pub musical: Range<u32>,
    pub aerodynamic: Range<u32>,
    pub shiny: Range<u32>,
}

impl Rule<'_> {
    pub fn is_match(&self, part: &Part) -> Option<Action<'_>> {
        match *self {
            Rule::Greater(category, ref value, action) => part
                .category_value(category)
                .cmp(value)
                .is_ge()
                .then_some(action),
            Rule::Less(category, ref value, action) => part
                .category_value(category)
                .cmp(value)
                .is_le()
                .then_some(action),
            Rule::Default(action) => Some(action),
        }
    }

    /// Like [is_match] except it takes in a PartRange and applies the rule
    /// to the entire range of values instead. This returns the matched portion
    /// of the ranges first, then the part of the ranges that didn't match,
    /// and lastly the action to be taken.
    pub fn match_range(&self, mut part: PartRange) -> (PartRange, PartRange, Action<'_>) {
        let mut non_match = PartRange {
            cool: 1..1,
            musical: 1..1,
            aerodynamic: 1..1,
            shiny: 1..1,
        };
        match *self {
            Rule::Greater(category, value, action) => {
                non_match = part.clone();
                let range = part.category_value_mut(category);
                let non_match_range = non_match.category_value_mut(category);
                range.start = value + 1;
                non_match_range.end = value + 1;
                (part, non_match, action)
            }
            Rule::Less(category, value, action) => {
                non_match = part.clone();
                let range = part.category_value_mut(category);
                let non_match_range = non_match.category_value_mut(category);
                range.end = value;
                non_match_range.start = value;
                (part, non_match, action)
            }
            Rule::Default(action) => (part, non_match, action),
        }
    }
}

impl Workflow<'_> {
    pub fn process(&self, part: &Part) -> Action<'_> {
        self.rules
            .iter()
            .map(|rule| rule.is_match(part))
            .find(Option::is_some)
            .flatten()
            .unwrap_or_else(|| {
                panic!(
                    "part {:?} did not match any rule in workflow {}",
                    part, self.name
                )
            })
    }
}

impl Part {
    pub fn category_value(&self, category: Category) -> u32 {
        match category {
            Category::CoolLooking => self.cool,
            Category::Musical => self.musical,
            Category::Aerodynamic => self.aerodynamic,
            Category::Shiny => self.shiny,
        }
    }

    pub fn total_rating(&self) -> u32 {
        self.cool + self.musical + self.aerodynamic + self.shiny
    }
}

impl PartRange {
    pub fn new() -> Self {
        Self {
            cool: 1..4001,
            musical: 1..4001,
            aerodynamic: 1..4001,
            shiny: 1..4001,
        }
    }

    pub fn category_value_mut(&mut self, category: Category) -> &mut Range<u32> {
        match category {
            Category::CoolLooking => &mut self.cool,
            Category::Musical => &mut self.musical,
            Category::Aerodynamic => &mut self.aerodynamic,
            Category::Shiny => &mut self.shiny,
        }
    }

    pub fn total_combination(&self) -> usize {
        let x = self.cool.len();
        let m = self.musical.len();
        let a = self.aerodynamic.len();
        let s = self.shiny.len();
        x * m * a * s
    }
}
