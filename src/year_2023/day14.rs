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

pub(crate) fn part_1_faster(input: &str) -> impl std::fmt::Display {
    let mut r = 0;
    let mut tops = [0; 100];
    let input = input.as_bytes();
    for row in 0..100 {
        for col in 0..100 {
            let c = input[row * 101 + col];
            if c == b'#' {
                tops[col] = row + 1;
            } else if c == b'O' {
                r += 100 - tops[col];
                tops[col] += 1;
            }
        }
    }
    r
}

use std::simd::prelude::*;

pub(crate) fn part_1_simd(input: &str) -> impl std::fmt::Display {
    let mut input = input.as_bytes();

    let wall = u8x32::splat(b'#');
    let rock = u8x32::splat(b'O');

    let mut r = 0;

    for _ in 0..3 {
        let mut tops = u8x32::splat(0);
        for row in 0..100 {
            let cs = u8x32::from_slice(&input[row * 101..]);

            let walls = cs.simd_eq(wall);
            let row_plus_1 = u8x32::splat(row as u8 + 1);
            tops = walls.select(row_plus_1, tops);

            let rocks = cs.simd_eq(rock);
            let res = rocks.select(u8x32::splat(100) - tops, u8x32::splat(0));
            let res = res.cast::<i16>();
            r += res.reduce_sum() as i32;
            let tops_plus_1 = tops + u8x32::splat(1);
            tops = rocks.select(tops_plus_1, tops);
        }

        input = &input[32..];
    }
    // end where we look at 4 bytes only
    let wall = u8x4::splat(b'#');
    let rock = u8x4::splat(b'O');
    {
        let mut tops = u8x4::splat(0);
        for row in 0..100 {
            let cs = u8x4::from_slice(&input[row * 101..]);

            let walls = cs.simd_eq(wall);
            let row_plus_1 = u8x4::splat(row as u8 + 1);
            tops = walls.select(row_plus_1, tops);

            let rocks = cs.simd_eq(rock);
            let res = rocks.select(u8x4::splat(100) - tops, u8x4::splat(0));
            let res = res.cast::<i16>();
            r += res.reduce_sum() as i32;
            let tops_plus_1 = tops + u8x4::splat(1);
            tops = rocks.select(tops_plus_1, tops);
        }
    }

    r
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

fn mv(grid: &mut Grid<char>) {
    for _ in 0..4 {
        for x in 0..grid.width() {
            let mut cur = 0;
            for y in 0..grid.height() {
                let c = grid[(x, y)];
                if c == 'O' {
                    grid[(x, y)] = '.';
                    grid[(x, cur)] = 'O';
                    cur += 1;
                } else if c == '#' {
                    cur = y + 1;
                }
            }
        }
        *grid = grid.rotate_cw();
    }
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut grid = input.parse::<Grid<char>>().unwrap();

    let mut set = HashMap::new();
    let mut cur_cycle_set: Vec<(usize, usize)> = Vec::new();

    for cy in 0..1000000000 {
        mv(&mut grid);
        let load = load(&grid);
        if let Some((cy2, load)) = set.get(&grid) {
            if cur_cycle_set.contains(&(*cy2, *load)) {
                return cur_cycle_set[(999999999 - cur_cycle_set[0].0) % cur_cycle_set.len()].1;
            }

            cur_cycle_set.push((*cy2, *load));
        } else {
            set.insert(grid.clone(), (cy, load));
            cur_cycle_set.clear()
        }
    }

    panic!("Failed to find a cycle")
}

fn floors(grid: &Grid<char>) -> [[[(usize, usize); 100]; 100]; 4] {
    let mut floors = [[[(0, 0); 100]; 100]; 4];

    // north
    for x in 0..100 {
        let mut cur = 0;
        for y in 0..100 {
            if grid[(x, y)] == '#' {
                cur = y + 1
            }
            floors[0][y][x] = (x, cur);
        }
    }

    // west
    for y in 0..100 {
        let mut cur = 0;
        for x in 0..100 {
            if grid[(x, y)] == '#' {
                cur = x + 1
            }
            floors[1][y][x] = (cur, y);
        }
    }

    // south
    for x in 0..100 {
        let mut cur = 99;
        for y in (0..100).rev() {
            if grid[(x, y)] == '#' {
                cur = y - 1
            }
            floors[2][y][x] = (x, cur);
        }
    }

    // east
    for y in 0..100 {
        let mut cur = 99;
        for x in (0..100).rev() {
            if grid[(x, y)] == '#' {
                cur = x - 1
            }
            floors[3][y][x] = (cur, y);
        }
    }

    floors
}

fn mv2(
    grid: &mut Grid<char>,
    rocks: &mut [(usize, usize)],
    floors: &[[[(usize, usize); 100]; 100]; 4],
) {
    // north
    for (x, y) in rocks.iter_mut() {
        grid[(*x, *y)] = '.';
        let (fx, mut fy) = floors[0][*y][*x];
        while grid[(fx, fy)] != '.' {
            fy += 1;
        }
        *y = fy;
        grid[(*x, *y)] = 'O';
    }

    // west
    for (x, y) in rocks.iter_mut() {
        grid[(*x, *y)] = '.';
        let (mut fx, fy) = floors[1][*y][*x];
        while grid[(fx, fy)] != '.' {
            fx += 1;
        }
        *x = fx;
        grid[(*x, *y)] = 'O';
    }

    // south
    for (x, y) in rocks.iter_mut() {
        grid[(*x, *y)] = '.';
        let (fx, mut fy) = floors[2][*y][*x];
        while grid[(fx, fy)] != '.' {
            fy -= 1;
        }
        *y = fy;
        grid[(*x, *y)] = 'O';
    }

    // east
    for (x, y) in rocks.iter_mut() {
        grid[(*x, *y)] = '.';
        let (mut fx, fy) = floors[3][*y][*x];
        while grid[(fx, fy)] != '.' {
            fx -= 1;
        }
        *x = fx;
        grid[(*x, *y)] = 'O';
    }
}

pub(crate) fn part_2_faster(input: &str) -> impl std::fmt::Display {
    let mut grid = input.parse::<Grid<char>>().unwrap();
    let mut rocks = grid
        .iter_idx()
        .filter(|(_, &c)| c == 'O')
        .map(|(p, _)| p.pair())
        .collect::<Vec<_>>();

    let mut set = HashMap::new();
    let mut cur_cycle_set: Vec<(usize, usize)> = Vec::new();

    let floors = floors(&grid);

    for cy in 0..1000000000 {
        mv2(&mut grid, &mut rocks, &floors);
        let load = load(&grid);
        if let Some((cy2, load)) = set.get(&grid) {
            if cur_cycle_set.contains(&(*cy2, *load)) {
                return cur_cycle_set[(999999999 - cur_cycle_set[0].0) % cur_cycle_set.len()].1;
            }

            cur_cycle_set.push((*cy2, *load));
        } else {
            set.insert(grid.clone(), (cy, load));
            cur_cycle_set.clear()
        }
    }

    panic!("Failed to find a cycle")
}

pub(crate) fn part_2_brent(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();
    let mut rocks = grid
        .iter_idx()
        .filter(|(_, &c)| c == 'O')
        .map(|(p, _)| p.pair())
        .collect::<Vec<_>>();

    let floors = floors(&grid);

    let mut pow = 1;
    let mut lam = 1;
    let mut tortoise = grid.clone();
    let mut hare = grid;
    mv2(&mut hare, &mut rocks, &floors);

    let mut count = 0;

    while hare != tortoise {
        if pow == lam {
            tortoise = hare.clone();
            pow *= 2;
            lam = 0;
        }
        mv2(&mut hare, &mut rocks, &floors);
        lam += 1;
        count += 1;
    }

    for _ in 0..(999999999 - count) % lam {
        mv2(&mut hare, &mut rocks, &floors);
    }

    load(&hare)
}

#[cfg(test)]
mod benches {
    use crate::get_input;
    use crate::year_2023::day14::*;
    use test::{black_box, Bencher};

    #[bench]
    fn part1_normal(b: &mut Bencher) {
        let input = &get_input(2023, 14).unwrap();
        b.iter(|| {
            black_box(part_1(input));
        })
    }

    #[bench]
    fn part1_faster(b: &mut Bencher) {
        let input = &get_input(2023, 14).unwrap();
        assert_eq!(part_1_faster(input).to_string(), part_1(input).to_string());
        b.iter(|| {
            black_box(part_1_faster(input));
        })
    }

    #[bench]
    fn part1_simd(b: &mut Bencher) {
        let input = &get_input(2023, 14).unwrap();
        assert_eq!(part_1_simd(input).to_string(), part_1(input).to_string());
        b.iter(|| {
            black_box(part_1_simd(input));
        })
    }

    #[bench]
    fn part2_normal(b: &mut Bencher) {
        let input = &get_input(2023, 14).unwrap();
        b.iter(|| {
            black_box(part_2(input));
        })
    }

    #[bench]
    fn part2_faster(b: &mut Bencher) {
        let input = &get_input(2023, 14).unwrap();
        assert_eq!(part_2_faster(input).to_string(), part_2(input).to_string());
        b.iter(|| {
            black_box(part_2_faster(input));
        })
    }

    #[bench]
    fn part2_brent(b: &mut Bencher) {
        let input = &get_input(2023, 14).unwrap();
        assert_eq!(part_2_brent(input).to_string(), part_2(input).to_string());
        b.iter(|| {
            black_box(part_2_brent(input));
        })
    }

    #[bench]
    fn moving(b: &mut Bencher) {
        let input = &get_input(2023, 14).unwrap();
        let mut grid = input.parse::<Grid<char>>().unwrap();
        let mut rocks = grid
            .iter_idx()
            .filter(|(_, &c)| c == 'O')
            .map(|(p, _)| p.pair())
            .collect::<Vec<_>>();

        let floors = floors(&grid);

        b.iter(|| {
            black_box(mv2(&mut grid, &mut rocks, &floors));
        })
    }
}
