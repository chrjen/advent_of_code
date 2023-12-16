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
        match (dir, self) {
            (Direction::North, Mirror::Left) => (Direction::West, None),
            (Direction::North, Mirror::Right) => (Direction::East, None),
            (Direction::West, Mirror::Left) => (Direction::North, None),
            (Direction::West, Mirror::Right) => (Direction::South, None),
            (Direction::South, Mirror::Left) => (Direction::East, None),
            (Direction::South, Mirror::Right) => (Direction::West, None),
            (Direction::East, Mirror::Left) => (Direction::South, None),
            (Direction::East, Mirror::Right) => (Direction::North, None),
            (Direction::North, Mirror::SplitV) => (Direction::North, None),
            (Direction::North, Mirror::SplitH) => (Direction::West, Some(Direction::East)),
            (Direction::West, Mirror::SplitV) => (Direction::North, Some(Direction::South)),
            (Direction::West, Mirror::SplitH) => (Direction::West, None),
            (Direction::South, Mirror::SplitV) => (Direction::South, None),
            (Direction::South, Mirror::SplitH) => (Direction::West, Some(Direction::East)),
            (Direction::East, Mirror::SplitV) => (Direction::North, Some(Direction::South)),
            (Direction::East, Mirror::SplitH) => (Direction::East, None),
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
            if !visited.insert((coord, dir)) {
                continue;
            }

            if coord == (1, 8) {
                let _x = 0;
            }

            if let Some(mirror) = self.mirrors.get(&coord) {
                match mirror.reflect_dir(&dir) {
                    (new_dir, Some(other_dir)) => {
                        let new_coord = new_dir.offset_1(coord);
                        if self.is_within_bounds(&new_coord)
                            && !visited.contains(&(new_coord, new_dir))
                        {
                            next.push((new_coord, new_dir));
                        }

                        let new_coord = other_dir.offset_1(coord);
                        if self.is_within_bounds(&new_coord)
                            && !visited.contains(&(new_coord, other_dir))
                        {
                            next.push((new_coord, other_dir));
                        }
                    }
                    (new_dir, None) => {
                        let new_coord = new_dir.offset_1(coord);
                        if self.is_within_bounds(&new_coord)
                            && !visited.contains(&(new_coord, new_dir))
                        {
                            next.push((new_coord, new_dir));
                        }
                    }
                }
            } else {
                let new_dir = dir;
                let new_coord = dir.offset_1(coord);
                if self.is_within_bounds(&new_coord) && !visited.contains(&(new_coord, new_dir)) {
                    next.push((new_coord, new_dir));
                }
            }
        }

        visited.into_iter().map(|(coord, _)| coord).collect()
    }
}
