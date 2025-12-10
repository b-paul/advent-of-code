//! Random maths functions

/// Perform the extended euclidean algorithm
pub fn gcdex(a: i32, b: i32) -> (i32, i32, i32, i32, i32) {
    // code from https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode lol

    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    }

    (old_r, old_s, old_t, s, t)
}
