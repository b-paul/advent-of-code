#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Direction4 {
    Left,
    Up,
    Down,
    Right,
}

impl Direction4 {
    pub fn opposite(self) -> Direction4 {
        match self {
            Direction4::Left => Direction4::Right,
            Direction4::Up => Direction4::Down,
            Direction4::Down => Direction4::Up,
            Direction4::Right => Direction4::Left,
        }
    }
}

impl From<Direction4> for usize {
    fn from(value: Direction4) -> Self {
        match value {
            Direction4::Left => 0,
            Direction4::Up => 1,
            Direction4::Down => 2,
            Direction4::Right => 3,
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
pub fn adjacent_4_u(x: usize, y: usize) -> Vec<(usize, usize)> {
    DIRECTIONS4
        .iter()
        .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
        .filter(|(x, y)| *x >= 0 && *y >= 0)
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}

// Iterates in order:
// Left Up Down Right
pub fn adjacent_4_ud(x: usize, y: usize) -> Vec<((usize, usize), Direction4)> {
    DIRECTIONS4D
        .iter()
        .map(|((dx, dy), dir)| ((x as isize + dx, y as isize + dy), dir))
        .filter(|((x, y), _)| *x >= 0 && *y >= 0)
        .map(|((x, y), dir)| ((x as usize, y as usize), *dir))
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
pub fn adjacent_8_u(x: usize, y: usize) -> Vec<(usize, usize)> {
    DIRECTIONS8
        .iter()
        .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
        .filter(|(x, y)| *x >= 0 && *y >= 0)
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}

// Iterates in order:
// UpLeft Up UpRight Left Right DownLeft Down DownRight
pub fn adjacent_8_ud(x: usize, y: usize) -> Vec<((usize, usize), Direction8)> {
    DIRECTIONS8D
        .iter()
        .map(|((dx, dy), dir)| ((x as isize + dx, y as isize + dy), dir))
        .filter(|((x, y), _)| *x >= 0 && *y >= 0)
        .map(|((x, y), dir)| ((x as usize, y as usize), *dir))
        .collect()
}
