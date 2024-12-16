use nalgebra::Vector2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub const fn to_vector(self) -> Vector2<i32> {
        match self {
            Direction::North => Vector2::new(0, -1),
            Direction::East => Vector2::new(1, 0),
            Direction::South => Vector2::new(0, 1),
            Direction::West => Vector2::new(-1, 0),
        }
    }

    pub const fn rot_cost(self, new_dir: Self) -> u32 {
        match (self, new_dir) {
            (Direction::North, Direction::North)
            | (Direction::South, Direction::South)
            | (Direction::West, Direction::West)
            | (Direction::East, Direction::East) => 0,
            (Direction::North, Direction::East)
            | (Direction::North, Direction::West)
            | (Direction::East, Direction::North)
            | (Direction::East, Direction::South)
            | (Direction::South, Direction::East)
            | (Direction::South, Direction::West)
            | (Direction::West, Direction::North)
            | (Direction::West, Direction::South) => 1000,
            (Direction::East, Direction::West)
            | (Direction::West, Direction::East)
            | (Direction::North, Direction::South)
            | (Direction::South, Direction::North) => 2000,
        }
    }

    pub fn all_directions() -> impl Iterator<Item = Direction> {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileNode {
    pub pos: Vector2<i32>,
    pub dir: Direction,
}

impl TileNode {
    pub fn new(pos: Vector2<i32>, dir: Direction) -> Self {
        TileNode { pos, dir }
    }
}
