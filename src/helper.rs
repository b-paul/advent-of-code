pub const DIRECTIONS4: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
pub const DIRECTIONS8: [(i32, i32); 8] =
    [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

// TODO make these take usizes
pub fn adjacent_4((x, y): (i32, i32)) -> [(i32, i32); 4] {
    DIRECTIONS4.map(|(dx, dy)| (x + dx, y + dy))
}

pub fn adjacent_8((x, y): (i32, i32)) -> [(i32, i32); 8] {
    DIRECTIONS8.map(|(dx, dy)| (x + dx, y + dy))
}
