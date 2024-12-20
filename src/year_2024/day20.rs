use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();

    let mut score = i64::MAX;
    let start = grid.find(&'S').unwrap();

    grid.bfs_4(
        start,
        |_, t, d| {
            if t == 'E' {
                score = score.min(d as i64);
            }
        },
        |_, _, t| t != '#',
    );

    let mut total = 0;

    for (sq, c) in grid.iter_idx() {
        if *c != '#' {
            continue;
        }
        let mut grid = grid.clone();
        let mut score2 = i64::MAX;
        grid[sq] = '.';
        grid.bfs_4(
            start,
            |_, t, d| {
                if t == 'E' {
                    score2 = score2.min(d as i64);
                }
            },
            |_, _, t| t != '#',
        );
        if score - score2 >= 100 {
            total += 1;
        }
    }

    total
}

#[test]
fn test() {
    let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    let output = 16;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();
    let mut time_grid = grid.clone().map(|_| i64::MAX);

    let mut score = i64::MAX;
    let start = grid.find(&'S').unwrap();

    grid.bfs_4(
        start,
        |p, t, d| {
            if t == 'E' {
                score = score.min(d as i64);
            }
            time_grid[p] = time_grid[p].min(d as i64);
        },
        |_, _, t| t != '#',
    );

    let mut total = 0;

    for (sq, _) in grid.iter_idx().filter(|(_, &c)| c != '#') {
        let mut ends = HashSet::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((sq, 0));
        visited.insert(sq);

        while let Some((from, depth)) = queue.pop_front() {
            if depth > 20 {
                continue;
            }
            if grid[from] != '#'
                && time_grid[from] as i64 - time_grid[sq] as i64 - depth as i64 >= 100
            {
                ends.insert(from);
            }
            for (to, _) in adjacent_4_ud(from.x, from.y) {
                if grid.contains_point(to) && !visited.contains(&to) {
                    queue.push_back((to, depth + 1));
                    visited.insert(to);
                }
            }
        }

        total += ends.len();
    }

    total
}
