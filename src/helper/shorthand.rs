//! Shorthand for code I've written a billion times during aoc

use std::fmt::Debug;
use std::str::FromStr;

use crate::helper::adjacency::Direction4;

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
