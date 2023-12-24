use std::collections::HashMap;

pub type Coord = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Forest,
    SlopeNorth,
    SlopeEast,
    SlopeSouth,
    SlopeWest,
}

#[derive(Debug, Clone)]
pub struct HikingTrails {
    pub start: Coord,
    pub end: Coord,
    pub map: HashMap<Coord, Tile>,
}

impl HikingTrails {
    pub fn new() -> Self {
        HikingTrails {
            start: (0, 0),
            end: (0, 0),
            map: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HikingTrailsRef<'a> {
    pub start: Coord,
    pub end: Coord,
    pub map: &'a HashMap<Coord, Tile>,
}
