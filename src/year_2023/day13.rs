use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input
        .split("\n\n")
        .map(|l| {
            let grid = l.parse::<Grid<char>>().unwrap();

            let mut verts = 0;
            let mut horz = 0;

            'col: for col in 1..grid.width() {
                let mut left = col - 1;
                let mut right = col;
                loop {
                    if grid.get_col(left) != grid.get_col(right) {
                        break;
                    }
                    if left == 0 || right == grid.width() - 1 {
                        verts += col;
                        break 'col;
                    }
                    left -= 1;
                    right += 1;
                }
            }

            'row: for row in 1..grid.height() {
                let mut left = row - 1;
                let mut right = row;
                loop {
                    if grid.get_row(left) != grid.get_row(right) {
                        break;
                    }
                    if left == 0 || right == grid.height() - 1 {
                        horz += row;
                        break 'row;
                    }
                    left -= 1;
                    right += 1;
                }
            }

            verts + horz * 100
        })
        .sum::<usize>()
}

#[test]
fn test() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
    let output = 404;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    input
        .split("\n\n")
        .map(|l| {
            let mut grid = l.parse::<Grid<char>>().unwrap();

            let mut verts = None;
            let mut horz = None;

            'col: for col in 1..grid.width() {
                let mut left = col - 1;
                let mut right = col;
                loop {
                    if grid.get_col(left) != grid.get_col(right) {
                        break;
                    }
                    if left == 0 || right == grid.width() - 1 {
                        verts = Some(col);
                        break 'col;
                    }
                    left -= 1;
                    right += 1;
                }
            }

            'row: for row in 1..grid.height() {
                let mut left = row - 1;
                let mut right = row;
                loop {
                    if grid.get_row(left) != grid.get_row(right) {
                        break;
                    }
                    if left == 0 || right == grid.height() - 1 {
                        horz = Some(row);
                        break 'row;
                    }
                    left -= 1;
                    right += 1;
                }
            }

            for y in 0..grid.height() {
                'next: for x in 0..grid.width() {
                    grid[(x, y)] = match grid[(x, y)] {
                        '.' => '#',
                        '#' => '.',
                        a => a,
                    };

                    let mut refls = 0;
                    let mut colr = 0;
                    let mut rowr = 0;

                    for col in 1..grid.width() {
                        let mut left = col - 1;
                        let mut right = col;
                        loop {
                            if grid.get_col(left) != grid.get_col(right) {
                                break;
                            }
                            if (left == 0 || right == grid.width() - 1) && Some(col) != verts {
                                if refls > 0 {
                                    grid[(x, y)] = match grid[(x, y)] {
                                        '.' => '#',
                                        '#' => '.',
                                        a => a,
                                    };
                                    continue 'next;
                                }
                                colr = col;
                                refls += 1;
                            }
                            left -= 1;
                            right += 1;
                        }
                    }

                    for row in 1..grid.height() {
                        let mut left = row - 1;
                        let mut right = row;
                        loop {
                            if grid.get_row(left) != grid.get_row(right) {
                                break;
                            }
                            if (left == 0 || right == grid.height() - 1) && Some(row) != horz {
                                if refls > 0 {
                                    grid[(x, y)] = match grid[(x, y)] {
                                        '.' => '#',
                                        '#' => '.',
                                        a => a,
                                    };
                                    continue 'next;
                                }
                                rowr = row;
                                refls += 1;
                            }
                            left -= 1;
                            right += 1;
                        }
                    }

                    if refls > 0 {
                        println!("{grid}");
                        println!("{colr} {rowr}");
                    }

                    grid[(x, y)] = match grid[(x, y)] {
                        '.' => '#',
                        '#' => '.',
                        a => a,
                    };
                    if refls == 0 {
                        continue;
                    }
                    return colr + rowr * 100;
                }
            }

            0
        })
        .sum::<usize>()
}
