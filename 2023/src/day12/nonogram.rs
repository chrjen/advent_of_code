use std::fmt::Display;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Fill,
    Cross,
    Empty,
}

#[derive(Debug, Clone)]
pub struct Row {
    pub tiles: Vec<Tile>,
    pub hints: Vec<u32>,
}

pub static mut COUNT: usize = 0;

impl Row {
    pub fn is_valid(&self) -> bool {
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
            });
        pattern
            .zip_longest(self.hints.iter().copied())
            .all(|v| match v {
                itertools::EitherOrBoth::Both(lhs, rhs) => lhs == rhs as usize,
                itertools::EitherOrBoth::Left(_) => false,
                itertools::EitherOrBoth::Right(_) => false,
            })
    }

    pub fn count_combinations(&self) -> u32 {
        unsafe {
            COUNT += 1;
        }
        if self.is_valid() {
            1
        } else if let Some(index) = self
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
