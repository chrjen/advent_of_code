use std::fmt::Display;

use itertools::Itertools;
use ndarray::Array2;
use num::{Signed, Zero, rational::Ratio};

pub type Num = Ratio<i128>;

#[derive(Debug, Clone)]
pub struct Trajectory {
    pub px: Num,
    pub py: Num,
    pub pz: Num,
    pub vx: Num,
    pub vy: Num,
    pub vz: Num,
}

impl Display for Trajectory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {} @ {}, {}, {}",
            self.px, self.py, self.pz, self.vx, self.vy, self.vz
        )
    }
}

impl Trajectory {
    /// Given two trajectories this returns the point at where they intersect if it
    /// exist. This does not take time into account, so the point does not
    /// necessarily represent a collision if the time of intersection is different
    /// between the two hailstone trajectories.
    ///
    /// Formula taken from [Wikipedia](https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection).
    /// Given to points on each line.
    ///
    /// ## Return
    /// The x and y coordinates of the intersection, or `None` if the trajectories
    /// are parallell or coincident (same line).
    pub fn trajectory_intersection(t0: &Trajectory, t1: &Trajectory) -> Option<(Num, Num)> {
        let (x1, y1) = (t0.px, t0.py);
        let (x2, y2) = ((t0.px + t0.vx), (t0.py + t0.vy));
        let (x3, y3) = (t1.px, t1.py);
        let (x4, y4) = ((t1.px + t1.vx), (t1.py + t1.vy));

        let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if denominator.is_zero() {
            return None;
        }

        let x = ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4)) / denominator;
        let y = ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4)) / denominator;

        Some((x, y))
    }

    pub fn rock_trajectory_position(
        t0: &Trajectory,
        t1: &Trajectory,
        t2: &Trajectory,
    ) -> (Num, Num, Num) {
        // Differences of position.
        let (p01x, p01y, p01z) = (t0.px - t1.px, t0.py - t1.py, t0.pz - t1.pz);
        let (p12x, p12y, p12z) = (t1.px - t2.px, t1.py - t2.py, t1.pz - t2.pz);
        let (p20x, p20y, p20z) = (t2.px - t0.px, t2.py - t0.py, t2.pz - t0.pz);

        // Differences of velocity.
        let (v01x, v01y, v01z) = (t0.vx - t1.vx, t0.vy - t1.vy, t0.vz - t1.vz);
        let (v12x, v12y, v12z) = (t1.vx - t2.vx, t1.vy - t2.vy, t1.vz - t2.vz);
        let (v20x, v20y, v20z) = (t2.vx - t0.vx, t2.vy - t0.vy, t2.vz - t0.vz);

        let mut mat = ndarray::arr2(&[
            [
                p01y * v01z - p01z * v01y,
                p01z * v01x - p01x * v01z,
                p01x * v01y - p01y * v01x,
                -t0.px * t1.py * v01z + t0.px * t1.pz * v01y - t0.py * t1.pz * v01x
                    + t0.py * t1.px * v01z
                    - t0.pz * t1.px * v01y
                    + t0.pz * t1.py * v01x,
            ],
            [
                p12y * v12z - p12z * v12y,
                p12z * v12x - p12x * v12z,
                p12x * v12y - p12y * v12x,
                -t1.px * t2.py * v12z + t1.px * t2.pz * v12y - t1.py * t2.pz * v12x
                    + t1.py * t2.px * v12z
                    - t1.pz * t2.px * v12y
                    + t1.pz * t2.py * v12x,
            ],
            [
                p20y * v20z - p20z * v20y,
                p20z * v20x - p20x * v20z,
                p20x * v20y - p20y * v20x,
                -t2.px * t0.py * v20z + t2.px * t0.pz * v20y - t2.py * t0.pz * v20x
                    + t2.py * t0.px * v20z
                    - t2.pz * t0.px * v20y
                    + t2.pz * t0.py * v20x,
            ],
        ]);

        gauss_jordan_elimination(&mut mat);

        let pos: Vec<Num> = mat.column(3).into_iter().copied().collect();

        (pos[0], pos[1], pos[2])
    }

    /// Calculates the time for a given point. This is only guaranteed to be
    /// correct if the point is actually on the trajectory line. If not then
    /// the returned time is undefined.
    ///
    /// ## Panic
    /// Will panic if the hailstone is not moving. I.e. `vx` and `vy` are both
    /// zero.
    pub fn point_time(&self, x: &Num, y: &Num) -> Num {
        if self.vx.is_zero() && self.vy.is_zero() {
            panic!("hailstone has to be moving to calculate time");
        }

        if self.vx.is_zero() {
            (y - self.py) / self.vy
        } else {
            (x - self.px) / self.vx
        }
    }
}

fn gauss_jordan_elimination(mat: &mut Array2<Num>) {
    // First, Gaussian elimination to get row echelon form.
    // Source: https://algorithm-wiki.csail.mit.edu/wiki/Gaussian_Elimination
    let m = 3;
    let n = 4;
    let mut h = 0;
    let mut k = 0;

    while h < m && k < n {
        let i_max = h + mat
            .column(k)
            .iter()
            .skip(h)
            .position_max_by_key(|v| v.abs())
            .unwrap();

        // Failed to find pivot, try next column.
        if mat[[i_max, k]] == Num::from(0) {
            k += 1;
            continue;
        }

        // Swap rows with pivot.
        for j in 0..n {
            mat.swap([h, j], [i_max, j]);
        }

        for i in h + 1..m {
            let f = mat[[i, k]] / mat[[h, k]];

            mat[[i, k]] = Num::from(0);
            for j in k + 1..n {
                mat[[i, j]] = mat[[i, j]] - mat[[h, j]] * f;
            }
        }

        h += 1;
        k += 1;
    }

    // Second, calculate the reduced row echelon form.
    for h in 0..m {
        if let Some((k, scale)) = mat
            .row(h)
            .iter()
            .copied()
            .enumerate()
            .find(|(_, v)| *v != Num::from(0))
        {
            // Reduce first entry for each row to 1.
            for value in mat.row_mut(h).iter_mut() {
                *value /= scale;
            }

            // Reduce column above 1 to be all zeroes.
            for i in 0..h {
                let f = mat[[i, k]];
                for j in 0..n {
                    mat[[i, j]] = mat[[i, j]] - mat[[h, j]] * f;
                }
            }
        }
    }
}
