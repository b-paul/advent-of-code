use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();
    let bounds = grid.bounds();

    let mut pqueue = BinaryHeap::new();
    let mut set = HashSet::new();

    // Distance, (x, y), dir, dircount
    pqueue.push((0i32, Point{x:1, y:0}, Direction4::Right, 1));
    pqueue.push((0i32, Point{x:0, y:1}, Direction4::Down, 1));

    while let Some((dist, p, dir, dircount)) = pqueue.pop() {
        let (x, y) = p.pair();
        if set.contains(&(p, dir, dircount)) {
            continue;
        }
        set.insert((p, dir, dircount));
        let dist = dist - grid[p].to_digit(10).unwrap() as i32;
        if x == grid.width() - 1 && y == grid.height() - 1 {
            return -dist;
        }
        for (_, dir2) in DIRECTIONS4D {
            if dir2 == dir.opposite() {
                continue;
            }
            if dircount >= 3 && dir == dir2 {
                continue;
            }
            let Some(p) = dir2.moveub(p, bounds) else {
                continue;
            };
            let dircount = if dir == dir2 { dircount + 1 } else { 1 };

            pqueue.push((dist, p, dir2, dircount));
        }
    }

    panic!("Path not found")
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();
    let bounds = grid.bounds();

    let mut pqueue = BinaryHeap::new();
    let mut set = HashSet::new();

    // Distance, (x, y), dir, dircount
    pqueue.push((0i32, Point{x:1, y:0}, Direction4::Right, 1));
    pqueue.push((0i32, Point{x:0, y:1}, Direction4::Down, 1));

    while let Some((dist, p, dir, dircount)) = pqueue.pop() {
        let (x, y) = p.pair();
        if set.contains(&(p, dir, dircount)) {
            continue;
        }
        set.insert((p, dir, dircount));
        let dist = dist - grid[(x, y)].to_digit(10).unwrap() as i32;
        if x == grid.width() - 1 && y == grid.height() - 1 && dircount >= 4 {
            return -dist;
        }
        for (_, dir2) in DIRECTIONS4D {
            if dir2 == dir.opposite() {
                continue;
            }

            if dircount < 4 && dir != dir2 {
                continue;
            }
            if dircount >= 10 && dir == dir2 {
                continue;
            }
            let Some(p) = dir2.moveub(p, bounds) else {
                continue;
            };
            let dircount = if dir == dir2 { dircount + 1 } else { 1 };

            pqueue.push((dist, p, dir2, dircount));
        }
    }

    panic!("Path not found")
}

#[cfg(test)]
mod benches {
    use crate::get_input;
    use crate::year_2023::day17::*;
    use test::{black_box, Bencher};

    #[bench]
    fn part1_normal(b: &mut Bencher) {
        let input = &get_input(2023, 17).unwrap();
        b.iter(|| {
            black_box(part_1(input));
        })
    }

    #[bench]
    fn part2_normal(b: &mut Bencher) {
        let input = &get_input(2023, 17).unwrap();
        b.iter(|| {
            black_box(part_2(input));
        })
    }
}
