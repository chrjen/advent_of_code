use std::fmt::Display;

use num::{rational::Ratio, Zero};

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
