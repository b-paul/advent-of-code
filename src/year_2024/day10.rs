use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let grid = input
        .parse::<Grid<char>>()
        .unwrap()
        .map(|c| c.to_digit(10).unwrap());

    let mut ans = 0;

    for pos in grid
        .iter_idx()
        .filter_map(|(pos, &c)| (c == 0).then_some(pos))
    {
        grid.dfs_4(
            pos,
            |_, c| {
                if c == 9 {
                    ans += 1;
                }
            },
            |_, from, to| from + 1 == to,
        );
    }

    ans
}

#[test]
fn test() {
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    let output = 81;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let grid = input
        .parse::<Grid<char>>()
        .unwrap()
        .map(|c| c.to_digit(10).unwrap());

    let mut ans = 0;

    for pos in grid
        .iter_idx()
        .filter_map(|(pos, &c)| (c == 0).then_some(pos))
    {
        grid.dfs_4_dups(
            pos,
            |_, c| {
                if c == 9 {
                    ans += 1;
                }
            },
            |_, from, to| from + 1 == to,
        );
    }

    ans
}
