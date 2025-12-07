use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();

    // we instead find the first splitter which is always below the starting point (i hope!)
    let start = grid.find(&'^').unwrap();

    let mut stack = Vec::new();
    let mut visited = HashSet::new();
    stack.push(grid.point(start).unwrap());
    visited.insert(start);

    while let Some(p) = stack.pop() {
        for p2 in [Direction4::Left, Direction4::Right]
            .into_iter()
            .flat_map(|d| p.move_dir(d))
        {
            if let Some(p3) = p2
                .trajectory_dir(Direction4::Down, 10000)
                .position(|&c| c == '^')
                .and_then(|off| p2.move_dirc(Direction4::Down, off))
            {
                if !visited.contains(&p3.pos()) {
                    visited.insert(p3.pos());
                    stack.push(p3);
                }
            }
        }
    }

    visited.len()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();

    let start = grid.find(&'S').unwrap();
    let mut dp = HashMap::<_, u64>::new();

    for p in grid
        .bounds()
        .iter_points()
        .rev()
        .filter_map(|p| (grid[p] != '.').then_some(grid.point(p)?))
    {
        dp.insert(p.pos(), 2);
        for p2 in [Direction4::Left, Direction4::Right]
            .into_iter()
            .flat_map(|d| p.move_dir(d))
        {
            if let Some(p3) = p2
                .trajectory_dir(Direction4::Down, 10000)
                .position(|&c| c == '^')
                .and_then(|off| p2.move_dirc(Direction4::Down, off))
            {
                *dp.entry(p.pos()).or_insert(0) += dp[&p3.pos()] - 1;
            }
        }
    }

    dp[&start]
}
