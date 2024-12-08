use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut grid = input.parse::<Grid<char>>().unwrap();
    let start = grid.find(&'S').unwrap();

    let mut count = 0;

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((start, 0));
    visited.insert((start, 0));

    while let Some((from, depth)) = queue.pop_front() {
        if depth == 64 {
            count += 1;
            grid[from] = 'O';
            continue;
        }
        for to in adjacent_4_u(from.x, from.y) {
            if grid.contains_point(to) && grid[to] != '#' && !visited.contains(&(to, depth + 1)) {
                queue.push_back((to, depth + 1));
                visited.insert((to, depth + 1));
            }
        }
    }

    count
}

#[test]
fn test() {
    let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    let output = 16;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    const X: usize = 26501365;

    let grid = input.parse::<Grid<char>>().unwrap();

    let mut full_count_a = 0;
    let mut full_count_b = 0;

    let start = grid.find(&'S').unwrap();

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((start, 0));
    visited.insert((start, 0));

    while let Some((from, depth)) = queue.pop_front() {
        if depth == 400 {
            full_count_a += 1;
            continue;
        }
        for to in adjacent_4_u(from.x, from.y) {
            if grid.contains_point(to) && grid[to] != '#' && !visited.contains(&(to, depth + 1)) {
                queue.push_back((to, depth + 1));
                visited.insert((to, depth + 1));
            }
        }
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((start, 0));
    visited.insert((start, 0));

    while let Some((from, depth)) = queue.pop_front() {
        if depth == 401 {
            full_count_b += 1;
            continue;
        }
        for to in adjacent_4_u(from.x, from.y) {
            if grid.contains_point(to) && grid[to] != '#' && !visited.contains(&(to, depth + 1)) {
                queue.push_back((to, depth + 1));
                visited.insert((to, depth + 1));
            }
        }
    }

    println!("{full_count_a} {full_count_b}");

    let start = (start.x as isize, start.y as isize);

    let mut res = 0;

    let mut extremes = Vec::new();

    for (_, dir) in DIRECTIONS4D {
        for i in 0.. {
            let far = match dir {
                Direction4::Left => (0 - i * grid.width() as isize, start.1),
                Direction4::Up => (start.0, 0 - i * grid.height() as isize),
                Direction4::Down => (
                    start.0,
                    grid.height() as isize - 1 + i * grid.height() as isize,
                ),
                Direction4::Right => (
                    grid.width() as isize - 1 + i * grid.width() as isize,
                    start.1,
                ),
            };

            if manhattan_i(start, far) > X {
                let extreme = match dir {
                    Direction4::Left => (-i + 1, 0),
                    Direction4::Up => (0, -i + 1),
                    Direction4::Down => (0, i - 1),
                    Direction4::Right => (i - 1, 0),
                };
                extremes.push(extreme);

                break;
            }
        }
    }

    println!("{extremes:?}");

    /*
    'dfs: while let Some((gx, gy)) = stack.pop() {
        let ul = (grid.width() as isize * gx, grid.height() as isize * gy);
        let ur = (
            grid.width() as isize * gx + grid.width() as isize - 1,
            grid.height() as isize * gy,
        );
        let dl = (
            grid.width() as isize * gx,
            grid.height() as isize * gy + grid.height() as isize - 1,
        );
        let dr = (
            grid.width() as isize * gx + grid.width() as isize - 1,
            grid.height() as isize * gy + grid.height() as isize - 1,
        );
        let poss = [ul, ur, dl, dr].map(|(x, y)| (x as isize, y as isize));
        for p in poss {
            if manhattan_i(start, p) > X {
                boundaries.push((gx, gy));
                continue 'dfs;
            }
        }
        if (gx + gy) % 2 == 0 {
            res += full_count_a;
        } else {
            res += full_count_b;
        }

        for p in adjacent_4_i(gx, gy) {
            if !set.contains(&p) {
                stack.push(p);
                set.insert(p);
            }
        }
    }
    */

    let width = -extremes[0].0;
    println!("{width}");

    let mut boundaries = Vec::new();

    for i in 1..2 {
        boundaries.push((-width + i, i));
        boundaries.push((-width + i, -i));
        boundaries.push((width - i, i));
        boundaries.push((width - i, -i));
    }

    println!("{boundaries:?}");

    let dubm = [
        (grid.width() as isize - 1, start.1),
        (start.0, grid.height() as isize - 1),
        (start.0, 0),
        (0, start.1),
    ];

    for start in dubm {
        let start = Point {
            x: start.0 as usize,
            y: start.1 as usize,
        };
        let x = X as isize - 131 * width + 65;
        println!("{x} {start:?}");
        let mut count = 0;
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((start, 0));
        visited.insert((start, 0));

        while let Some((from, depth)) = queue.pop_front() {
            if depth == x {
                count += 1;
                continue;
            }
            for to in adjacent_4_u(from.x, from.y) {
                if grid.contains_point(to) && grid[to] != '#' && !visited.contains(&(to, depth + 1))
                {
                    queue.push_back((to, depth + 1));
                    visited.insert((to, depth + 1));
                }
            }
        }
        println!("{count}");
        res += count;
    }

    println!("m {}", manhattan_i(start, (-26501039, -1)));

    let dubm = [
        (grid.width() as isize - 1, 0),
        (grid.width() as isize - 1, grid.height() as isize - 1),
        (0, 0),
        (0, grid.height() as isize - 1),
    ];
    for start in dubm {
        let start = Point {
            x: start.0 as usize,
            y: start.1 as usize,
        };
        //let x = X as isize - 131 * width + 130;
        let x = 195;
        println!("{x} {start:?}");
        let mut count = 0;
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((start, 0));
        visited.insert((start, 0));

        while let Some((from, depth)) = queue.pop_front() {
            if depth == x {
                count += 1;
                continue;
            }
            for to in adjacent_4_u(from.x, from.y) {
                if grid.contains_point(to) && grid[to] != '#' && !visited.contains(&(to, depth + 1))
                {
                    queue.push_back((to, depth + 1));
                    visited.insert((to, depth + 1));
                }
            }
        }
        println!("{count}");
        res += count * (width - 1);
    }

    let dubm = [
        (grid.width() as isize - 1, 0),
        (grid.width() as isize - 1, grid.height() as isize - 1),
        (0, 0),
        (0, grid.height() as isize - 1),
    ];
    for start in dubm {
        let start = Point {
            x: start.0 as usize,
            y: start.1 as usize,
        };
        //let x = X as isize - 131 * width + 130;
        let x = 195 - 131;
        println!("{x} {start:?}");
        let mut count = 0;
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((start, 0));
        visited.insert((start, 0));

        while let Some((from, depth)) = queue.pop_front() {
            if depth == x {
                count += 1;
                continue;
            }
            for to in adjacent_4_u(from.x, from.y) {
                if grid.contains_point(to) && grid[to] != '#' && !visited.contains(&(to, depth + 1))
                {
                    queue.push_back((to, depth + 1));
                    visited.insert((to, depth + 1));
                }
            }
        }
        println!("{count}");
        res += count * width;
    }

    println!("hi");

    // 4 * width-1 + 4 * width-2 + ...
    // 4 * 1 + 1
    // (width-1 + 1) * (width-1) * 2

    for i in 1..width {
        if i % 2 == 0 {
            res += 4 * i * full_count_b;
        } else {
            res += 4 * i * full_count_a;
        }
    }

    res + full_count_b
}
