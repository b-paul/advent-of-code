pub const DIRECTIONS4: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
pub const DIRECTIONS8: [(isize, isize); 8] =
    [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

pub fn adjacent_4_i(x: isize, y: isize) -> Vec<(isize, isize)> {
    DIRECTIONS4
        .iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .collect()
}

pub fn adjacent_4_u(x: usize, y: usize) -> Vec<(usize, usize)> {
    DIRECTIONS4
        .iter()
        .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
        .filter(|(x, y)| *x >= 0 && *y >= 0)
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}

pub fn adjacent_8_i(x: isize, y: isize) -> Vec<(isize, isize)> {
    DIRECTIONS8
        .iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .collect()
}

pub fn adjacent_8_u(x: usize, y: usize) -> Vec<(usize, usize)> {
    DIRECTIONS8
        .iter()
        .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
        .filter(|(x, y)| *x >= 0 && *y >= 0)
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}
