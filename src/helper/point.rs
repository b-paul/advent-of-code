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
pub fn manhattan_u((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize  {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

pub fn manhattan_i((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

// euclidean maybe
