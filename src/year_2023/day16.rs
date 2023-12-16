use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();
    let mut energised = grid.clone().map(|_| false);

    let mut stack = Vec::new();
    let mut set = HashSet::new();

    stack.push((Direction4::Right, (-1, 0)));
    set.insert((Direction4::Right, (-1, 0)));

    while let Some((dir, (x, y))) = stack.pop() {
        if x >= 0 && y >= 0 {
            energised[(x as usize, y as usize)] = true;
        }

        let (dx, dy) = DIRECTIONS4[dir as usize];
        let (x, y) = (x + dx, y + dy);
        if x < 0 || x as usize >= grid.width() || y < 0 || y as usize >= grid.height() {
            continue;
        }

        match grid[(x as usize, y as usize)] {
            '.' => {
                let key = (dir, (x, y));
                if !set.contains(&key) {
                    stack.push(key);
                    set.insert(key);
                }
            }
            '/' => {
                let dir = match dir {
                    Direction4::Left => Direction4::Down,
                    Direction4::Up => Direction4::Right,
                    Direction4::Down => Direction4::Left,
                    Direction4::Right => Direction4::Up,
                };
                let key = (dir, (x, y));
                if !set.contains(&key) {
                    stack.push(key);
                    set.insert(key);
                }
            }
            '\\' => {
                let dir = match dir {
                    Direction4::Left => Direction4::Up,
                    Direction4::Up => Direction4::Left,
                    Direction4::Down => Direction4::Right,
                    Direction4::Right => Direction4::Down,
                };
                let key = (dir, (x, y));
                if !set.contains(&key) {
                    stack.push(key);
                    set.insert(key);
                }
            }
            '-' => match dir {
                Direction4::Left | Direction4::Right => {
                    let key = (dir, (x, y));
                    if !set.contains(&key) {
                        stack.push(key);
                        set.insert(key);
                    }
                }
                Direction4::Up | Direction4::Down => {
                    let dir = Direction4::Right;
                    let key = (dir, (x, y));
                    if !set.contains(&key) {
                        stack.push(key);
                        set.insert(key);
                    }
                    let dir = Direction4::Left;
                    let key = (dir, (x, y));
                    if !set.contains(&key) {
                        stack.push(key);
                        set.insert(key);
                    }
                }
            },
            '|' => match dir {
                Direction4::Up | Direction4::Down => {
                    let key = (dir, (x, y));
                    if !set.contains(&key) {
                        stack.push(key);
                        set.insert(key);
                    }
                }
                Direction4::Left | Direction4::Right => {
                    let dir = Direction4::Up;
                    let key = (dir, (x, y));
                    if !set.contains(&key) {
                        stack.push(key);
                        set.insert(key);
                    }
                    let dir = Direction4::Down;
                    let key = (dir, (x, y));
                    if !set.contains(&key) {
                        stack.push(key);
                        set.insert(key);
                    }
                }
            },
            a => panic!("invalid grid square {a}"),
        }
    }

    let ep = energised.clone().map(|c| match c {
        true => '#',
        false => '.',
    });
    println!("{ep}");

    energised
        .iter_idx()
        .filter(|(_, &t)| t)
        .inspect(|(p, _)| println!("{:?}", p))
        .count()
}

#[test]
fn test() {
    let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
    let output = 46;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

fn run(grid: &Grid<char>, start: (Direction4, (isize, isize))) -> usize {
    let mut energised = grid.clone().map(|_| false);

    let mut stack = Vec::new();
    let mut set = HashSet::new();

    stack.push(start);
    set.insert(start);

    while let Some((dir, (x, y))) = stack.pop() {
        if x >= 0 && (x as usize) < grid.width() && y >= 0 && (y as usize) < grid.height() {
            energised[(x as usize, y as usize)] = true;
        }

        let (dx, dy) = DIRECTIONS4[dir as usize];
        let (x, y) = (x + dx, y + dy);
        if x < 0 || x as usize >= grid.width() || y < 0 || y as usize >= grid.height() {
            continue;
        }

        match grid[(x as usize, y as usize)] {
            '.' => {
                let key = (dir, (x, y));
                if !set.contains(&key) {
                    stack.push(key);
                    set.insert(key);
                }
            }
            '/' => {
                let dir = match dir {
                    Direction4::Left => Direction4::Down,
                    Direction4::Up => Direction4::Right,
                    Direction4::Down => Direction4::Left,
                    Direction4::Right => Direction4::Up,
                };
                let key = (dir, (x, y));
                if !set.contains(&key) {
                    stack.push(key);
                    set.insert(key);
                }
            }
            '\\' => {
                let dir = match dir {
                    Direction4::Left => Direction4::Up,
                    Direction4::Up => Direction4::Left,
                    Direction4::Down => Direction4::Right,
                    Direction4::Right => Direction4::Down,
                };
                let key = (dir, (x, y));
                if !set.contains(&key) {
                    stack.push(key);
                    set.insert(key);
                }
            }
            '-' => match dir {
                Direction4::Left | Direction4::Right => {
                    let key = (dir, (x, y));
                    if !set.contains(&key) {
                        stack.push(key);
                        set.insert(key);
                    }
                }
                Direction4::Up | Direction4::Down => {
                    let dir = Direction4::Right;
                    let key = (dir, (x, y));
                    if !set.contains(&key) {
                        stack.push(key);
                        set.insert(key);
                    }
                    let dir = Direction4::Left;
                    let key = (dir, (x, y));
                    if !set.contains(&key) {
                        stack.push(key);
                        set.insert(key);
                    }
                }
            },
            '|' => match dir {
                Direction4::Up | Direction4::Down => {
                    let key = (dir, (x, y));
                    if !set.contains(&key) {
                        stack.push(key);
                        set.insert(key);
                    }
                }
                Direction4::Left | Direction4::Right => {
                    let dir = Direction4::Up;
                    let key = (dir, (x, y));
                    if !set.contains(&key) {
                        stack.push(key);
                        set.insert(key);
                    }
                    let dir = Direction4::Down;
                    let key = (dir, (x, y));
                    if !set.contains(&key) {
                        stack.push(key);
                        set.insert(key);
                    }
                }
            },
            a => panic!("invalid grid square {a}"),
        }
    }

    let ep = energised.clone().map(|c| match c {
        true => '#',
        false => '.',
    });
    println!("{ep}");

    energised.iter_idx().filter(|(_, &t)| t).count()
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
