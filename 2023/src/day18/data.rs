use std::fmt::{self, Debug};

pub type Coord = (i64, i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Left,
    Down,
    Right,
}

impl Dir {
    pub fn offset(&self) -> Coord {
        match self {
            Dir::Up => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Down => (0, -1),
            Dir::Right => (1, 0),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Colour(pub u32);

impl Colour {
    /// Returns the red component of the colour.
    pub fn red(&self) -> u8 {
        ((self.0 >> 16) & 0xff) as u8
    }

    /// Returns the red component of the colour.
    pub fn green(&self) -> u8 {
        ((self.0 >> 8) & 0xff) as u8
    }

    /// Returns the red component of the colour.
    pub fn blue(&self) -> u8 {
        (self.0 & 0xff) as u8
    }

    /// Returns the direction component of the "colour".
    pub fn dir(&self) -> Dir {
        match self.0 % 4 {
            0 => Dir::Right,
            1 => Dir::Down,
            2 => Dir::Left,
            3 => Dir::Up,
            _ => unreachable!(),
        }
    }

    /// Returns the distance component of the "colour".
    pub fn dist(&self) -> usize {
        (self.0 / 16) as usize
    }
}

impl Debug for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:06X}", self.0)
    }
}

impl From<Colour> for crossterm::style::Color {
    fn from(val: Colour) -> crossterm::style::Color {
        crossterm::style::Color::Rgb {
            r: val.red(),
            g: val.green(),
            b: val.blue(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DigStep {
    /// Direction to dig.
    pub dir: Dir,
    /// Distance to dig.
    pub dist: usize,
    /// Colour to paint the edge.
    pub colour: Colour,
}

impl DigStep {
    pub fn from_colour(c: Colour) -> Self {
        Self {
            dir: c.dir(),
            dist: c.dist(),
            colour: c,
        }
    }
}
