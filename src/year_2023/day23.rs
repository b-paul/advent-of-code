use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();

    let end = Point {
        x: grid.width() - 2,
        y: grid.height() - 1,
    };

    let mut max = 0;

    let mut stack = Vec::new();
    let mut set = HashSet::new();
    set.insert(Point { x: 1, y: 0 });
    stack.push((set, Point { x: 1, y: 0 }, 0));

    while let Some((set, p, depth)) = stack.pop() {
        if p == end {
            max = max.max(depth);
        }

        for (to, dir) in adjacent_4_ud(p.x, p.y) {
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
    let start = Point { x: 1, y: 0 };

    let view = grid.graph_view_4(|_, _, to| to == '#');

    view.collapse_count(start)
        .write_graphviz_undirected(vec![start].into_iter(), "graph.dot")
        .unwrap();

    "todo"
    /*
    let bounds = grid.bounds();

    let start = Point { x: 1, y: 0 };
    let end = Point {
        x: grid.width() - 2,
        y: grid.height() - 1,
    };

    let mut graph = HashMap::<Point, BTreeMap<Point, usize>>::new();

    let mut stack = Vec::new();
    let mut visited = HashSet::new();
    stack.push(start);
    visited.insert(start);

    while let Some(from) = stack.pop() {
        for (_, mut dir) in DIRECTIONS4D {
            let Some(mut to) = dir.moveub(from, bounds) else {
                continue;
            };
            if grid.get(to) == Some(&'#') {
                continue;
            }
            let mut dist = 1;
            loop {
                let mut nexts = adjacent_4_ud(to.x, to.y).into_iter().filter(|&(p, dir2)| {
                    dir2 != dir.opposite() && grid.contains_point(p) && grid[p] != '#'
                });
                let Some((next_to, next_dir)) = nexts.next() else {
                    break;
                };
                if nexts.next().is_some() {
                    break;
                }
                to = next_to;
                dir = next_dir;
                dist += 1;
            }
            graph.entry(from).or_default().insert(to, dist);
            if !visited.contains(&to) {
                stack.push(to);
                visited.insert(to);
            }
        }
    }

    /*
    println!("graph {{");
    for (v1, map) in graph.iter() {
        print!("\"{v1:?}\" -- {{ ");
        for (v2, _) in map.iter() {
            print!("\"{v2:?}\" ");
        }
        println!("}}");
    }
    */
    // Conclusion:
    // This graph is tiled as a lattice! so like
    // o - o - o - o
    // |   |   |   |
    // o - o - o - o
    // |   |   |   |
    // o - o - o - o
    // |   |   |   |
    // o - o - o - o
    // where o are vertices and - or | are edges
    // on the actual input, it was 6x6
    // also the corners get simplified but whatever

    let mut max = 0;

    let mut stack = Vec::new();
    let mut set = HashSet::new();
    set.insert(start);
    stack.push((set, start, 0));

    let mut ops = 0;

    while let Some((set, from, depth)) = stack.pop() {
        ops += 1;
        dbg!(ops);
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
        */
}
