use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let points = input
        .lines()
        .map(|l| {
            let nums = l.split(',').map(p::<usize>).collect_vec();
            Point {
                x: nums[0],
                y: nums[1],
            }
        })
        .collect_vec();

    let width = points.iter().map(|p| p.x).max().unwrap() + 1;
    let height = points.iter().map(|p| p.y).max().unwrap() + 1;

    let mut grid = Grid::new_filled('.', Bounds { width, height });
    let exit = Point {
        x: width - 1,
        y: height - 1,
    };

    // Change 1024 to 12 when running the test :(
    for p in &points[..1024] {
        grid[*p] = '#';
    }

    let mut best_score = None;
    grid.bfs_4(
        Point { x: 0, y: 0 },
        |p, _, d| {
            if p == exit && best_score == None {
                best_score = Some(d);
            }
        },
        |_, _, t| t != '#',
    );

    best_score.unwrap()
}

#[test]
fn test() {
    let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    let output = 117440;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let points = input
        .lines()
        .map(|l| {
            let nums = l.split(',').map(p::<usize>).collect_vec();
            Point {
                x: nums[0],
                y: nums[1],
            }
        })
        .collect_vec();

    let width = points.iter().map(|p| p.x).max().unwrap() + 1;
    let height = points.iter().map(|p| p.y).max().unwrap() + 1;

    let mut grid = Grid::new_filled('.', Bounds { width, height });
    let exit = Point {
        x: width - 1,
        y: height - 1,
    };

    for d in 0..points.len() {
        grid[points[d]] = '#';
        let mut ok = false;
        grid.bfs_4(
            Point { x: 0, y: 0 },
            |p, _, _| {
                if p == exit {
                    ok = true;
                }
            },
            |_, _, t| t != '#',
        );
        if !ok {
            println!("{}", points[d]);
            return "manual submit today :(".to_string();
        }
    }

    "manual submit today :(".to_string()
}
