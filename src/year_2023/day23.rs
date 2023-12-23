use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();

    let end = (grid.width() - 2, grid.height() - 1);

    let mut max = 0;

    let mut stack = Vec::new();
    let mut set = HashSet::new();
    set.insert((1, 0));
    stack.push((set, (1, 0), 0));

    while let Some((set, (x, y), depth)) = stack.pop() {
        if (x, y) == end {
            max = max.max(depth);
        }

        for (to, dir) in adjacent_4_ud(x, y) {
            if !grid.contains_point(to) {
                continue;
            }

            if set.contains(&to) {
                continue;
            }
            if grid[to] == '.'
                || grid[to]
                    == match dir {
                        Direction4::Left => '<',
                        Direction4::Up => '^',
                        Direction4::Down => 'v',
                        Direction4::Right => '>',
                    }
            {
                let mut set = set.clone();
                set.insert(to);
                stack.push((set, to, depth + 1));
            }
        }
    }

    max
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();

    let bounds = (grid.width(), grid.height());

    let start = (1, 0);
    let end = (grid.width() - 2, grid.height() - 1);

    let mut graph = HashMap::<(usize, usize), BTreeMap<(usize, usize), usize>>::new();

    let mut stack = Vec::new();
    let mut visited = HashSet::new();
    stack.push(start);
    visited.insert(start);

    while let Some(from) = stack.pop() {
        for (_, dir) in DIRECTIONS4D {
            let mut to = from;
            'grid: while grid[to] != '#' {
                let t = dir.moveub(to, bounds);
                match t {
                    Some(t) => {
                        if grid[t] == '#' {
                            break;
                        } else {
                            for (a, d) in adjacent_4_ud(t.0, t.1) {
                                if !grid.contains_point(a) {
                                    continue;
                                }
                                if dir == d || dir == d.opposite() {
                                    continue;
                                }
                                if grid[a] != '#' {
                                    to = t;
                                    break 'grid;
                                }
                            }
                            to = t;
                        }
                    }
                    None => break,
                }
            }
            if to != from {
                graph
                    .entry(from)
                    .or_default()
                    .insert(to, manhattan_u(from, to));
                if !visited.contains(&to) {
                    stack.push(to);
                    visited.insert(to);
                }
            }
        }
    }

    'simplification: loop {
        for (key, val) in graph.clone().iter() {
            if val.len() == 2 {
                let mut i = val.iter();
                let (a, d1) = i.next().unwrap();
                let (b, d2) = i.next().unwrap();
                let dist = d1 + d2;

                //println!("{a:?} {key:?} {b:?}");

                graph.remove(key);

                graph.get_mut(&a).unwrap().remove(key);
                graph.get_mut(&a).unwrap().insert(*b, dist);
                graph.get_mut(&b).unwrap().remove(key);
                graph.get_mut(&b).unwrap().insert(*a, dist);

                continue 'simplification;
            }
        }

        break;
    }

    let mut max = 0;

    let mut stack = Vec::new();
    let mut set = HashSet::new();
    set.insert(start);
    stack.push((set, start, 0));

    while let Some((set, from, depth)) = stack.pop() {
        if from == end {
            max = max.max(depth);
        }

        let mut todo = BinaryHeap::new();

        for (to, dist) in &graph[&from] {
            let to = *to;
            if !grid.contains_point(to) {
                continue;
            }

            if set.contains(&to) {
                continue;
            }
            todo.push((-(depth as isize + *dist as isize), to, dist));
        }
        for (_, to, dist) in todo {
            let mut set = set.clone();
            set.insert(to);
            stack.push((set, to, depth + dist));
        }
    }

    max
}
