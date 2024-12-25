use crate::helper::prelude::*;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let orig = Point { x: 0, y: 0 };

    const HEIGHT: usize = 7;

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for g in input.split("\n\n") {
        let grid = g.parse::<Grid<char>>().unwrap();

        if grid[orig] == '#' {
            // Lock
            locks.push(
                grid.iter_cols()
                    .map(|c| c.filter(|&&c| c == '#').count() - 1)
                    .collect_vec(),
            );
        } else {
            // Key
            keys.push(
                grid.iter_cols()
                    .map(|c| HEIGHT - c.filter(|&&c| c == '.').count() - 1)
                    .collect_vec(),
            );
        }
    }

    locks
        .iter()
        .cartesian_product(keys.iter())
        .filter(|(lock, key)| lock.iter().zip(key.iter()).all(|(l, k)| l + k < HEIGHT - 1))
        .count()
}

#[test]
fn test() {
    let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
    let output = 3;
    assert_eq!(part_1(input).to_string(), output.to_string());
}

pub fn part_2(_input: &str) -> impl std::fmt::Display {
    "Click the button!"
}
