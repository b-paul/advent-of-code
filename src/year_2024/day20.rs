use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn solve<const LEN: isize>(input: &str) -> usize {
    let grid = input.parse::<Grid<char>>().unwrap();
    // TODO this should be a Grid<Option<usize>> I reckon
    let mut time_grid = Grid::new_filled(i64::MAX, grid.bounds());

    let end = grid.find(&'E').unwrap();

    grid.bfs_4(end, |p, _, d| time_grid[p] = d as i64, |_, _, t| t != '#');

    grid.iter_idx()
        .filter_map(|(p, &c)| (c != '#').then_some(p))
        .map(|sq| {
            let mut n = 0;
            for dx in -LEN..=LEN {
                let rem = LEN - dx.abs();
                for dy in -rem..=rem {
                    let Some(p) = sq.move_off_bounded(Offset { dx, dy }, grid.bounds()) else {
                        continue;
                    };
                    let d = dx.abs() + dy.abs();
                    if grid[p] != '#' && time_grid[sq] - time_grid[p] - d as i64 >= 100 {
                        n += 1;
                    }
                }
            }
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
