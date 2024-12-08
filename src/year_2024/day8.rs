use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();

    let mut antinodes = grid.clone().map(|_| false);

    // TODO perhaps this kind of structure could be a helper with a FromIter impl

    let mut point_lists = HashMap::<_, Vec<_>>::new();

    for (p, v) in grid.iter_idx() {
        if v.is_alphanumeric() {
            point_lists.entry(*v).or_default().push(p);
        }
    }

    for v in point_lists.values() {
        for (&a, &b) in v.iter().cartesian_product(v.iter()).filter(|(a, b)| a != b) {
            let d = a.rel_off(b);
            // We do the other direction when doing the (b, a) step
            if let Some(p) = a.move_off(d) {
                if antinodes.contains_point(p) {
                    antinodes[p] = true;
                }
            }
        }
    }

    /*
    println!(
        "{}",
        antinodes.clone().map(|b| match b {
            true => '#',
            false => '.',
        })
    );
    */

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

    let mut point_lists = HashMap::<_, Vec<_>>::new();

    for (p, v) in grid.iter_idx() {
        if v.is_alphanumeric() {
            point_lists.entry(*v).or_default().push(p);
        }
    }

    for v in point_lists.values() {
        for (&a, &b) in v.iter().cartesian_product(v.iter()).filter(|(a, b)| a != b) {
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
    }

    /*
    println!(
        "{}",
        antinodes.clone().map(|b| match b {
            true => '#',
            false => '.',
        })
    );
    */

    antinodes.iter_idx().filter(|(_, &b)| b).count()
}

#[cfg(test)]
mod benches {
    use crate::get_input;
    use crate::year_2024::day8::*;
    use test::{black_box, Bencher};

    #[bench]
    fn part1_normal(b: &mut Bencher) {
        let input = &get_input(2024, 8).unwrap();
        b.iter(|| {
            black_box(part_1(input));
        })
    }

    #[bench]
    fn part2_normal(b: &mut Bencher) {
        let input = &get_input(2024, 8).unwrap();
        b.iter(|| {
            black_box(part_2(input));
        })
    }
}
