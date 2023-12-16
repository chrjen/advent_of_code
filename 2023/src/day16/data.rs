use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

pub type Coord = (usize, usize);

#[derive(Debug, Clone, Copy)]
pub enum Mirror {
    /// Left leaning mirror. Redirect light from the left, down.
    Left,
    /// Right leaning mirror. Redirect light from the left, up.
    Right,
    /// Vertical splitter.
    SplitV,
    /// Horizontal splitter.
    SplitH,
}

impl Mirror {
    pub fn reflect_dir(&self, dir: &Direction) -> (Direction, Option<Direction>) {
        use Direction as Dir;
        match (dir, self) {
            (Dir::North | Dir::South, Mirror::Left) => (dir.ccw(), None),
            (Dir::West | Dir::East, Mirror::Left) => (dir.cw(), None),
            (Dir::North | Dir::South, Mirror::Right) => (dir.cw(), None),
            (Dir::West | Dir::East, Mirror::Right) => (dir.ccw(), None),
            (Dir::North | Dir::South, Mirror::SplitV) => (*dir, None),
            (Dir::West | Dir::East, Mirror::SplitH) => (*dir, None),
            (_, Mirror::SplitH | Mirror::SplitV) => (dir.ccw(), Some(dir.cw())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    /// Returns new coordinates offset by 1 in this direction.
    pub fn offset_1(&self, (x, y): Coord) -> Coord {
        match self {
            Direction::North => (x, y - 1),
            Direction::West => (x - 1, y),
            Direction::South => (x, y + 1),
            Direction::East => (x + 1, y),
        }
    }

    /// Returns the direction 90-degrees counterclockwise to this one.
    pub fn ccw(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    /// Returns the direction 90-degrees clockwise to this one.
    pub fn cw(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Contraption {
    pub x_bound: Range<usize>,
    pub y_bound: Range<usize>,
    pub mirrors: HashMap<Coord, Mirror>,
}

impl Contraption {
    pub fn is_within_bounds(&self, (x, y): &Coord) -> bool {
        self.x_bound.contains(x) && self.y_bound.contains(y)
    }

    pub fn fire_beam(&self, initial: (Coord, Direction)) -> HashSet<Coord> {
        let mut next = vec![initial];
        let mut visited = HashSet::new();

        while let Some((coord, dir)) = next.pop() {
            if !self.is_within_bounds(&coord) || !visited.insert((coord, dir)) {
                continue;
            }

            // Move in the direction and possibly add to next.
            let mut move_in = |new_dir: Direction| {
                let new_coord = new_dir.offset_1(coord);
                next.push((new_coord, new_dir));
            };

            if let Some(mirror) = self.mirrors.get(&coord) {
                match mirror.reflect_dir(&dir) {
                    (new_dir, Some(other_dir)) => {
                        move_in(new_dir);
                        move_in(other_dir);
                    }
                    (new_dir, None) => {
                        move_in(new_dir);
                    }
                }
            } else {
                move_in(dir);
            }
        }

        visited.into_iter().map(|(coord, _)| coord).collect()
    }
}
