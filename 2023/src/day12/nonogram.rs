use std::{cmp::Ordering, fmt::Display};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Fill,
    Cross,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Valid,
    Invalid,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Row {
    pub tiles: Vec<Tile>,
    pub hints: Vec<usize>,
}

pub static mut COUNT: usize = 0;

impl Row {
    pub fn is_valid(&self) -> Status {
        let pattern = self
            .tiles
            .iter()
            .copied()
            .dedup_with_count()
            .filter_map(|(count, tile)| {
                if tile == Tile::Fill {
                    Some(count)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if pattern.is_empty() {
            return Status::Unknown;
        }

        match pattern.len().cmp(&self.hints.len()) {
            Ordering::Less => {
                // println!("GOT LESS");
                for hint_window in self.hints.windows(pattern.len()) {
                    // println!("hint: {:?} vs. {:?}", hint_window, pattern);
                    match pattern.as_slice().cmp(hint_window) {
                        Ordering::Less => return Status::Unknown,
                        Ordering::Equal => return Status::Unknown,
                        Ordering::Greater => continue,
                    }
                }
                return Status::Invalid;
            }
            Ordering::Equal => {
                // println!("GOT EQUAL");
                return match pattern.cmp(&self.hints) {
                    Ordering::Less => Status::Unknown,
                    Ordering::Equal => Status::Valid,
                    Ordering::Greater => {
                        let total_fill: usize = self.hints.iter().sum();
                        let fill: usize = pattern.iter().sum();
                        if fill <= total_fill {
                            return Status::Unknown;
                        } else {
                            return Status::Invalid;
                        }
                    }
                };
            }
            Ordering::Greater => {
                // println!("GOT GREATER");
                let total_fill: usize = self.hints.iter().sum();
                let fill: usize = pattern.iter().sum();
                if fill <= total_fill {
                    return Status::Unknown;
                } else {
                    return Status::Invalid;
                }
            }
        }

        // for pair in pattern.zip_longest(self.hints.iter().copied()) {
        //     match dbg!(pair) {
        //         itertools::EitherOrBoth::Both(pattern, hint) => match pattern.cmp(&(hint as usize))
        //         {
        //             std::cmp::Ordering::Less => return Status::Unknown,
        //             std::cmp::Ordering::Equal => continue,
        //             std::cmp::Ordering::Greater => return Status::Invalid,
        //         },
        //         itertools::EitherOrBoth::Left(_) => return Status::Invalid,
        //         itertools::EitherOrBoth::Right(_) => return Status::Unknown,
        //     }
        // }
    }

    pub fn unfold(&mut self, times: usize) {
        self.tiles = self.tiles.repeat(times);
    }

    pub fn count_combinations(&self) -> usize {
        unsafe {
            COUNT += 1;
        }

        println!("\x1b[36m{self}\x1b[0m");

        // match dbg!(self.is_valid()) {
        match self.is_valid() {
            Status::Valid => 1,
            Status::Invalid => 0,
            Status::Unknown => {
                if let Some(index) = self
                    .tiles
                    .iter()
                    .copied()
                    .position(|tile| tile == Tile::Empty)
                {
                    let mut with_fill = self.clone();
                    let mut with_cross = self.clone();

                    with_fill.tiles[index] = Tile::Fill;
                    with_cross.tiles[index] = Tile::Cross;

                    with_fill.count_combinations() + with_cross.count_combinations()
                } else {
                    0
                }
            }
        }
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for tile in &self.tiles {
            match tile {
                Tile::Fill => write!(f, "#")?,
                Tile::Cross => write!(f, ".")?,
                Tile::Empty => write!(f, "?")?,
            }
        }

        write!(f, " {:?}", self.hints)?;

        Ok(())
    }
}
