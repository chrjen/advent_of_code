use nalgebra::Vector2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    Robot,
}

impl Tile {
    pub const fn try_from_char(c: char) -> Result<Tile, char> {
        match c {
            '#' => Ok(Tile::Wall),
            'O' => Ok(Tile::Box),
            '@' => Ok(Tile::Robot),
            '[' => Ok(Tile::BoxLeft),
            ']' => Ok(Tile::BoxRight),
            _ => Err(c),
        }
    }

    #[allow(dead_code)]
    pub const fn into_char(self) -> char {
        match self {
            Tile::Wall => '#',
            Tile::Box => 'O',
            Tile::Robot => '@',
            Tile::BoxLeft => '[',
            Tile::BoxRight => ']',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub const fn try_from_char(c: char) -> Result<Direction, char> {
        match c {
            '^' => Ok(Direction::North),
            '>' => Ok(Direction::East),
            'v' => Ok(Direction::South),
            '<' => Ok(Direction::West),
            _ => Err(c),
        }
    }

    pub const fn to_vec(self) -> Vector2<i32> {
        match self {
            Direction::North => Vector2::new(0, -1),
            Direction::East => Vector2::new(1, 0),
            Direction::South => Vector2::new(0, 1),
            Direction::West => Vector2::new(-1, 0),
        }
    }
}
