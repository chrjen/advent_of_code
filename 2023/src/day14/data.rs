use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    ops::Range,
};

#[derive(Debug, Clone, Copy, Hash)]
pub enum Rock {
    Round,
    Cubed,
}

#[derive(Debug, Default, Clone)]
pub struct Platform {
    pub x_bound: Range<u32>,
    pub y_bound: Range<u32>,
    pub rocks: HashMap<(u32, u32), Rock>,
}

impl Platform {
    pub fn fall_north(&mut self) -> u64 {
        let mut hasher = DefaultHasher::new();

        for x in self.x_bound.clone() {
            let mut free_y = self.y_bound.start;
            for y in self.y_bound.clone() {
                let rock = self.rocks.get(&(x, y));
                match rock {
                    Some(rock @ Rock::Cubed) => {
                        (x, y, rock).hash(&mut hasher);
                        free_y = y + 1;
                    }
                    Some(rock @ Rock::Round) => {
                        (x, y, rock).hash(&mut hasher);
                        self.rocks.remove(&(x, y));
                        self.rocks.insert((x, free_y), Rock::Round);
                        free_y += 1;
                    }
                    None => {}
                };
            }
        }

        hasher.finish()
    }

    pub fn fall_west(&mut self) -> u64 {
        let mut hasher = DefaultHasher::new();

        for y in self.y_bound.clone() {
            let mut free_x = self.x_bound.start;
            for x in self.x_bound.clone() {
                let rock = self.rocks.get(&(x, y));
                match rock {
                    Some(rock @ Rock::Cubed) => {
                        (x, y, rock).hash(&mut hasher);
                        free_x = x + 1;
                    }
                    Some(rock @ Rock::Round) => {
                        (x, y, rock).hash(&mut hasher);
                        self.rocks.remove(&(x, y));
                        self.rocks.insert((free_x, y), Rock::Round);
                        free_x += 1;
                    }
                    None => {}
                };
            }
        }

        hasher.finish()
    }

    pub fn fall_south(&mut self) -> u64 {
        let mut hasher = DefaultHasher::new();

        for x in self.x_bound.clone() {
            let mut free_y = self.y_bound.end - 1;
            for y in self.y_bound.clone().rev() {
                let rock = self.rocks.get(&(x, y));
                match rock {
                    Some(rock @ Rock::Cubed) => {
                        (x, y, rock).hash(&mut hasher);
                        free_y = y - 1;
                    }
                    Some(rock @ Rock::Round) => {
                        (x, y, rock).hash(&mut hasher);
                        self.rocks.remove(&(x, y));
                        self.rocks.insert((x, free_y), Rock::Round);
                        free_y -= 1;
                    }
                    None => {}
                };
            }
        }

        hasher.finish()
    }

    pub fn fall_east(&mut self) -> u64 {
        let mut hasher = DefaultHasher::new();

        for y in self.y_bound.clone() {
            let mut free_x = self.x_bound.end - 1;
            for x in self.x_bound.clone().rev() {
                let rock = self.rocks.get(&(x, y));
                match rock {
                    Some(rock @ Rock::Cubed) => {
                        (x, y, rock).hash(&mut hasher);
                        free_x = x - 1;
                    }
                    Some(rock @ Rock::Round) => {
                        (x, y, rock).hash(&mut hasher);
                        self.rocks.remove(&(x, y));
                        self.rocks.insert((free_x, y), Rock::Round);
                        free_x -= 1;
                    }
                    None => {}
                };
            }
        }

        hasher.finish()
    }

    pub fn load_north(&self) -> u32 {
        self.rocks
            .iter()
            .map(|((_, y), rock)| match rock {
                Rock::Round => self.y_bound.end - y,
                Rock::Cubed => 0,
            })
            .sum()
    }
}
