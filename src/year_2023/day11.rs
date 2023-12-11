use crate::helper::prelude::*;
use std::collections::*;
use itertools::Itertools;

fn dist((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    let x1 = x1 as isize;
    let y1 = y1 as isize;
    let x2 = x2 as isize;
    let y2 = y2 as isize;

    (x1 - x2).abs() as usize + (y1 - y2).abs() as usize
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let (mut grid, mut width, mut height) = input.parse::<Grid<char>>().unwrap().to_vecvec();

    println!("{grid:?}");

    let mut i = height - 1;
    loop {
        if grid[i].iter().all(|c| *c == '.') {
            let row = grid[i].clone();
            grid.insert(i, row);
            height += 1;
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }

    let mut grid2 = Vec::with_capacity(width);

    for y in 0..width {
        let mut row = Vec::with_capacity(height);
        for x in 0..height {
            row.push(grid[x][y]);
        }
        grid2.push(row);
    }

    println!("done");

    let mut grid = grid2;

    let mut i = width - 1;
    loop {
        if grid[i].iter().all(|c| *c == '.') {
            let row = grid[i].clone();
            grid.insert(i, row);
            width += 1;
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }

    println!("done");

    let mut grid2 = Vec::with_capacity(width);

    for y in 0..height {
        let mut row = Vec::with_capacity(height);
        for x in 0..width {
            row.push(grid[x][y]);
        }
        grid2.push(row);
    }

    let grid = grid2;

    println!("done");

    let mut galaxies = Vec::new();

    for x in 0..width {
        for y in 0..height {
            if grid[y][x] == '#' {
                galaxies.push((x, y));
            }
        }
    }

    let mut r = 0;

    for (i, a) in galaxies.iter().enumerate() {
        for (j, b) in galaxies.iter().enumerate() {
            if i != j {
                r += dist(*a, *b);
            }
        }
    }

    r/2
}

#[test]
fn part1() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    let output = 374;
    //assert_eq!(part_1(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let (grid, width, height) = input.parse::<Grid<char>>().unwrap().to_vecvec();

    let mut empty_rows = Vec::new();

    for y in 0..height {
        if grid[y].iter().all(|c| *c == '.') {
            empty_rows.push(true);
        } else {
            empty_rows.push(false);
        }
    }

    let mut empty_cols = Vec::new();

    for x in 0..width {
        let mut empty = true;
        for y in 0..height {
            if grid[y][x] != '.' {
                empty = false;
            }
        }

        empty_cols.push(empty);
    }

    let mut galaxies = Vec::new();

    for x in 0..width {
        for y in 0..height {
            if grid[y][x] == '#' {
                galaxies.push((x, y));
            }
        }
    }

    println!("{}", galaxies.len());

    let mut r = 0;

    for (i, (x1, y1)) in galaxies.iter().enumerate() {
        for (j, (x2, y2)) in galaxies.iter().enumerate() {
            if i != j {
                let xl = *x1.min(x2);
                let xr = *x1.max(x2);
                let yl = *y1.min(y2);
                let yr = *y1.max(y2);
                let mut d = 0;
                for x in (xl + 1)..xr {
                    if empty_cols[x] {
                        d += 999999;
                    }
                }
                for y in (yl + 1)..yr {
                    if empty_rows[y] {
                        d += 999999;
                    }
                }

                d += dist((*x1, *y1), (*x2, *y2));

                r += d;
            }
        }
    }

    r/2
}

#[test]
fn part2() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    let output = 8410;
    assert_eq!(part_2(input).to_string(), output.to_string());
}
