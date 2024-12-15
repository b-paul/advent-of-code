use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

fn try_push(grid: &Grid<char>, robot: Point, dir: Direction4) -> Option<Grid<char>> {
    let mut new_grid = grid.clone();

    let mut cur = robot;
    let mut cur_c = '.';
    loop {
        let next = cur.move_off(dir.offset())?;
        new_grid[cur] = cur_c;
        new_grid[next] = grid[cur];
        cur_c = grid[cur];
        cur = next;
        if grid[next] == '#' {
            return None;
        } else if grid[next] == '.' {
            break;
        }
    }
    Some(new_grid)
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut input = input.split("\n\n");
    let mut grid = input.next().unwrap().parse::<Grid<char>>().unwrap();
    let moves = input
        .next()
        .unwrap()
        .chars()
        .flat_map(|c| match c {
            '^' => Some(Direction4::Up),
            '>' => Some(Direction4::Right),
            'v' => Some(Direction4::Down),
            '<' => Some(Direction4::Left),
            _ => None,
        })
        .collect_vec();

    let mut robot = grid.find(&'@').unwrap();

    for dir in moves {
        if let Some(new_grid) = try_push(&grid, robot, dir) {
            robot = robot.move_off(dir.offset()).unwrap();
            grid = new_grid;
        }
    }

    grid.iter_idx()
        .filter_map(|(p, &c)| (c == 'O').then_some(p.y * 100 + p.x))
        .sum::<usize>()
}

fn try_push2(grid: &mut Grid<char>, point: Point, dir: Direction4, replace: char) -> Option<()> {
    if grid[point] == '.' {
        return Some(());
    }
    if grid[point] == '#' {
        return None;
    }
    let next = point.move_off(dir.offset())?;
    match dir {
        Direction4::Up | Direction4::Down => {
            match grid[next] {
                '.' => {}
                '[' => {
                    try_push2(grid, next, dir, grid[point])?;
                    let point = point.move_off(Direction4::Right.offset())?;
                    let next = point.move_off(dir.offset())?;
                    try_push2(grid, next, dir, '.')?;
                }
                ']' => {
                    try_push2(grid, next, dir, grid[point])?;
                    let point = point.move_off(Direction4::Left.offset())?;
                    let next = point.move_off(dir.offset())?;
                    try_push2(grid, next, dir, '.')?;
                },
                _ => return None,
            }
        }
        Direction4::Right | Direction4::Left => {
            try_push2(grid, point.move_off(dir.offset())?, dir, grid[point])?;
        }
    }
    grid[next] = grid[point];
    grid[point] = replace;
    Some(())
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut input = input.split("\n\n");
    let read_grid = input.next().unwrap().parse::<Grid<char>>().unwrap();
    let moves = input
        .next()
        .unwrap()
        .chars()
        .flat_map(|c| match c {
            '^' => Some(Direction4::Up),
            '>' => Some(Direction4::Right),
            'v' => Some(Direction4::Down),
            '<' => Some(Direction4::Left),
            _ => None,
        })
        .collect_vec();

    let mut grid = Grid::new_filled(
        '.',
        Bounds {
            width: read_grid.width() * 2,
            height: read_grid.height(),
        },
    );

    for (pos, &c) in read_grid.iter_idx() {
        let p2 = Point {
            x: pos.x * 2,
            y: pos.y,
        };
        let p3 = Point {
            x: pos.x * 2 + 1,
            y: pos.y,
        };
        match c {
            '#' => {
                grid[p2] = '#';
                grid[p3] = '#';
            }
            'O' => {
                grid[p2] = '[';
                grid[p3] = ']';
            }
            '.' => {
                grid[p2] = '.';
                grid[p3] = '.';
            }
            '@' => {
                grid[p2] = '@';
                grid[p3] = '.';
            }
            _ => panic!(),
        }
    }

    let mut robot = grid.find(&'@').unwrap();

    for dir in moves {
        let mut new_grid = grid.clone();
        if let Some(_) = try_push2(&mut new_grid, robot, dir, '.') {
            robot = robot.move_off(dir.offset()).unwrap();
            grid = new_grid;
        }
    }

    grid.iter_idx()
        .filter_map(|(p, &c)| (c == '[').then_some(p.y * 100 + p.x))
        .sum::<usize>()
}
