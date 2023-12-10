use std::{collections::HashMap, fmt::Display};

use super::Coord;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
pub enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl Pipe {
    /// Takes the current position and a from direction and returns the new position and direction
    /// if the input makes sense for the type of pipe. If the pipe does not have an end from the
    /// direction you are coming from, None is returned instead.
    ///
    /// The direction `from` always mean the direction your back is facing. So if you are travelling
    /// Northwards then you are coming from [Direction::South]. If you then enter a [Pipe::SouthEast]
    /// pipe, then you end up travelling East afterwards, so your new `from` direction would become
    /// [Direction::West] as that is where you back is facing.
    pub fn travel(&self, (x, y): Coord, from: Direction) -> Option<(Coord, Direction)> {
        match (*self, from) {
            (Pipe::Vertical, Direction::North) => Some(((x, y + 1), Direction::North)),
            (Pipe::Vertical, Direction::South) => Some(((x, y - 1), Direction::South)),
            (Pipe::Vertical, Direction::East) => None,
            (Pipe::Vertical, Direction::West) => None,
            (Pipe::Horizontal, Direction::North) => None,
            (Pipe::Horizontal, Direction::South) => None,
            (Pipe::Horizontal, Direction::East) => Some(((x - 1, y), Direction::East)),
            (Pipe::Horizontal, Direction::West) => Some(((x + 1, y), Direction::West)),
            (Pipe::NorthEast, Direction::North) => Some(((x + 1, y), Direction::West)),
            (Pipe::NorthEast, Direction::South) => None,
            (Pipe::NorthEast, Direction::East) => Some(((x, y - 1), Direction::South)),
            (Pipe::NorthEast, Direction::West) => None,
            (Pipe::NorthWest, Direction::North) => Some(((x - 1, y), Direction::East)),
            (Pipe::NorthWest, Direction::South) => None,
            (Pipe::NorthWest, Direction::East) => None,
            (Pipe::NorthWest, Direction::West) => Some(((x, y - 1), Direction::South)),
            (Pipe::SouthWest, Direction::North) => None,
            (Pipe::SouthWest, Direction::South) => Some(((x - 1, y), Direction::East)),
            (Pipe::SouthWest, Direction::East) => None,
            (Pipe::SouthWest, Direction::West) => Some(((x, y + 1), Direction::North)),
            (Pipe::SouthEast, Direction::North) => None,
            (Pipe::SouthEast, Direction::South) => Some(((x + 1, y), Direction::West)),
            (Pipe::SouthEast, Direction::East) => Some(((x, y + 1), Direction::North)),
            (Pipe::SouthEast, Direction::West) => None,
        }
    }

    /// Returns a new [Pipe] from the given [directions](Direction). The directions
    /// are from directions which mean where you back is facing as you enter and
    /// exit the Pipe. Example [Pipe::SouthWest] will be returned with either of
    /// the two combinations below:
    ///
    /// `enter_from = Direction::South` and `exit_from = Direction::East`
    ///
    /// or
    ///
    /// `enter_from = Direction::West` and `exit_from = Direction::North`
    ///
    /// There are no 180-degree/U-turn tubes so this will panic if `enter_from` and
    /// `exit_from` are opposite from each other.
    pub fn from_directions(enter_from: Direction, exit_from: Direction) -> Self {
        match (enter_from, exit_from) {
            (Direction::North, Direction::North) | (Direction::South, Direction::South) => {
                Pipe::Vertical
            }
            (Direction::East, Direction::East) | (Direction::West, Direction::West) => {
                Pipe::Horizontal
            }
            (Direction::North, Direction::East) | (Direction::West, Direction::South) => {
                Pipe::NorthWest
            }
            (Direction::North, Direction::West) | (Direction::East, Direction::South) => {
                Pipe::NorthEast
            }
            (Direction::South, Direction::East) | (Direction::West, Direction::North) => {
                Pipe::SouthWest
            }
            (Direction::South, Direction::West) | (Direction::East, Direction::North) => {
                Pipe::SouthEast
            }
            (Direction::North, Direction::South)
            | (Direction::South, Direction::North)
            | (Direction::East, Direction::West)
            | (Direction::West, Direction::East) => {
                panic!("no pipe can exit same place it entered")
            }
        }
    }
}

impl TryFrom<char> for Pipe {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '|' => Ok(Pipe::Vertical),
            '-' => Ok(Pipe::Horizontal),
            'L' => Ok(Pipe::NorthEast),
            'J' => Ok(Pipe::NorthWest),
            '7' => Ok(Pipe::SouthWest),
            'F' => Ok(Pipe::SouthEast),
            c => Err(format!("unknown pipe, got '{c}'")),
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Pipe::Vertical => '│',
                Pipe::Horizontal => '─',
                Pipe::NorthEast => '└',
                Pipe::NorthWest => '┘',
                Pipe::SouthWest => '┐',
                Pipe::SouthEast => '┌',
            }
        )
    }
}

#[derive(Debug, Default, Clone)]
pub struct Field {
    pub start: Coord,
    pub layout: HashMap<Coord, Pipe>,
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x_bound, y_bound) = super::get_bounds(self.layout.keys());

        for y in y_bound {
            for x in x_bound.clone() {
                if self.start == (x, y) {
                    write!(f, "S")?;
                } else {
                    let pipe = self.layout.get(&(x, y));
                    match pipe {
                        Some(pipe) => write!(f, "{}", pipe)?,
                        None => write!(f, ".")?,
                    }
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
