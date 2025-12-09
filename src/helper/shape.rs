//! Helper functions for working with shapes

use super::point::Point;
use std::ops::RangeInclusive;

/// A line going either directly vertical or directly horizontal
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct StraightLine(pub Point, pub Point);

impl StraightLine {
    /// Returns a line if the two points are either directly vertical or directly horizonal.
    pub fn from_points(a: Point, b: Point) -> Option<Self> {
        (a.x == b.x || a.y == b.y).then_some(StraightLine(a, b))
    }

    /// Determine whether this line is backwards meaning that the first point has a greater either
    /// x or y coordinate than the second.
    pub fn backwards(&self) -> bool {
        (self.vertical() && self.0.y > self.1.y) || (self.horizontal() && self.0.x > self.1.x)
    }

    /// Determine whether this line is vertical
    pub fn vertical(self) -> bool {
        self.0.x == self.1.x
    }

    /// Determine whether this line is horizontal
    pub fn horizontal(self) -> bool {
        self.0.y == self.1.y
    }

    /// The range of either x or y coordinates this line takes up if the line is horizontal or
    /// vertical respectively.
    pub fn point_range(self) -> RangeInclusive<usize> {
        if self.vertical() {
            let min = self.0.y.min(self.1.y);
            let max = self.0.y.max(self.1.y);
            min..=max
        } else {
            let min = self.0.x.min(self.1.x);
            let max = self.0.x.max(self.1.x);
            min..=max
        }
    }

    /// Determine whether this line intersects perpendicularly with another, and returns the point
    /// at which they intersect if they do.
    pub fn intersects_perp(self, other: StraightLine) -> Option<Point> {
        (self.vertical()
            && other.horizontal()
            && self.point_range().contains(&other.0.y)
            && other.point_range().contains(&self.0.x))
        .then_some(Point {
            x: self.0.x,
            y: other.0.y,
        })
        .or((self.horizontal()
            && other.vertical()
            && self.point_range().contains(&other.0.x)
            && other.point_range().contains(&self.0.y))
        .then_some(Point {
            x: other.0.x,
            y: self.0.y,
        }))
    }

    /// Get all points in this line
    pub fn points(&self) -> Vec<Point> {
        if self.vertical() {
            self.point_range()
                .map(|y| Point { x: self.0.x, y })
                .collect()
        } else {
            self.point_range()
                .map(|x| Point { x, y: self.0.y })
                .collect()
        }
    }

    /// Remove all points from the given line from this line and return the resulting possibly
    /// multiple lines left.
    pub fn subtract_line(self, minus: StraightLine) -> Vec<StraightLine> {
        if self.intersects_perp(minus).is_none() {
            return vec![self];
        }
        let (r1, r2) = match (self.vertical(), minus.vertical()) {
            (true, true) | (false, false) => (self.point_range(), minus.point_range()),
            (true, false) => (self.point_range(), minus.0.y..=minus.0.y),
            (false, true) => (self.point_range(), minus.0.x..=minus.0.x),
        };
        match (*r2.start() <= *r1.start(), *r1.end() <= *r2.end()) {
            (true, true) => vec![].into_iter(),
            (true, false) => vec![*r2.end() + 1..=*r1.end()].into_iter(),
            (false, true) => vec![*r1.start()..=*r2.start() - 1].into_iter(),
            (false, false) => vec![*r1.start()..=*r2.start() - 1, *r2.end() + 1..=*r1.start()].into_iter(),
        }
        .map(move |r: RangeInclusive<usize>| {
            if self.vertical() {
                StraightLine(
                    Point {
                        x: self.0.x,
                        y: *r.start(),
                    },
                    Point {
                        x: self.0.x,
                        y: *r.end(),
                    },
                )
            } else {
                StraightLine(
                    Point {
                        x: *r.start(),
                        y: self.0.y,
                    },
                    Point {
                        x: *r.end(),
                        y: self.0.y,
                    },
                )
            }
        }).collect()
    }
}

/// A rectangle formed by a top right and bottom left vertex in 2d space (with integer
/// coordinates).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Rect {
    pub ul: Point,
    pub dr: Point,
}

impl Rect {
    /// Form a rectangle from two opposite corners (they don't need to be the top left and bottom
    /// right necessarily, this is corrected for).
    pub fn new(a: Point, b: Point) -> Rect {
        let ul = Point {
            x: a.x.min(b.x),
            y: a.y.min(b.y),
        };
        let dr = Point {
            x: a.x.max(b.x),
            y: a.y.max(b.y),
        };
        Rect { ul, dr }
    }

    /// Compute the area of this rectangle.
    pub fn area(&self) -> usize {
        (self.dr.x - self.ul.x + 1) * (self.dr.y - self.ul.y + 1)
    }

    /// The up right point of this rectangle.
    pub fn ur(&self) -> Point {
        Point {
            x: self.dr.x,
            y: self.ul.y,
        }
    }

    /// The down left point of this rectangle.
    pub fn dl(&self) -> Point {
        Point {
            x: self.ul.x,
            y: self.dr.y,
        }
    }

    /// Returns the four bounding lines of this rectangle.
    pub fn boundary(&self) -> [StraightLine; 4] {
        [
            StraightLine(self.ul, self.ur()),
            StraightLine(self.ur(), self.dr),
            StraightLine(self.dr, self.dl()),
            StraightLine(self.dl(), self.ul),
        ]
    }

    /// Determine whether a point is contained inside this rectangle.
    pub fn contains_point(&self, p: Point) -> bool {
        (self.ul.x..=self.dr.x).contains(&p.x) && (self.ul.y..=self.dr.y).contains(&p.y)
    }

    /// Determine whether a line intersects this rectangle
    pub fn intersects_line(&self, l: StraightLine) -> bool {
        let [top, right, bottom, left] = self.boundary();
        l.vertical() && (top.intersects_perp(l).is_some() || bottom.intersects_perp(l).is_some())
            || l.horizontal()
                && (right.intersects_perp(l).is_some() || left.intersects_perp(l).is_some())
    }
}
