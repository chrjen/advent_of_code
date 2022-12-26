use std::{
    collections::HashSet,
    iter::{Copied, Cycle, Enumerate},
    slice::Iter,
};

const ROCK_BIT_SIZE: usize = 3;
const WIND_BIT_SIZE: usize = 14;

#[derive(Debug, Clone, Copy)]
pub enum Wind {
    Right,
    Left,
}

#[derive(Debug, Clone, Copy)]
pub enum Rock {
    Minus,
    Plus,
    Corner,
    Bar,
    Box,
}

impl Rock {
    pub const fn shape(&self) -> &'static [(i64, i64)] {
        match self {
            Rock::Minus => &[(0, 0), (1, 0), (2, 0), (3, 0)],
            Rock::Plus => &[(0, 1), (1, 0), (1, 1), (2, 1), (1, 2)],
            Rock::Corner => &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Rock::Bar => &[(0, 0), (0, 1), (0, 2), (0, 3)],
            Rock::Box => &[(0, 0), (1, 0), (0, 1), (1, 1)],
        }
    }

    #[allow(unused)]
    pub const fn width(&self) -> i64 {
        match self {
            Rock::Minus => 4,
            Rock::Plus => 3,
            Rock::Corner => 3,
            Rock::Bar => 1,
            Rock::Box => 2,
        }
    }

    pub const fn height(&self) -> i64 {
        match self {
            Rock::Minus => 1,
            Rock::Plus => 3,
            Rock::Corner => 3,
            Rock::Bar => 4,
            Rock::Box => 2,
        }
    }

    pub fn iter() -> Copied<Iter<'static, Rock>> {
        use Rock::*;
        [Minus, Plus, Corner, Bar, Box].iter().copied()
    }
}

pub struct World<I, I2>
where
    I: Iterator<Item = (usize, Wind)> + Clone,
    I2: Iterator<Item = (usize, Rock)> + Clone,
{
    rocks: HashSet<(i64, i64)>,
    rock_iter: Cycle<I2>,
    wind: Cycle<I>,
    top: i64,
    bottom: i64,
    left: i64,
    right: i64,
}

impl<I> World<Enumerate<I>, Enumerate<Copied<Iter<'_, Rock>>>>
where
    I: Iterator<Item = Wind> + Clone,
{
    pub fn new(wind: I, left: i64, right: i64, bottom: i64) -> Self {
        World {
            rocks: HashSet::new(),
            rock_iter: Rock::iter().enumerate().cycle(),
            wind: wind.enumerate().cycle(),
            top: bottom,
            bottom,
            left,
            right,
        }
    }

    pub fn add_rock(&mut self, loc: (i64, i64), rock: Rock) {
        let (x, y) = loc;
        for &(dx, dy) in rock.shape() {
            if !self.rocks.insert((x + dx, y + dy)) {
                panic!("rock already inserted");
            }
        }
    }

    /// Drops a single rock in the world, simulating it until is finally settles.
    ///
    /// ## Return
    /// A single number that tries to represent the current state of the world
    /// in a single number for easy comparisons of world state. The state value
    /// returned is much smaller (in bits) than all states possible so is not a
    /// perfect representation of the internal state of the world.
    pub fn drop_rock(&mut self) -> u64 {
        let (rock_idx, rock) = self.rock_iter.next().unwrap(); // Should always be `Some`.
        let (mut rock_x, mut rock_y) = (self.left + 2, self.top + 3);

        let wind_idx = loop {
            let (wind_idx, wind) = self.wind.next().unwrap(); // Should always be `Some`.

            match wind {
                Wind::Right => {
                    if !self.is_colliding((rock_x + 1, rock_y), rock) {
                        rock_x += 1;
                    }
                }
                Wind::Left => {
                    if !self.is_colliding((rock_x - 1, rock_y), rock) {
                        rock_x -= 1;
                    }
                }
            }

            if !self.is_colliding((rock_x, rock_y - 1), rock) {
                rock_y -= 1;
            } else {
                self.add_rock((rock_x, rock_y), rock);
                self.top = self.top.max(rock_y + rock.height());

                break wind_idx;
            }
        };

        // Calculate return type.
        let mut world_state: u64 = 0;
        for y in self.top - 9..=self.top {
            for x in self.left..self.right {
                world_state <<= 1;
                if self.rocks.contains(&(x, y)) {
                    world_state |= 1;
                }
            }
        }

        if wind_idx >> WIND_BIT_SIZE != 0 {
            panic!("number of wind states exceeded max allowed");
        }

        if rock_idx >> ROCK_BIT_SIZE != 0 {
            panic!("number of rock states exceeded max allowed");
        }

        world_state <<= WIND_BIT_SIZE;
        world_state |= wind_idx as u64;
        world_state <<= ROCK_BIT_SIZE;
        world_state |= rock_idx as u64;
        world_state
    }

    pub fn is_colliding(&self, loc: (i64, i64), rock: Rock) -> bool {
        let (x, y) = loc;
        rock.shape().iter().any(|(dx, dy)| {
            let (x, y) = (x + dx, y + dy);
            x < self.left || x > self.right || self.rocks.contains(&(x, y)) || y < self.bottom
        })
    }

    pub fn top(&self) -> i64 {
        self.top
    }
}
