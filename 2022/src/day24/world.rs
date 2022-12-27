use std::{
    collections::{HashMap, HashSet},
    mem,
};

#[derive(Debug, Clone, Copy)]
enum WindDirection {
    Up,
    Down,
    Left,
    Right,
}

impl WindDirection {
    /// Calculates the current position of the wind after `step` steps. The wind
    /// is thought of as mostly one-dimensional, wrapping around before hitting
    /// a wall.
    fn move_n(&self, start: usize, step: usize, world: &World) -> usize {
        let (range, negative) = match self {
            WindDirection::Up => (world.top + 1..world.bottom, true),
            WindDirection::Left => (world.left + 1..world.right, true),
            WindDirection::Down => (world.top + 1..world.bottom, false),
            WindDirection::Right => (world.left + 1..world.right, false),
        };

        if negative {
            // Making use of the following property of modular arithmetic to avoid negative numbers.
            // -m ≡ m * (n-1)  (mod n)
            ((step % range.len()) * (range.len() - 1) + (start - range.start)) % range.len()
                + range.start
        } else {
            ((step % range.len()) + (start - range.start)) % range.len() + range.start
        }
    }
}

type X = usize;
type Y = usize;

#[derive(Debug, Default, Clone)]
pub struct World {
    walls: HashSet<(X, Y)>,
    wind_vertical: HashMap<X, Vec<(Y, WindDirection)>>,
    wind_horizontal: HashMap<Y, Vec<(X, WindDirection)>>,
    start: (X, Y),
    end: (X, Y),
    top: usize,
    left: usize,
    bottom: usize,
    right: usize,
}

impl World {
    /// Creates a world from a given input.
    pub fn from_str(input: &str) -> World {
        let chars = input.lines().enumerate().flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, c)| (row + 1, col + 1, c))
        });

        let mut world = World {
            top: usize::MAX,
            left: usize::MAX,
            bottom: usize::MIN,
            right: usize::MIN,
            ..World::default()
        };

        for (row, col, c) in chars {
            world.top = world.left.min(row);
            world.left = world.left.min(col);
            world.bottom = world.left.max(row);
            world.right = world.right.max(col);

            match c {
                '#' => {
                    world.walls.insert((col, row));
                }
                '<' | '>' | '^' | 'v' => {
                    let (wind_map, key, wind) = match c {
                        '>' => (&mut world.wind_horizontal, row, (col, WindDirection::Right)),
                        '<' => (&mut world.wind_horizontal, row, (col, WindDirection::Left)),
                        '^' => (&mut world.wind_vertical, col, (row, WindDirection::Up)),
                        'v' => (&mut world.wind_vertical, col, (row, WindDirection::Down)),
                        _ => unreachable!(),
                    };
                    wind_map
                        .entry(key)
                        .and_modify(|v| v.push(wind))
                        .or_insert_with(|| vec![wind]);
                }
                _ => {}
            }
        }

        // Find start.
        for x in world.left..world.right {
            if !world.walls.contains(&(x, world.top)) {
                world.start = (x, world.top);
                break;
            }
        }

        // Find end.
        for x in world.left..world.right {
            if !world.walls.contains(&(x, world.bottom)) {
                world.end = (x, world.bottom);
                break;
            }
        }

        world
    }

    /// Does path-finding from `start` to `end` with all winds starting after `step`
    /// simulation steps.
    pub fn walk(&self, step: &mut usize, start: (usize, usize), end: (usize, usize)) {
        let mut current_step = HashSet::new();
        let mut next_step = HashSet::new();
        let mut visited = HashSet::new();
        current_step.insert(start);

        'outer: loop {
            for &(x, y) in current_step.iter() {
                if (x, y) == end {
                    break 'outer;
                }

                let neighbours =
                    [(x, y), (x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].into_iter();
                for neighbour in neighbours {
                    if self.is_empty(*step + 1, neighbour) {
                        next_step.insert(neighbour);
                    }
                }

                visited.insert((*step, x, y));
            }

            mem::swap(&mut current_step, &mut next_step);
            next_step.clear();
            *step += 1;
        }
    }

    /// Checks if a given tile is empty at a certain simulation step.
    fn is_empty(&self, step: usize, (x, y): (usize, usize)) -> bool {
        let is_wind = self
            .wind_vertical
            .get(&x)
            .and_then(|v| v.iter().find(|(y_, dir)| dir.move_n(*y_, step, self) == y))
            .or_else(|| {
                self.wind_horizontal
                    .get(&y)
                    .and_then(|v| v.iter().find(|(x_, dir)| dir.move_n(*x_, step, self) == x))
            })
            .is_some();

        let is_wall = self.walls.contains(&(x, y));

        !is_wind
            && !is_wall
            && x >= self.left
            && x <= self.right
            && y >= self.top
            && y <= self.bottom
    }

    pub fn start(&self) -> (usize, usize) {
        self.start
    }

    pub fn end(&self) -> (usize, usize) {
        self.end
    }

    #[allow(dead_code)]
    pub fn print_for_steps(&self, step_from: usize, step_until: usize) {
        for step in step_from..step_until {
            println!();
            println!("Step {step}:");
            for y in self.top..=self.bottom {
                for x in self.left..=self.right {
                    if let Some(&(_, dir)) = self
                        .wind_vertical
                        .get(&x)
                        .and_then(|v| v.iter().find(|(y_, dir)| dir.move_n(*y_, step, self) == y))
                        .or_else(|| {
                            self.wind_horizontal.get(&y).and_then(|v| {
                                v.iter().find(|(x_, dir)| dir.move_n(*x_, step, self) == x)
                            })
                        })
                    {
                        match dir {
                            WindDirection::Up => print!("^"),
                            WindDirection::Down => print!("v"),
                            WindDirection::Left => print!("<"),
                            WindDirection::Right => print!(">"),
                        }
                    } else if self.walls.contains(&(x, y)) {
                        print!("#");
                    } else if self.start == (x, y) {
                        print!("S");
                    } else if self.end == (x, y) {
                        print!("E");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
        }
    }
}
