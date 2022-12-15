use std::{collections::HashMap, fmt::Display, ops::RangeInclusive};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Loc {
    pub x: i32,
    pub y: i32,
}

impl Loc {
    pub fn manhattan(self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl From<(i32, i32)> for Loc {
    fn from(t: (i32, i32)) -> Self {
        Loc { x: t.0, y: t.1 }
    }
}

pub enum Tile {
    Sensor(u32), // Contains the distance/radius to the nearest beacon.
    Beacon,
}

/// Represents the world. In this case the large network of subterranean tunnels
/// as per the puzzle text.
pub struct World {
    pub tiles: HashMap<Loc, Tile>,
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.top()..self.bottom() {
            for x in self.left()..=self.right() {
                let loc = Loc::from((x, y));

                let tile = self.tiles.get(&loc);

                if let Some(tile) = tile {
                    match tile {
                        Tile::Sensor(_) => {
                            write!(f, "S")?;
                        }
                        Tile::Beacon => {
                            write!(f, "B")?;
                        }
                    }
                } else if self.within_sensor_range(loc).is_some() {
                    write!(f, "#")?;
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
    /// Returns an iterator over all the sensors in the world.
    pub fn sensors(&self) -> impl Iterator<Item = (&Loc, &u32)> {
        self.tiles.iter().filter_map(|(loc, tile)| {
            if let Tile::Sensor(r) = tile {
                Some((loc, r))
            } else {
                None
            }
        })
    }

    /// Scans the line `y` and counts how many *empty* tiles on that line are within
    /// the range of a sensor.
    pub fn scan_line(&self, y: i32) -> usize {
        let mut count = 0;

        for x in self.left()..self.right() {
            let current = Loc { x, y };

            if self.tiles.contains_key(&current) {
                continue;
            }

            if self.within_sensor_range(current).is_some() {
                count += 1;
            }
        }

        count
    }

    pub fn locate_beacon(
        &self,
        x_range: RangeInclusive<i32>,
        y_range: RangeInclusive<i32>,
    ) -> Option<Loc> {
        for y in y_range {
            let mut current = Loc { x: 0, y };
            while x_range.contains(&current.x) {
                // Iterate over all sensors we are within range of. For each sensor
                // calculate how much we need to add to the x-value to get out of range
                // of the sensor. Then we take the biggest and add it to x. If no sensors
                // are within range then `dx` is `None` and we have found our answer.
                let dx = self
                    .sensors()
                    .filter(|(sensor, radius)| sensor.manhattan(&current) <= **radius)
                    .map(|(sensor, &radius)| {
                        let (dx, dy) = (sensor.x - current.x, sensor.y - current.y);
                        dx.saturating_add_unsigned(radius) + 1 - dy.abs()
                    })
                    .max();

                if let Some(dx) = dx {
                    current.x += dx;
                } else {
                    return Some(current);
                }
            }
        }

        None
    }

    /// Returns the location and radius of the first sensor that `loc` is in
    /// range of. `None` if `loc` is not within the range of any sensor.
    fn within_sensor_range(&self, loc: Loc) -> Option<(Loc, u32)> {
        for (&sensor, &radius) in self.sensors() {
            if sensor.manhattan(&loc) <= radius {
                return Some((sensor, radius));
            }
        }
        None
    }

    /// Left-most x-value.
    pub fn left(&self) -> i32 {
        self.sensors()
            .map(|(loc, r)| loc.x - *r as i32)
            .min()
            .unwrap_or(0)
    }

    /// Right-most x-value.
    pub fn right(&self) -> i32 {
        self.sensors()
            .map(|(loc, r)| loc.x + *r as i32)
            .max()
            .unwrap_or(0)
    }

    /// Top-most y-value.
    pub fn top(&self) -> i32 {
        self.sensors()
            .map(|(loc, r)| loc.y - *r as i32)
            .min()
            .unwrap_or(0)
    }

    /// Bottom-most y-value.
    pub fn bottom(&self) -> i32 {
        self.sensors()
            .map(|(loc, r)| loc.y + *r as i32)
            .max()
            .unwrap_or(0)
    }
}
