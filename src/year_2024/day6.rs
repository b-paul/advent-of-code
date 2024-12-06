use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();
    let mut squares = HashSet::new();

    let mut pos = grid.find(&'^').unwrap();
    let mut dir = Direction4::Up;

    loop {
        squares.insert(pos);
        let Some(posp) = dir.moveub(pos, (grid.width(), grid.height())) else {
            break;
        };
        if grid.get(posp) == Some(&'#') {
            dir = dir.cw();
        } else {
            pos = posp;
        }
    }

    squares.len()
}

#[test]
fn test() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    let output = 6;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();
    grid.iter_idx()
        .filter(|(pos, _)| {
            let mut grid = grid.clone();
            grid[*pos] = '#';
            let mut squares = HashSet::new();

            let Some(mut pos) = grid.find(&'^') else {
                return false;
            };
            let mut dir = Direction4::Up;

            loop {
                if squares.contains(&(pos, dir)) {
                    return true;
                }
                squares.insert((pos, dir));
                let Some(posp) = dir.moveub(pos, (grid.width(), grid.height())) else {
                    return false;
                };
                if grid.get(posp) == Some(&'#') {
                    dir = dir.cw();
                } else {
                    pos = posp;
                }
            }
        })
        .count()
}
