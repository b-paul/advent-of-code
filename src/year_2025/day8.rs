use crate::helper::prelude::*;
use itertools::Itertools;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    const CONNECTIONS: usize = 1000;

    let points = input
        .lines()
        .map(|s| s.split(',').map(p::<u64>).collect_vec())
        .collect_vec();

    let mut uf = UnionFind::new(points.iter());

    for (a, b) in points
        .iter()
        .enumerate()
        .flat_map(|(i, p)| points[i + 1..].iter().map(move |p2| (p, p2)))
        .sorted_by_key(|(a, b)| {
            a.iter()
                .zip(b.iter())
                .map(|(&x, &y)| x.abs_diff(y).pow(2))
                .sum::<u64>()
        })
        .take(CONNECTIONS)
    {
        uf.join(a, b);
    }

    uf.sizes().sorted().rev().take(3).product::<usize>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let points = input
        .lines()
        .map(|s| s.split(',').map(p::<u64>).collect_vec())
        .collect_vec();

    let mut uf = UnionFind::new(points.iter());

    for (a, b) in points
        .iter()
        .enumerate()
        .flat_map(|(i, p)| points[i + 1..].iter().map(move |p2| (p, p2)))
        .sorted_by_key(|(a, b)| {
            a.iter()
                .zip(b.iter())
                .map(|(&x, &y)| x.abs_diff(y).pow(2))
                .sum::<u64>()
        })
    {
        uf.join(a, b);

        if uf.len() == 1 {
            return a[0] * b[0];
        }
    }

    unreachable!()
}
