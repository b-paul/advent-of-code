use crate::helper::grid::Grid;
use std::collections::{HashMap, HashSet};

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();

    grid.iter_cols()
        .map(|c| {
            let mut r = 0;
            let mut cur = 0;
            for (i, c) in c.copied().enumerate() {
                if c == 'O' {
                    r += grid.height() - cur;
                    cur += 1;
                } else if c == '#' {
                    cur = i + 1;
                }
            }
            r
        })
        .sum::<usize>()
}

fn load(grid: &Grid<char>) -> usize {
    grid.iter_cols()
        .map(|col| {
            col.enumerate()
                .map(|(i, &c)| if c == 'O' { grid.height() - i } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn mv(grid: &Grid<char>, cycle: usize) -> Grid<char> {
    match cycle % 4 {
        0 => {
            // north
            let mut new_grid = grid.clone();
            for (x, col) in grid.iter_cols().enumerate() {
                let mut cur = 0;
                for (y, c) in col.copied().enumerate() {
                    if c == 'O' {
                        new_grid[(x, y)] = '.';
                        new_grid[(x, cur)] = 'O';
                        cur += 1;
                    } else if c == '#' {
                        cur = y + 1;
                    }
                }
            }
            new_grid
        }
        1 => {
            // west
            let mut new_grid = grid.clone();
            for (y, row) in grid.iter_rows().enumerate() {
                let mut cur = 0;
                for (x, c) in row.copied().enumerate() {
                    if c == 'O' {
                        new_grid[(x, y)] = '.';
                        new_grid[(cur, y)] = 'O';
                        cur += 1;
                    } else if c == '#' {
                        cur = x + 1;
                    }
                }
            }
            new_grid
        }
        2 => {
            // south
            let mut new_grid = grid.clone();
            for (x, col) in grid.iter_cols().enumerate() {
                let mut cur = grid.height() - 1;
                let mut col = col.copied().enumerate().collect::<Vec<_>>();
                col.reverse();
                for (y, c) in col {
                    if c == 'O' {
                        new_grid[(x, y)] = '.';
                        new_grid[(x, cur)] = 'O';
                        cur -= 1;
                    } else if c == '#' && y != 0 {
                        cur = y - 1;
                    }
                }
            }
            new_grid
        }
        3 => {
            // east
            let mut new_grid = grid.clone();
            for (y, row) in grid.iter_rows().enumerate() {
                let mut cur = grid.width() - 1;
                let mut row = row.copied().enumerate().collect::<Vec<_>>();
                row.reverse();
                for (x, c) in row {
                    if c == 'O' {
                        new_grid[(x, y)] = '.';
                        new_grid[(cur, y)] = 'O';
                        cur -= 1;
                    } else if c == '#' && x != 0 {
                        cur = x - 1;
                    }
                }
            }
            new_grid
        }
        _ => unreachable!(),
    }
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut grid = input.parse::<Grid<char>>().unwrap();

    let mut set = HashMap::new();
    let mut map = HashMap::new();
    let mut cur_cycle_set: Vec<(usize, usize)> = Vec::new();

    for cy in 0..1000000000 {
        for dir in 0..4 {
            grid = mv(&grid, dir);
        }
        let hash = grid.clone().to_vec();

        let load = load(&grid);
        if let Some((cy2, load)) = set.get(&hash) {
            if cur_cycle_set.contains(&(*cy2, *load)) {
                return cur_cycle_set[(999999999 - cur_cycle_set[0].0) % cur_cycle_set.len()].1;
            }

            cur_cycle_set.push((*cy2, *load));
        } else {
            set.insert(hash, (cy, load));
            cur_cycle_set.clear()
        }
        map.insert(cy, load);
    }

    0
}
