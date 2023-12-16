use std::{
    fmt::Display,
    mem,
    ops::{Index, IndexMut},
};

pub const SOLUTION: common::Solver = common::Solver {
    name: "Day 18: Like a GIF For Your Yard",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

fn str_to_grid(s: &str) -> Grid {
    let mut width = 0;
    let mut height = 0;
    let mut cells = Vec::new();

    let mut first = true;
    for (i, c) in s.chars().enumerate() {
        match c {
            '\n' => {
                if first {
                    first = false;
                    width = i;
                    height += 1;
                } else {
                    if (i + 1) % (width + 1) != 0 {
                        panic!("lines at different width");
                    }
                    height += 1;
                }
            }
            '#' => cells.push(true),
            '.' => cells.push(false),
            c => panic!("illegal character: {}", c),
        }
    }

    Grid {
        width,
        height,
        cells,
    }
}

impl Grid {
    /// Returns the number of neighbouring cells that are currently lit.
    fn num_neighbours(&self, x: usize, y: usize) -> usize {
        if x >= self.width || y >= self.height {
            panic!(
                "index exceeded width or height: x={}, y={}, w={}, h={}",
                x, y, self.width, self.height
            );
        }

        const TOP_L: usize = 0;
        const TOP_M: usize = 1;
        const TOP_R: usize = 2;
        const MID_L: usize = 3;
        const MID_R: usize = 4;
        const BOT_L: usize = 5;
        const BOT_M: usize = 6;
        const BOT_R: usize = 7;

        let mut count = 0;
        let mut check = [true; 8];

        if x == 0 {
            check[TOP_L] = false;
            check[MID_L] = false;
            check[BOT_L] = false;
        }
        if y == 0 {
            check[TOP_L] = false;
            check[TOP_M] = false;
            check[TOP_R] = false;
        }
        if x == self.width - 1 {
            check[TOP_R] = false;
            check[MID_R] = false;
            check[BOT_R] = false;
        }
        if y == self.height - 1 {
            check[BOT_L] = false;
            check[BOT_M] = false;
            check[BOT_R] = false;
        }

        if check[TOP_L] && self.cells[self.width * (y - 1) + (x - 1)] {
            count += 1;
        }
        if check[TOP_M] && self.cells[self.width * (y - 1) + x] {
            count += 1;
        }
        if check[TOP_R] && self.cells[self.width * (y - 1) + (x + 1)] {
            count += 1;
        }
        if check[MID_L] && self.cells[self.width * y + (x - 1)] {
            count += 1;
        }
        if check[MID_R] && self.cells[self.width * y + (x + 1)] {
            count += 1;
        }
        if check[BOT_L] && self.cells[self.width * (y + 1) + (x - 1)] {
            count += 1;
        }
        if check[BOT_M] && self.cells[self.width * (y + 1) + x] {
            count += 1;
        }
        if check[BOT_R] && self.cells[self.width * (y + 1) + (x + 1)] {
            count += 1;
        }

        count
    }

    fn iter(&self) -> GridIter {
        GridIter {
            grid: self,
            idx: 0,
            len: self.height * self.width,
        }
    }
}

impl Index<usize> for Grid {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index]
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = if self[self.width * y + x] { '#' } else { '.' };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct GridIter<'a> {
    grid: &'a Grid,
    idx: usize,
    len: usize,
}

impl<'a> Iterator for GridIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let item: Option<Self::Item>;

        if self.idx < self.len {
            item = Some(self.grid[self.idx]);
            self.idx += 1;
        } else {
            item = None;
        }

        item
    }
}

/// Takes the state of the grid in `src` and steps forward one step and writes
/// the resulting state to `dst`.
fn step(dst: &mut Grid, src: &Grid) {
    for y in 0..src.height {
        for x in 0..src.width {
            let n = src.num_neighbours(x, y);
            let idx = dst.width * y + x;

            if src[idx] {
                dst[idx] = n == 2 || n == 3;
            } else {
                dst[idx] = n == 3;
            }
        }
    }
}

/// Same as [`step`], but leaves all corners as is.
fn step_ignore_corners(dst: &mut Grid, src: &Grid) {
    for y in 0..src.height {
        let range = if y == 0 || y == src.height - 1 {
            1..src.width - 1
        } else {
            0..src.width
        };

        for x in range {
            let n = src.num_neighbours(x, y);
            let idx = dst.width * y + x;

            if src[idx] {
                dst[idx] = n == 2 || n == 3;
            } else {
                dst[idx] = n == 3;
            }
        }
    }
}

fn solve_(input: &str, iterations: usize, ignore_corners: bool) -> usize {
    let mut src = str_to_grid(input);
    let mut dst = src.clone();

    if ignore_corners {
        for _ in 0..iterations {
            step_ignore_corners(&mut dst, &src);
            mem::swap(&mut src, &mut dst);
        }
    } else {
        for _ in 0..iterations {
            step(&mut dst, &src);
            mem::swap(&mut src, &mut dst);
        }
    }

    src.iter().filter(|&x| x).count()
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let part1 = solve_(&input, 100, false);
    let part2 = solve_(&input, 100, true);
    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::solution;

    // Part 1
    #[test]
    fn p1_example_1() {
        let input = r#".#.#.#
...##.
#....#
..#...
#.#..#
####..
"#;
        assert_eq!(solve_(input, 4, false), 4);
    }
    solution!(p1, p1_solution, "768");

    // Part 2
    #[test]
    fn p2_example_1() {
        let input = r#"##.#.#
...##.
#....#
..#...
#.#..#
####.#
"#;
        assert_eq!(solve_(input, 5, true), 17);
    }
    solution!(p2, p2_solution, "781");
}
