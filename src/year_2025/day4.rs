use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input
        .parse::<Grid<char>>()
        .unwrap()
        .iter_points()
        .filter(|&p| {
            *p.val() == '@' && p.adjs::<Direction8>().filter(|p| *p.val() == '@').count() < 4
        })
        .count()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    // TODO clean this up a lot !!!

    let grid = input.parse::<Grid<char>>().unwrap();
    let mut adj_count = grid.clone().map_i(|p, c| match c {
        '@' => grid
            .point(p)
            .unwrap()
            .adjs::<Direction8>()
            .filter(|p| *p.val() == '@')
            .count() as i32,
        _ => -1,
    });
    let mut stack = adj_count
        .iter_idx()
        .filter_map(|(p, n)| (0..4).contains(n).then_some(p))
        .collect::<Vec<_>>();
    let mut visited = stack.iter().copied().collect::<HashSet<_>>();
    let mut n = 0;

    while let Some(p) = stack.pop() {
        n += 1;
        for p2 in Direction8::dir_list()
            .into_iter()
            .flat_map(|d| p.move_dir_bounded(d, grid.bounds()))
        {
            adj_count[p2] -= 1;
            if adj_count[p2] == 3 && !visited.contains(&p2) {
                stack.push(p2);
                visited.insert(p2);
            }
        }
    }

    n
}
