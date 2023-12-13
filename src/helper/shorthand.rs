//! Shorthand for code I've written a billion times during aoc

use std::fmt::Debug;
use std::str::FromStr;

pub fn p<T>(x: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    x.parse().unwrap()
}
