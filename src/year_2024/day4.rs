use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();
    let mut count = 0;

    for ((yb, xb), _) in grid.iter_idx() {
        const S: &str = "XMAS";

        for ((dy, dx), _) in DIRECTIONS8D {
            let (mut x, mut y) = (xb, yb);
            let mut no = false;
            for i in 0..4usize {
                if grid.get((x, y)) != S.chars().nth(i).as_ref() {
                    no = true;
                    break;
                }
                if i == 3 {
                    break;
                }
                let Some((xp, yp)) = ({
                    let (x, y) = (x as isize + dx, y as isize + dy);
                    if x < 0 || y < 0 {
                        None
                    } else {
                        Some((x as usize, y as usize))
                    }
                }) else {
                    no = true;
                    break;
                };
                x = xp;
                y = yp;
            }
            if !no {
                count += 1;
            }
        }
    }

    count
}

#[test]
fn test() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    let output = 9;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();
    let mut count = 0;

    for ((x, y), c) in grid.iter_idx() {
        if *c == 'A' {
            let s = "SSMMSSMM";
            'outer: for i in 0..4 {
                for (j, (dy, dx)) in [(1, 1), (1, -1), (-1, -1), (-1, 1)].into_iter().enumerate() {
                    let Some((x, y)) = ({
                        let (x, y) = (x as isize + dx, y as isize + dy);
                        if x < 0 || y < 0 {
                            None
                        } else {
                            Some((x as usize, y as usize))
                        }
                    }) else {
                        continue 'outer;
                    };
                    if grid.get((x, y)) != s.chars().nth(j + i).as_ref() {
                        continue 'outer;
                    }
                }
                count += 1;
            }
        }
    }

    count
}
