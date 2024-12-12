use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

// Check that this side is an edge
fn edge(grid: &Grid<char>, p: Point, d: Direction4) -> bool {
    p.move_off(d.offset())
        .is_none_or(|p2| grid.get(p) != grid.get(p2))
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();

    let mut searched = HashSet::new();

    grid.iter_idx()
        .map(|(sq, _)| {
            let mut area = 0;
            let mut perimeter = 0;
            if searched.contains(&sq) {
                return 0;
            }
            grid.dfs_4(
                sq,
                |p, _| {
                    area += 1;
                    searched.insert(p);
                    for d in Direction4::dir_list() {
                        if edge(&grid, p, d) {
                            perimeter += 1;
                        }
                    }
                },
                |_, f, t| f == t,
            );
            area * perimeter
        })
        .sum::<u64>()
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

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();

    let mut searched = HashSet::new();

    grid.iter_idx()
        .map(|(sq, c)| {
            let mut area = 0;
            let mut sides = 0;
            if searched.contains(&sq) {
                return 0;
            }
            let mut side_checked = HashSet::new();
            grid.dfs_4(
                sq,
                |p, _| {
                    area += 1;
                    searched.insert(p);
                    for d in Direction4::dir_list() {
                        if !side_checked.contains(&(p, d)) && edge(&grid, p, d) {
                            for d2 in [d.cw(), d.acw()] {
                                let mut point = grid.point(p).unwrap();
                                side_checked.insert((point.pos(), d));
                                while let Some(p2) = point.move_dir(d2) {
                                    if p2.val() != c || !edge(&grid, p2.pos(), d) {
                                        break;
                                    }
                                    point = p2;
                                    side_checked.insert((point.pos(), d));
                                }
                            }
                            sides += 1;
                        }
                    }
                },
                |_, f, t| f == t,
            );
            area * sides
        })
        .sum::<u64>()
}
