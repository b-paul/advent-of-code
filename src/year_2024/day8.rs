use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();

    let mut antinodes = grid.clone().map(|_| false);

    // Potential optimistions:
    // For each frequency, store a list of points on the grid that the frequency exists at.
    // This way, when we do this n^2 loop, n is much much smaller.

    for (a, b) in grid
        .iter_idx()
        .cartesian_product(grid.iter_idx())
        .filter_map(|((a, c1), (b, c2))| {
            (a != b && c1.is_alphanumeric() && c1 == c2).then_some((a, b))
        })
    {
        let d = a.rel_off(b);
        // We do the other direction when doing the (b, a) step
        if let Some(p) = a.move_off(d) {
            if antinodes.contains_point(p) {
                antinodes[p] = true;
            }
        }
    }

    println!(
        "{}",
        antinodes.clone().map(|b| match b {
            true => '#',
            false => '.',
        })
    );

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
        .filter_map(|((a, c1), (b, c2))| {
            (a != b && c1.is_alphanumeric() && c1 == c2).then_some((a, b))
        })
    {
        let d = a.rel_off(b);
        let mut p = a;
        while antinodes.contains_point(p) {
            antinodes[p] = true;
            p = match p.move_off(d) {
                Some(p) => p,
                None => break,
            };
        }
    }

    println!(
        "{}",
        antinodes.clone().map(|b| match b {
            true => '#',
            false => '.',
        })
    );

    antinodes.iter_idx().filter(|(_, &b)| b).count()
}
