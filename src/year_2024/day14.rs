use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let robots = input
        .lines()
        .map(|l| {
            let nums = l.split_whitespace().collect_vec();
            let (_, pos) = nums[0].split_once("p=").unwrap();
            let (_, v) = nums[1].split_once("v=").unwrap();
            let pos = pos.split(',').map(p::<usize>).collect_vec();
            let vel = v.split(',').map(p::<isize>).collect_vec();
            (
                Point {
                    x: pos[0],
                    y: pos[1],
                },
                Offset {
                    dx: vel[0],
                    dy: vel[1],
                },
            )
        })
        .collect_vec();
    let width = robots.iter().map(|(p, _)| p.x).max().unwrap() + 1;
    let height = robots.iter().map(|(p, _)| p.y).max().unwrap() + 1;
    let bounds = Bounds { width, height };

    let center = (width / 2, height / 2);

    let mut counts = [0, 0, 0, 0];

    for (p, v) in robots {
        let p2 = p.move_off_wrapping(v.times(100), bounds);
        if p2.x < center.0 && p2.y < center.1 {
            counts[0] += 1;
        }
        if p2.x < center.0 && p2.y > center.1 {
            counts[1] += 1;
        }
        if p2.x > center.0 && p2.y < center.1 {
            counts[2] += 1;
        }
        if p2.x > center.0 && p2.y > center.1 {
            counts[3] += 1;
        }
    }

    counts.into_iter().product::<u64>()
}

#[test]
fn test() {
    let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    let output = 12;
    assert_eq!(part_1(input).to_string(), output.to_string());
}

fn print_robots(r: &[(Point, Offset)], size: Bounds) {
    let mut grid = Grid::new_filled('.', size);

    for &(p, _) in r {
        grid[p] = '#';
    }
    println!("{grid}");
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut robots = input
        .lines()
        .map(|l| {
            let nums = l.split_whitespace().collect_vec();
            let (_, pos) = nums[0].split_once("p=").unwrap();
            let (_, v) = nums[1].split_once("v=").unwrap();
            let pos = pos.split(',').map(p::<usize>).collect_vec();
            let vel = v.split(',').map(p::<isize>).collect_vec();
            (
                Point {
                    x: pos[0],
                    y: pos[1],
                },
                Offset {
                    dx: vel[0],
                    dy: vel[1],
                },
            )
        })
        .collect_vec();
    let width = robots.iter().map(|(p, _)| p.x).max().unwrap() + 1;
    let height = robots.iter().map(|(p, _)| p.y).max().unwrap() + 1;
    let bounds = Bounds { width, height };

    let mut step = 0;

    loop {
        if robots.iter().map(|&(p, _)| p).collect::<HashSet<_>>().len() == robots.len() {
            print_robots(&robots, bounds);
            return step;
        }
        for (p, v) in robots.iter_mut() {
            let p2 = p.move_off_wrapping(*v, bounds);
            *p = p2;
        }
        step += 1;
    }
}
