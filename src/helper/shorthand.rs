//! Shorthand for code I've written a billion times during aoc

use std::fmt::Debug;
use std::str::FromStr;

use crate::helper::{adjacency::Direction4, point::Point};

pub fn p<T>(x: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    x.parse().unwrap()
}

pub fn read_dir(c: char) -> Option<Direction4> {
    match c {
        '^' => Some(Direction4::Up),
        '>' => Some(Direction4::Right),
        'v' => Some(Direction4::Down),
        '<' => Some(Direction4::Left),
        _ => None,
    }
}

/// Read a point that looks something like "10,12" (so two numbers seperated by a comma)
pub fn read_point(s: &str) -> Point {
    let nums = s.split(',').map(p::<usize>).collect::<Vec<_>>();
    Point {
        x: nums[0],
        y: nums[1],
    }
}
