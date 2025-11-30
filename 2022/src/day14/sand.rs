use std::collections::HashMap;
use std::fmt::Display;

use crossterm::Command;
use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Loc {
    pub x: i32,
    pub y: i32,
}

impl Loc {
    pub fn offset(mut self, dx: i32, dy: i32) -> Self {
        self.x += dx;
        self.y += dy;
        self
    }
}

impl From<(i32, i32)> for Loc {
    fn from(t: (i32, i32)) -> Self {
        Loc { x: t.0, y: t.1 }
    }
}

pub enum Tile {
    Rock,
    Sand,
}

/// Represents a single sand simulation world.
pub struct World {
    source: Loc,
    tiles: HashMap<Loc, Tile>,
    floor: i32,
    count: usize,
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.source.y..=self.floor {
            for x in self.left()..=self.right() {
                let tile = if y == self.floor() {
                    Some(&Tile::Rock)
                } else {
                    self.tiles.get(&Loc::from((x, y)))
                };

                if let Some(tile) = tile {
                    match tile {
                        Tile::Rock => {
                            write!(f, "#")?;
                        }
                        Tile::Sand => {
                            write!(f, "o")?;
                        }
                    }
                } else if Loc::from((x, y)) == self.source() {
                    write!(f, "+")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl World {
    /// Returns a new sand simulation with the given initial tile layout.
    /// New grains of sand will originate from the source.
    pub fn new(source: Loc, floor: i32, tiles: HashMap<Loc, Tile>) -> Self {
        World {
            source,
            tiles,
            floor,
            count: 0,
        }
    }

    /// Adds a single grain of sand at the location of `self.source` and simulates
    /// it until it falls to rest. The location where the grain comes to rest are returned.
    pub fn add_sand_grain(&mut self) -> Loc {
        let mut grain = self.source;

        while grain.y < self.floor - 1 {
            if !self.tiles.contains_key(&grain.offset(0, 1)) {
                grain = grain.offset(0, 1);
            } else if !self.tiles.contains_key(&grain.offset(-1, 1)) {
                grain = grain.offset(-1, 1);
            } else if !self.tiles.contains_key(&grain.offset(1, 1)) {
                grain = grain.offset(1, 1);
            } else {
                break; // Grain settled down before hitting floor.
            }
        }

        if self.tiles.insert(grain, Tile::Sand).is_none() {
            self.count += 1;
        }

        grain
    }

    #[allow(dead_code)]
    pub fn display_colour(&self) -> WorldDisplayColour<'_> {
        WorldDisplayColour { world: self }
    }

    pub fn source(&self) -> Loc {
        self.source
    }

    pub fn floor(&self) -> i32 {
        self.floor
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn left(&self) -> i32 {
        self.tiles.keys().map(|loc| loc.x).min().unwrap_or(0)
    }

    pub fn right(&self) -> i32 {
        self.tiles.keys().map(|loc| loc.x).max().unwrap_or(0)
    }
}

pub struct WorldDisplayColour<'a> {
    world: &'a World,
}

impl Display for WorldDisplayColour<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.world.source.y..=self.world.floor {
            for x in self.world.left()..=self.world.right() {
                let tile = if y == self.world.floor() {
                    Some(&Tile::Rock)
                } else {
                    self.world.tiles.get(&Loc::from((x, y)))
                };

                if let Some(tile) = tile {
                    match tile {
                        Tile::Rock => {
                            SetForegroundColor(Color::Black).write_ansi(f)?;
                            SetBackgroundColor(Color::DarkGrey).write_ansi(f)?;
                            write!(f, " ")?;
                        }
                        Tile::Sand => {
                            SetForegroundColor(Color::Black).write_ansi(f)?;
                            SetBackgroundColor(Color::Yellow).write_ansi(f)?;
                            write!(f, " ")?;
                        }
                    }
                } else if Loc::from((x, y)) == self.world.source() {
                    SetForegroundColor(Color::Black).write_ansi(f)?;
                    SetBackgroundColor(Color::Red).write_ansi(f)?;
                    write!(f, "+")?;
                } else {
                    SetForegroundColor(Color::DarkGrey).write_ansi(f)?;
                    SetBackgroundColor(Color::Reset).write_ansi(f)?;
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        ResetColor.write_ansi(f)?;
        Ok(())
    }
}
