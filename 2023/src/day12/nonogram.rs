use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Fill,
    Cross,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Row {
    pub tiles: Vec<Tile>,
    pub hints: Vec<usize>,
}

pub static mut COUNT: usize = 0;

impl Row {
    pub fn unfold(&mut self) {
        self.hints = self.hints.repeat(5);

        self.tiles.reserve(5 * self.tiles.len());
        let tiles = self.tiles.clone();

        for _ in 0..4 {
            self.tiles.push(Tile::Empty);
            for &tile in tiles.iter() {
                self.tiles.push(tile);
            }
        }
    }

    pub fn count_combinations(&self, cache: &mut HashMap<Row, usize>) -> usize {
        unsafe {
            COUNT += 1;
        }

        let mut sum = 0;
        let mut hints = self.hints.clone();

        if hints.is_empty() {
            if self.tiles.contains(&Tile::Fill) {
                return 0;
            } else {
                return 1;
            }
        }

        let size = hints.pop().unwrap();

        if self.tiles.is_empty() {
            return 0;
        }

        for i in 0..=self.tiles.len() {
            let mut tiles_it = self.tiles.iter().rev();

            if self.tiles.len().checked_sub(size + i).is_none() {
                break;
            }

            if tiles_it.by_ref().take(i).any(|tile| *tile == Tile::Fill) {
                break;
            }

            if tiles_it
                .by_ref()
                .take(size)
                .any(|tile| *tile == Tile::Cross)
            {
                continue;
            }

            if matches!(tiles_it.next(), Some(Tile::Fill)) {
                continue;
            }

            let row = Row {
                hints: hints.clone(),
                tiles: tiles_it.rev().copied().collect(),
            };

            if let Some(count) = cache.get(&row) {
                sum += count;
            } else {
                let count = row.count_combinations(cache);
                cache.insert(row, count);
                sum += count;
            }
        }

        sum
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
