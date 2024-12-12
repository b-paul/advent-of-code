use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();

    let mut searched = HashSet::new();
    let mut total = 0;

    for (sq, c) in grid.iter_idx() {
        let mut area = 0;
        let mut peri = 0;
        if searched.contains(&sq) {
            continue;
        }
        grid.dfs_4(
            sq,
            |p, _| {
                area += 1;
                searched.insert(p);
                for d in Direction4::dir_list() {
                    if let Some(p) = p.move_off(d.offset()) {
                        if !grid.contains_point(p) {
                            peri += 1;
                        } else if grid.get(p) != Some(c) {
                            peri += 1;
                        }
                    } else {
                        peri += 1;
                    }
                }
            },
            |_, f, t| f == t,
        );
        total += area * peri;
    }

    total
}

#[test]
fn test() {
    let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
    let output = 368;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

fn check(grid: &Grid<char>, p: Point, c: &char, d: Direction4) -> bool {
    if let Some(p) = p.move_off(d.offset()) {
        if !grid.contains_point(p) {
            true
        } else if grid.get(p) != Some(c) {
            true
        } else {
            false
        }
    } else {
        true
    }
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();

    let mut searched = HashSet::new();
    let mut total = 0;

    for (sq, c) in grid.iter_idx() {
        let mut area = 0;
        let mut sides = 0;
        if searched.contains(&sq) {
            continue;
        }
        let mut side_checked = HashSet::new();
        grid.dfs_4(
            sq,
            |p, _| {
                area += 1;
                searched.insert(p);
                for d in Direction4::dir_list() {
                    if side_checked.contains(&(p, d)) {
                        continue;
                    }
                    if check(&grid, p, c, d) {
                        let mut point = grid.point(p).unwrap();
                        while let Some(p2) = point.move_dir(d.cw()) {
                            if p2.val() != c {
                                break;
                            }
                            if !check(&grid, p2.pos(), c, d) {
                                break;
                            }
                            point = p2;
                        }
                        side_checked.insert((point.pos(), d));
                        while let Some(p2) = point.move_dir(d.acw()) {
                            if p2.val() != c {
                                break;
                            }
                            if !check(&grid, p2.pos(), c, d) {
                                break;
                            }
                            point = p2;
                            side_checked.insert((point.pos(), d));
                        }
                        side_checked.insert((point.pos(), d));
                        sides += 1;
                    }
                }
            },
            |_, f, t| f == t,
        );
        total += area * sides;
    }

    total
}
