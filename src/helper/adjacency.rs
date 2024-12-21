use crate::helper::point::{Bounds, Offset, Point};
// TODO utils to manipulate directions
// TODO account for isize -> usize overflow maybe????
// TODO neg for opposite

/// Trait for a direction. The purpose of this trait is to avoid having to implement the various
/// move functions multiple times.
pub trait Direction: Copy {
    /// The dirrection opposite to this one.
    fn opposite(self) -> Self;
    /// The direction a clockwise rotation from this one.
    fn cw(self) -> Self;
    /// The direction an anti-clockwise rotation from this one.
    fn acw(self) -> Self;
    /// The (x, y) offset this direction gives.
    fn offset(self) -> Offset;

    /* I would do this however it needs const_generic_exprs :(
    const COUNT: usize;
    const DIRS: [Self; Self::COUNT];
    */
    fn dir_list() -> Vec<Self>;

    /// Move the point in this direction, assuming the point is an isize.
    fn movei(self, (x, y): (isize, isize)) -> (isize, isize) {
        let (dx, dy) = self.offset().pair();
        (x + dx, y + dy)
    }

    /// Move the point in this direction c times, assuming the point is an isize.
    fn moveic(self, (x, y): (isize, isize), c: isize) -> (isize, isize) {
        let (dx, dy) = self.offset().pair();
        (x + dx * c, y + dy * c)
    }

    /// Move the point in this direction, assuming the point is an isize, within the bounds of
    /// bound_ul (giving the top left of the bounding box) and bound_dr (the bottom right of the
    /// bounding box).
    fn moveib(
        self,
        (x, y): (isize, isize),
        bound_ul: (isize, isize),
        bound_dr: (isize, isize),
    ) -> Option<(isize, isize)> {
        let (blx, bly) = bound_ul;
        let (brx, bry) = bound_dr;
        let (dx, dy) = self.offset().pair();
        let (x, y) = (x + dx, y + dy);
        if x < blx || x >= brx || y < bly || y >= bry {
            None
        } else {
            Some((x, y))
        }
    }

    /// Move the point in this direction, assuming the point is a usize, returning None if we
    /// underflow.
    fn moveu(self, point: Point) -> Option<Point> {
        point.move_off(self.offset())
    }

    /// Move the point in this direction c times, assuming the point is a usize, returning None if
    /// we underflow.
    fn moveuc(self, point: Point, c: usize) -> Option<Point> {
        point.move_off(self.offset().times(c))
    }

    /// Move the point in this direction, assuming the point is a usize, within the bounds given by
    /// bounds, which gives a bottom right bound (we assume that we are bounded in the top left by
    /// (0, 0)), returning None if we underflow.
    fn moveub(self, point: Point, bounds: Bounds) -> Option<Point> {
        let point = point.move_off(self.offset())?;
        bounds.contains_point(point).then_some(point)
    }

    // TODO
    // unsized bounded top left?
    // unsized with count?
    // bounds + counts?
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Direction4 {
    Up,
    Right,
    Down,
    Left,
}

impl From<Direction4> for usize {
    fn from(value: Direction4) -> Self {
        match value {
            Direction4::Up => 0,
            Direction4::Right => 1,
            Direction4::Down => 2,
            Direction4::Left => 3,
        }
    }
}

impl Direction for Direction4 {
    fn opposite(self) -> Direction4 {
        match self {
            Direction4::Up => Direction4::Down,
            Direction4::Right => Direction4::Left,
            Direction4::Down => Direction4::Up,
            Direction4::Left => Direction4::Right,
        }
    }

    fn cw(self) -> Direction4 {
        match self {
            Direction4::Up => Direction4::Right,
            Direction4::Right => Direction4::Down,
            Direction4::Down => Direction4::Left,
            Direction4::Left => Direction4::Up,
        }
    }

    fn acw(self) -> Direction4 {
        match self {
            Direction4::Up => Direction4::Left,
            Direction4::Right => Direction4::Up,
            Direction4::Down => Direction4::Right,
            Direction4::Left => Direction4::Down,
        }
    }

    fn offset(self) -> Offset {
        match self {
            Direction4::Up => Offset { dx: 0, dy: -1 },
            Direction4::Right => Offset { dx: 1, dy: 0 },
            Direction4::Down => Offset { dx: 0, dy: 1 },
            Direction4::Left => Offset { dx: -1, dy: 0 },
        }
    }

    fn dir_list() -> Vec<Direction4> {
        vec![
            Direction4::Up,
            Direction4::Right,
            Direction4::Down,
            Direction4::Left,
        ]
    }
}

impl Direction4 {
    /// Get the rotation from self to other
    pub fn rotation_between(self, other: Self) -> Rotation4 {
        match (self, other) {
            (Direction4::Up, Direction4::Up) => Rotation4::None,
            (Direction4::Up, Direction4::Right) => Rotation4::Clockwise,
            (Direction4::Up, Direction4::Down) => Rotation4::Double,
            (Direction4::Up, Direction4::Left) => Rotation4::Anticlockwise,
            (Direction4::Right, Direction4::Up) => Rotation4::Anticlockwise,
            (Direction4::Right, Direction4::Right) => Rotation4::None,
            (Direction4::Right, Direction4::Down) => Rotation4::Clockwise,
            (Direction4::Right, Direction4::Left) => Rotation4::Double,
            (Direction4::Down, Direction4::Up) => Rotation4::Double,
            (Direction4::Down, Direction4::Right) => Rotation4::Anticlockwise,
            (Direction4::Down, Direction4::Down) => Rotation4::None,
            (Direction4::Down, Direction4::Left) => Rotation4::Clockwise,
            (Direction4::Left, Direction4::Up) => Rotation4::Clockwise,
            (Direction4::Left, Direction4::Right) => Rotation4::Double,
            (Direction4::Left, Direction4::Down) => Rotation4::Anticlockwise,
            (Direction4::Left, Direction4::Left) => Rotation4::None,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum DirectionDiag4 {
    UpLeft,
    UpRight,
    DownRight,
    DownLeft,
}

impl Direction for DirectionDiag4 {
    fn opposite(self) -> DirectionDiag4 {
        match self {
            DirectionDiag4::UpLeft => DirectionDiag4::DownRight,
            DirectionDiag4::UpRight => DirectionDiag4::DownLeft,
            DirectionDiag4::DownRight => DirectionDiag4::UpLeft,
            DirectionDiag4::DownLeft => DirectionDiag4::UpRight,
        }
    }

    fn cw(self) -> DirectionDiag4 {
        match self {
            DirectionDiag4::UpLeft => DirectionDiag4::UpRight,
            DirectionDiag4::UpRight => DirectionDiag4::DownRight,
            DirectionDiag4::DownRight => DirectionDiag4::DownLeft,
            DirectionDiag4::DownLeft => DirectionDiag4::UpLeft,
        }
    }

    fn acw(self) -> DirectionDiag4 {
        match self {
            DirectionDiag4::UpLeft => DirectionDiag4::DownLeft,
            DirectionDiag4::UpRight => DirectionDiag4::UpLeft,
            DirectionDiag4::DownRight => DirectionDiag4::UpRight,
            DirectionDiag4::DownLeft => DirectionDiag4::DownRight,
        }
    }

    fn offset(self) -> Offset {
        match self {
            DirectionDiag4::UpLeft => Offset { dx: -1, dy: -1 },
            DirectionDiag4::UpRight => Offset { dx: -1, dy: 1 },
            DirectionDiag4::DownRight => Offset { dx: 1, dy: 1 },
            DirectionDiag4::DownLeft => Offset { dx: 1, dy: -1 },
        }
    }

    fn dir_list() -> Vec<Self> {
        vec![
            DirectionDiag4::UpLeft,
            DirectionDiag4::UpRight,
            DirectionDiag4::DownRight,
            DirectionDiag4::DownLeft,
        ]
    }
}

impl Direction4 {
    pub fn char(self) -> char {
        match self {
            Direction4::Up => '^',
            Direction4::Right => '>',
            Direction4::Down => 'v',
            Direction4::Left => '<',
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Direction8 {
    UpLeft,
    Up,
    UpRight,
    Left,
    Right,
    DownLeft,
    Down,
    DownRight,
}

impl Direction for Direction8 {
    fn opposite(self) -> Self {
        match self {
            Direction8::UpLeft => Direction8::DownRight,
            Direction8::Up => Direction8::Down,
            Direction8::UpRight => Direction8::DownLeft,
            Direction8::Right => Direction8::Left,
            Direction8::Left => Direction8::Right,
            Direction8::DownLeft => Direction8::UpRight,
            Direction8::Down => Direction8::Up,
            Direction8::DownRight => Direction8::UpLeft,
        }
    }

    fn cw(self) -> Self {
        match self {
            Direction8::UpLeft => Direction8::UpRight,
            Direction8::Up => Direction8::Right,
            Direction8::UpRight => Direction8::DownRight,
            Direction8::Right => Direction8::Down,
            Direction8::Left => Direction8::Up,
            Direction8::DownLeft => Direction8::UpLeft,
            Direction8::Down => Direction8::Left,
            Direction8::DownRight => Direction8::DownLeft,
        }
    }

    fn acw(self) -> Self {
        match self {
            Direction8::UpLeft => Direction8::DownLeft,
            Direction8::Up => Direction8::Left,
            Direction8::UpRight => Direction8::UpLeft,
            Direction8::Right => Direction8::Up,
            Direction8::Left => Direction8::Down,
            Direction8::DownLeft => Direction8::DownRight,
            Direction8::Down => Direction8::Right,
            Direction8::DownRight => Direction8::UpRight,
        }
    }

    fn offset(self) -> Offset {
        match self {
            Direction8::UpLeft => Offset { dx: -1, dy: -1 },
            Direction8::Up => Offset { dx: 0, dy: -1 },
            Direction8::UpRight => Offset { dx: -1, dy: 1 },
            Direction8::Right => Offset { dx: 1, dy: 0 },
            Direction8::Left => Offset { dx: -1, dy: 0 },
            Direction8::DownLeft => Offset { dx: 1, dy: -1 },
            Direction8::Down => Offset { dx: 0, dy: 1 },
            Direction8::DownRight => Offset { dx: 1, dy: 1 },
        }
    }

    fn dir_list() -> Vec<Self> {
        vec![
            Direction8::UpLeft,
            Direction8::Up,
            Direction8::UpRight,
            Direction8::Right,
            Direction8::Left,
            Direction8::DownLeft,
            Direction8::Down,
            Direction8::DownRight,
        ]
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Rotation4 {
    None,
    Clockwise,
    Double,
    Anticlockwise,
}

impl Rotation4 {
    /// The rotation opposite to this one.
    pub fn opposite(self) -> Self {
        match self {
            Rotation4::None => Rotation4::Double,
            Rotation4::Clockwise => Rotation4::Anticlockwise,
            Rotation4::Double => Rotation4::None,
            Rotation4::Anticlockwise => Rotation4::Clockwise,
        }
    }

    /// The rotation a clockwise rotation from this one.
    pub fn cw(self) -> Self {
        match self {
            Rotation4::None => Rotation4::Clockwise,
            Rotation4::Clockwise => Rotation4::Double,
            Rotation4::Double => Rotation4::Anticlockwise,
            Rotation4::Anticlockwise => Rotation4::None,
        }
    }

    /// The rotation an anti-clockwise rotation from this one.
    pub fn acw(self) -> Self {
        match self {
            Rotation4::None => Rotation4::Anticlockwise,
            Rotation4::Clockwise => Rotation4::None,
            Rotation4::Double => Rotation4::Clockwise,
            Rotation4::Anticlockwise => Rotation4::Double,
        }
    }

    /// A list of the possible rotations.
    pub fn rot_list() -> Vec<Rotation4> {
        vec![
            Rotation4::None,
            Rotation4::Clockwise,
            Rotation4::Double,
            Rotation4::Anticlockwise,
        ]
    }

    /// Flip the rotation about the None/Clockwise axis
    pub fn flip(self) -> Rotation4 {
        match self {
            Rotation4::None => Rotation4::None,
            Rotation4::Clockwise => Rotation4::Anticlockwise,
            Rotation4::Double => Rotation4::Double,
            Rotation4::Anticlockwise => Rotation4::Clockwise,
        }
    }
}

impl From<Rotation4> for usize {
    fn from(value: Rotation4) -> Self {
        match value {
            Rotation4::None => 0,
            Rotation4::Clockwise => 1,
            Rotation4::Double => 2,
            Rotation4::Anticlockwise => 3,
        }
    }
}

// TODO get rid of these or something
pub const DIRECTIONS4: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
pub const DIRECTIONS4D: [((isize, isize), Direction4); 4] = [
    ((-1, 0), Direction4::Left),
    ((0, -1), Direction4::Up),
    ((0, 1), Direction4::Down),
    ((1, 0), Direction4::Right),
];
pub const DIRECTIONS8: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
pub const DIRECTIONS8D: [((isize, isize), Direction8); 8] = [
    ((-1, -1), Direction8::UpLeft),
    ((-1, 0), Direction8::Up),
    ((-1, 1), Direction8::UpRight),
    ((0, -1), Direction8::Left),
    ((0, 1), Direction8::Right),
    ((1, -1), Direction8::DownLeft),
    ((1, 0), Direction8::Down),
    ((1, 1), Direction8::DownRight),
];

// TODO replace these methods with associated methods in the Direction trait

// Iterates in order:
// Left Up Down Right
pub fn adjacent_4_i(x: isize, y: isize) -> Vec<(isize, isize)> {
    DIRECTIONS4
        .iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .collect()
}

// Iterates in order:
// Left Up Down Right
pub fn adjacent_4_u(x: usize, y: usize) -> Vec<Point> {
    DIRECTIONS4
        .iter()
        .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
        .filter(|(x, y)| *x >= 0 && *y >= 0)
        .map(|(x, y)| (x as usize, y as usize))
        .map(|(x, y)| Point { x, y })
        .collect()
}

// Iterates in order:
// Left Up Down Right
pub fn adjacent_4_ud(x: usize, y: usize) -> Vec<(Point, Direction4)> {
    DIRECTIONS4D
        .iter()
        .map(|((dx, dy), dir)| ((x as isize + dx, y as isize + dy), dir))
        .filter(|((x, y), _)| *x >= 0 && *y >= 0)
        .map(|((x, y), dir)| ((x as usize, y as usize), *dir))
        .map(|((x, y), dir)| (Point { x, y }, dir))
        .collect()
}

// Iterates in order:
// UpLeft Up UpRight Left Right DownLeft Down DownRight
pub fn adjacent_8_i(x: isize, y: isize) -> Vec<(isize, isize)> {
    DIRECTIONS8
        .iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .collect()
}

// Iterates in order:
// UpLeft Up UpRight Left Right DownLeft Down DownRight
pub fn adjacent_8_u(x: usize, y: usize) -> Vec<Point> {
    DIRECTIONS8
        .iter()
        .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
        .filter(|(x, y)| *x >= 0 && *y >= 0)
        .map(|(x, y)| (x as usize, y as usize))
        .map(|(x, y)| Point { x, y })
        .collect()
}

// Iterates in order:
// UpLeft Up UpRight Left Right DownLeft Down DownRight
pub fn adjacent_8_ud(x: usize, y: usize) -> Vec<(Point, Direction8)> {
    DIRECTIONS8D
        .iter()
        .map(|((dx, dy), dir)| ((x as isize + dx, y as isize + dy), dir))
        .filter(|((x, y), _)| *x >= 0 && *y >= 0)
        .map(|((x, y), dir)| ((x as usize, y as usize), *dir))
        .map(|((x, y), dir)| (Point { x, y }, dir))
        .collect()
}
