use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();

    let mut antinodes = grid.clone().map(|_| false);

    for (a, b) in grid
        .iter_idx()
        .cartesian_product(grid.iter_idx())
        .filter_map(|((posa, c1), (posb, c2))| {
            (posa != posb && c1.is_alphanumeric() && c1 == c2).then_some((posa, posb))
        })
    {
        let d = rel_off(a, b);
        // We do the other direction when doing the (b, a) step
        if let Some(pa) = move_off(a, d) {
            if antinodes.contains_point(pa) {
                antinodes[pa] = true;
            }
        }
    }

    println!("{}", antinodes.clone().map(|b| match b {
        true => '#',
        false => '.',
    }));

    antinodes.iter_idx().filter(|(_, &b)| b).count()
}

#[test]
fn test() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    let output = 34;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();

    let mut antinodes = grid.clone().map(|_| false);

    for (a, b) in grid
        .iter_idx()
        .cartesian_product(grid.iter_idx())
        .filter_map(|((posa, c1), (posb, c2))| {
            (posa != posb && c1.is_alphanumeric() && c1 == c2).then_some((posa, posb))
        })
    {
        let d = rel_off(a, b);
        let mut p = a;
        while antinodes.contains_point(p) {
            antinodes[p] = true;
            p = match move_off(p, d) {
                Some(p) => p,
                None => break,
            };
        }
    }

    println!("{}", antinodes.clone().map(|b| match b {
        true => '#',
        false => '.',
    }));

    antinodes.iter_idx().filter(|(_, &b)| b).count()
}
