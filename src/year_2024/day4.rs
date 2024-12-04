use crate::helper::prelude::*;
use itertools::Itertools;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let grid = input
        .parse::<Grid<char>>()
        .expect("Failed to parse grid input");

    grid.iter_idx()
        .map(|(p, _)| {
            Direction8::dir_list()
                .into_iter()
                .filter(|dir| {
                    (0..4usize).all(|i| {
                        dir.moveuc(p, i)
                            .is_some_and(|p| grid.get(p) == "XMAS".chars().nth(i).as_ref())
                    })
                })
                .count()
        })
        .sum::<usize>()
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
    let grid = input
        .parse::<Grid<char>>()
        .expect("Failed to parse grid input");

    grid.iter_idx()
        .filter_map(|(p, &c)| (c == 'A').then_some(p))
        .map(|p| {
            (0..4)
                .filter(|i| {
                    DirectionDiag4::dir_list()
                        .into_iter()
                        .enumerate()
                        .all(|(j, dir)| {
                            dir.moveu(p).is_some_and(|p| {
                                grid.get(p) == "SSMMSSMM".chars().nth(j + i).as_ref()
                            })
                        })
                })
                .count()
        })
        .sum::<usize>()
}
