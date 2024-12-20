use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn solve<const LEN: usize>(input: &str) -> usize {
    let grid = input.parse::<Grid<char>>().unwrap();
    // TODO this should be a Grid<Option<usize>> I reckon
    let mut time_grid = grid.clone().map(|_| i64::MAX);

    let end = grid.find(&'E').unwrap();

    grid.bfs_4(
        end,
        |p, _, d| time_grid[p] = d as i64,
        |_, _, t| t != '#',
    );

    grid.iter_idx()
        .filter_map(|(p, &c)| (c != '#').then_some(p))
        .map(|sq| {
            let mut n = 0;
            grid.bfs_4_depth_limit(
                sq,
                |p, t, d| {
                    if t != '#' && time_grid[sq] - time_grid[p] - d as i64 >= 100 {
                        n += 1;
                    }
                },
                |_, _, _| true,
                LEN,
            );
            n
        })
        .sum()
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    solve::<2>(input)
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    solve::<20>(input)
}
