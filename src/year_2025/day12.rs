use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input
        .split("\n\n")
        .nth(6)
        .unwrap()
        .lines()
        .filter(|&s| {
            let (dim, ns) = s.split_once(": ").unwrap();
            let (width, height) = dim.split_once('x').unwrap();
            let (width, height) = (p::<usize>(width), p::<usize>(height));
            let ns = ns.split_whitespace().map(p::<usize>).collect_vec();

            // If we partition the overall grid into 3x3 squares and we have more of such squares than
            // the number of presents to place, we don't need to check anything!
            if ns.iter().sum::<usize>() <= (width / 3) * (height / 3) {
                return true;
            }

            // THAT'S SUFFICIENT?????
            false
        })
        .count()
}

pub fn part_2(_input: &str) -> impl std::fmt::Display {
    "no part 2 today!"
}
