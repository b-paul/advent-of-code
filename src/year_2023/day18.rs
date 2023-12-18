use std::collections::*;
use itertools::Itertools;
use crate::helper::prelude::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut cur = (0, 0);
    let mut area = 0;

    for line in input.lines() {
        let is = line.split(' ').collect_vec();
        let (dir, count, _) = (is[0], is[1].parse::<isize>().unwrap(), is[2]);
        let dir = match dir.chars().next().unwrap() {
            'R' => Direction4::Right,
            'L' => Direction4::Left,
            'U' => Direction4::Up,
            'D' => Direction4::Down,
            d => panic!("Invalid direction {d}"),
        };

        let next = dir.moveic(cur, count);

        // shoelace formula
        area += cur.0 * next.1 - next.0 * cur.1;

        // Add the edges
        area += cur.0.abs_diff(next.0) as isize;
        area += cur.1.abs_diff(next.1) as isize;

        cur = next;
    }

    // abs diff misses one corner, but then the edges get joined together so only one corner is
    // missed overall (i think ?!?!)
    area / 2 + 1
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut cur = (0, 0);
    let mut area = 0;

    for line in input.lines() {
        let is = line.split(' ').collect_vec();
        let (_, _, col) = (is[0], is[1].parse::<usize>().unwrap(), is[2]);
        let dir = &col[7..8];
        let dir = match dir.chars().next().unwrap() {
            '0' => Direction4::Right,
            '2' => Direction4::Left,
            '3' => Direction4::Up,
            '1' => Direction4::Down,
            d => panic!("Invalid direction {d}"),
        };
        let mut count = 0;
        let col = &col[2..7];
        for c in col.chars() {
            count *= 16;
            count += c.to_digit(16).unwrap() as isize;
        }

        let next = dir.moveic(cur, count);

        // shoelace formula
        area += cur.0 * next.1 - next.0 * cur.1;

        // Add the edges
        area += cur.0.abs_diff(next.0) as isize;
        area += cur.1.abs_diff(next.1) as isize;

        cur = next;
    }

    // abs diff misses one corner, but then the edges get joined together so only one corner is
    // missed overall (i think ?!?!)
    area / 2 + 1
}

#[cfg(test)]
mod benches {
    use crate::get_input;
    use crate::year_2023::day18::*;
    use test::{black_box, Bencher};

    #[bench]
    fn part1_normal(b: &mut Bencher) {
        let input = &get_input(2023, 18).unwrap();
        b.iter(|| {
            black_box(part_1(input));
        })
    }

    #[bench]
    fn part2_normal(b: &mut Bencher) {
        let input = &get_input(2023, 18).unwrap();
        b.iter(|| {
            black_box(part_2(input));
        })
    }
}
