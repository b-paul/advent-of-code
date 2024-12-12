use std::ops::{Add, Neg};

// TODO
// Multiply points (see day 10 line 99)
// Rotate points relative to a grid / to some bounds

// TODO perhaps calling a 2d point a point hurts generalisations to other dimensions
/// A point in a bounded 2d space. Points will always have non-negative x and y coordinates.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

/// A 2d offset.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Offset {
    pub dx: isize,
    pub dy: isize,
}

/// A bound for 2d space. Together this gets paired with the bound (0, 0) to make a full bounding
/// box.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Bound {
    pub width: usize,
    pub height: usize,
}

impl Point {
    /// Convert a point into an (x, y) pair.
    pub fn pair(self) -> (usize, usize) {
        (self.x, self.y)
    }

    /// Move a point by some offset.
    pub fn move_off(self, off: Offset) -> Option<Point> {
        let x = self.x as isize + off.dx;
        let y = self.y as isize + off.dy;
        (x >= 0 && y >= 0).then_some(Point {
            x: x as usize,
            y: y as usize,
        })
    }

    /// Returns the relative offset from this point to another point.
    pub fn rel_off(self, other: Point) -> Offset {
        Offset {
            dx: self.x as isize - other.x as isize,
            dy: self.y as isize - other.y as isize
        }
    }
}

impl Offset {
    /// Convert an offset into a (dx, dy) pair.
    pub fn pair(self) -> (isize, isize) {
        (self.dx, self.dy)
    }

    /// Get the offset in the opposite direction to this one. Equivalently, rotate this offset by
    /// 180 degrees.
    pub fn reverse(self) -> Offset {
        Offset {
            dx: -self.dx,
            dy: -self.dy,
        }
    }

    /// Multiply the offset by the specified amount.
    pub fn times(self, amount: usize) -> Offset {
        Offset {
            dx: self.dx * amount as isize,
            dy: self.dy * amount as isize,
        }
    }
}

impl Bound {
    /// Convert an offset into a (width, height) pair.
    pub fn pair(self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Determine whether a given point sits inside this bounding box.
    pub fn contains_point(&self, p: Point) -> bool {
        p.x < self.width && p.y < self.height
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Neg for Offset {
    type Output = Offset;

    fn neg(self) -> Self::Output {
        self.reverse()
    }
}

// TODO maybe move this old stuff

/// Get the left point out of two points, working on usize points
pub fn left_pu((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> (usize, usize) {
    let xl = x1.min(x2);
    let yl = y1.min(y2);
    (xl, yl)
}

/// Get the left point out of two points, working on isize points
pub fn left_pi((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> (isize, isize) {
    let xl = x1.min(x2);
    let yl = y1.min(y2);
    (xl, yl)
}

/// Get the right point out of two points, working on usize points
pub fn right_pu((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> (usize, usize) {
    let xr = x1.max(x2);
    let yr = y1.max(y2);
    (xr, yr)
}

/// Get the right point out of two points, working on isize points
pub fn right_pi((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> (isize, isize) {
    let xr = x1.max(x2);
    let yr = y1.max(y2);
    (xr, yr)
}

// distance functions
pub fn manhattan_u((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

pub fn manhattan_i((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

// euclidean maybe
