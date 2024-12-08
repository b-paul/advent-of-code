use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

fn run(grid: &Grid<char>, start: (Direction4, (isize, isize))) -> usize {
    let mut energised = Grid::new_filled(false, grid.bounds());

    let bound_ul = (0, 0);
    let bound_dr = (grid.width() as isize, grid.height() as isize);

    let mut stack = Vec::new();
    let mut set = HashSet::new();

    stack.push(start);
    set.insert(start);

    while let Some((dir, (x, y))) = stack.pop() {
        if x >= 0 && (x as usize) < grid.width() && y >= 0 && (y as usize) < grid.height() {
            energised[(x as usize, y as usize)] = true;
        }

        let Some((x, y)) = dir.moveib((x, y), bound_ul, bound_dr) else {
            continue;
        };

        let mut dirs = Vec::new();

        match grid[(x as usize, y as usize)] {
            '.' => {
                dirs.push(dir);
            }
            '/' => {
                let dir = match dir {
                    Direction4::Left => Direction4::Down,
                    Direction4::Up => Direction4::Right,
                    Direction4::Down => Direction4::Left,
                    Direction4::Right => Direction4::Up,
                };
                dirs.push(dir);
            }
            '\\' => {
                let dir = match dir {
                    Direction4::Left => Direction4::Up,
                    Direction4::Up => Direction4::Left,
                    Direction4::Down => Direction4::Right,
                    Direction4::Right => Direction4::Down,
                };
                dirs.push(dir);
            }
            '-' => match dir {
                Direction4::Left | Direction4::Right => {
                    dirs.push(dir);
                }
                Direction4::Up | Direction4::Down => {
                    dirs.push(Direction4::Left);
                    dirs.push(Direction4::Right);
                }
            },
            '|' => match dir {
                Direction4::Up | Direction4::Down => {
                    dirs.push(dir);
                }
                Direction4::Left | Direction4::Right => {
                    dirs.push(Direction4::Up);
                    dirs.push(Direction4::Down);
                }
            },
            a => panic!("invalid grid square {a}"),
        }
        for dir in dirs {
            let key = (dir, (x, y));
            if !set.contains(&key) {
                stack.push(key);
                set.insert(key);
            }
        }
    }

    energised.iter_idx().filter(|(_, &t)| t).count()
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();
    run(&grid, (Direction4::Right, (-1, 0)))
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();
    let mut max = 0;

    for x in 0..grid.width() {
        max = max.max(run(&grid, (Direction4::Down, (x as isize, -1))));
        max = max.max(run(
            &grid,
            (Direction4::Up, (x as isize, grid.height() as isize)),
        ));
    }
    for y in 0..grid.height() {
        max = max.max(run(&grid, (Direction4::Right, (-1, y as isize))));
        max = max.max(run(
            &grid,
            (Direction4::Left, (grid.width() as isize, y as isize)),
        ));
    }

    max
}
