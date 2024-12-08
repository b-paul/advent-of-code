use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();

    let freqs = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890".chars();

    let mut antinodes = grid.clone().map(|_| false);

    for f in freqs {
        for posa in grid.iter_idx().filter_map(|(a, &c)| (c == f).then_some(a)) {
            for posb in grid.iter_idx().filter_map(|(a, &c)| (c == f).then_some(a)) {
                if posa.0 > posb.0 || (posa.0 == posb.0 && posa.1 >= posb.1) {
                    continue;
                }
                let (dx, dy) = (
                    posa.0 as isize - posb.0 as isize,
                    posa.1 as isize - posb.1 as isize,
                );
                let pa = antinodes
                    .point(posa)
                    .unwrap()
                    .move_off((dx, dy))
                    .map(|p| p.pos());
                let pb = antinodes
                    .point(posb)
                    .unwrap()
                    .move_off((-dx, -dy))
                    .map(|p| p.pos());
                if let Some(pa) = pa {
                    if antinodes.contains_point(pa) {
                        antinodes[pa] = true;
                    }
                }
                if let Some(pb) = pb {
                    if antinodes.contains_point(pb) {
                        antinodes[pb] = true;
                    }
                }
            }
        }
    }

    let print = antinodes.clone().map(|b| match b {
        true => '#',
        false => '.',
    });
    println!("{print}");

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

    let freqs = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890".chars();

    let mut antinodes = grid.clone().map(|_| false);

    for f in freqs {
        for posa in grid.iter_idx().filter_map(|(a, &c)| (c == f).then_some(a)) {
            for posb in grid.iter_idx().filter_map(|(a, &c)| (c == f).then_some(a)) {
                if posa.0 > posb.0 || (posa.0 == posb.0 && posa.1 >= posb.1) {
                    continue;
                }
                let (dx, dy) = (
                    posa.0 as isize - posb.0 as isize,
                    posa.1 as isize - posb.1 as isize,
                );
                let mut pa = posa;
                antinodes[pa] = true;
                while let Some(pan) = move_off(pa, (dx, dy)) {
                    pa = pan;
                    if antinodes.contains_point(pa) {
                        antinodes[pa] = true;
                    } else {
                        break
                    }
                }
                let mut pb = posb;
                antinodes[pb] = true;
                while let Some(pbn) = move_off(pb, (-dx, -dy)) {
                    pb = pbn;
                    if antinodes.contains_point(pb) {
                        antinodes[pb] = true;
                    } else {
                        break
                    }
                }
            }
        }
    }

    let print = antinodes.clone().map(|b| match b {
        true => '#',
        false => '.',
    });
    println!("{print}");

    antinodes.iter_idx().filter(|(_, &b)| b).count()
}
