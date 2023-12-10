use crate::helper::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut start = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (x, y);
                    }
                    c
                })
                .collect_vec()
        })
        .collect_vec();

    let mut max_depth = 0;
    let mut stack = VecDeque::new();
    let mut visited = HashSet::new();
    stack.push_back((start, 0));
    visited.insert(start);

    while let Some(((x, y), depth)) = stack.pop_front() {
        max_depth = max_depth.max(depth);

        let c = grid[y][x];

        // Up
        if y > 0 && "S|LJ".contains(c) {
            let nx = x;
            let ny = y - 1;
            if !visited.contains(&(nx, ny)) {
                let c = grid[ny][nx];
                if "|7F".contains(c) {
                    stack.push_back(((nx, ny), depth + 1));
                    visited.insert((nx, ny));
                }
            }
        }
        // Down
        if y < grid.len() - 1 && "S|7F".contains(c) {
            let nx = x;
            let ny = y + 1;
            if !visited.contains(&(nx, ny)) {
                let c = grid[ny][nx];
                if "|LJ".contains(c) {
                    stack.push_back(((nx, ny), depth + 1));
                    visited.insert((nx, ny));
                }
            }
        }
        // Left
        if x > 0 && "S-J7".contains(c) {
            let nx = x - 1;
            let ny = y;
            if !visited.contains(&(nx, ny)) {
                let c = grid[ny][nx];
                if "-LF".contains(c) {
                    stack.push_back(((nx, ny), depth + 1));
                    visited.insert((nx, ny));
                }
            }
        }

        // Right
        if x < grid[0].len() - 1 && "S-LF".contains(c) {
            let nx = x + 1;
            let ny = y;
            if !visited.contains(&(nx, ny)) {
                let c = grid[ny][nx];
                if "-7J".contains(c) {
                    stack.push_back(((nx, ny), depth + 1));
                    visited.insert((nx, ny));
                }
            }
        }
    }

    max_depth
}

/*
#[test]
fn part1() {
    let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    let output = 8;
    assert_eq!(part_1(input).to_string(), output.to_string());
}
*/

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut start = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (x, y);
                    }
                    c
                })
                .collect_vec()
        })
        .collect_vec();

    let mut main = grid.iter().map(|l| l.iter().map(|_| false).collect_vec()).collect_vec();

    let mut stack = VecDeque::new();
    let mut visited = HashSet::new();
    stack.push_back((start, 0));
    visited.insert(start);

    while let Some(((x, y), depth)) = stack.pop_front() {
        main[y][x] = true;

        let c = grid[y][x];

        // Up
        if y > 0 && "S|LJ".contains(c) {
            let nx = x;
            let ny = y - 1;
            if !visited.contains(&(nx, ny)) {
                let c = grid[ny][nx];
                if "|7F".contains(c) {
                    stack.push_back(((nx, ny), depth + 1));
                    visited.insert((nx, ny));
                }
            }
        }
        // Down
        if y < grid.len() - 1 && "S|7F".contains(c) {
            let nx = x;
            let ny = y + 1;
            if !visited.contains(&(nx, ny)) {
                let c = grid[ny][nx];
                if "|LJ".contains(c) {
                    stack.push_back(((nx, ny), depth + 1));
                    visited.insert((nx, ny));
                }
            }
        }
        // Left
        if x > 0 && "S-J7".contains(c) {
            let nx = x - 1;
            let ny = y;
            if !visited.contains(&(nx, ny)) {
                let c = grid[ny][nx];
                if "-LF".contains(c) {
                    stack.push_back(((nx, ny), depth + 1));
                    visited.insert((nx, ny));
                }
            }
        }

        // Right
        if x < grid[0].len() - 1 && "S-LF".contains(c) {
            let nx = x + 1;
            let ny = y;
            if !visited.contains(&(nx, ny)) {
                let c = grid[ny][nx];
                if "-7J".contains(c) {
                    stack.push_back(((nx, ny), depth + 1));
                    visited.insert((nx, ny));
                }
            }
        }
    }

    let grid = grid.into_iter().enumerate().map(|(y, l)|{
        l.into_iter().enumerate().map(|(x, c)| {
            if main[y][x] {
                c
            } else {
                '.'
            }
        }).collect_vec()
    }).collect_vec();

    for line in grid.iter() {
        for c in line {
            print!("{c}");
            if *c == '.' {
            }
        }
        println!();
    }

    // ...
    // .F.
    // ...
    // |
    // v
    // .....
    // .....
    // ..##.
    // ..#..
    // .....
    let mut adjusted_grid = Vec::new();
    let h = grid.len();
    let w = grid[0].len();
    for _ in 0..h {
        adjusted_grid.push(vec!['.'; w*2 + 1]);
        adjusted_grid.push(vec!['.'; w*2 + 1]);
    }
    adjusted_grid.push(vec!['.'; w*2 + 1]);

    for (y, l) in grid.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c == '.' {
                continue;
            }
            adjusted_grid[2*y+1][2*x+1] = '#';
            if "|LJS".contains(*c) { adjusted_grid[2*y][2*x+1] = '#'; }
            if "|7FS".contains(*c) { adjusted_grid[2*y+2][2*x+1] = '#'; }
            if "-LFS".contains(*c) { adjusted_grid[2*y+1][2*x+2] = '#'; }
            if "-J7S".contains(*c) { adjusted_grid[2*y+1][2*x] = '#'; }
            if x == 0 {
                adjusted_grid[2*y+1][2*x] = '.';
            }
            if x == w-1 {
                adjusted_grid[2*y+1][2*x+2] = '.';
            }
            if y == 0 {
                adjusted_grid[2*y][2*x+1] = '.';
            }
            if y == h-1 {
                adjusted_grid[2*y+2][2*x+1] = '.';
            }
        }
    }

    let mut set = HashSet::new();
    let mut stack = Vec::new();
    stack.push((0, 0));
    set.insert((0,0));
    while let Some((x, y)) = stack.pop() {
        adjusted_grid[y][x] = '?';
        for (x, y) in adjacent_4_u(x, y) {
            if x > 2*w { continue; }
            if y > 2*h { continue; }
            if adjusted_grid[y][x] == '.' && !set.contains(&(x, y)) {
                set.insert((x, y));
                stack.push((x, y));
            }
        }
    }

    for line in adjusted_grid.iter() {
        for c in line {
            print!("{c}");
        }
        println!();
    }

    let mut ground = 0;
    for (y, l) in grid.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if adjusted_grid[2*y+1][2*x+1] == '.' {
                ground += 1;
            }
        }
    }


    ground
}

#[test]
fn part2() {
    let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    let output = 10;
    assert_eq!(part_2(input).to_string(), output.to_string());
}
