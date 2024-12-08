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
                .filter(|&dir| {
                    grid.point(p.pair())
                        .expect("This point better be in bounds")
                        .trajectory_dir(dir, 4)
                        .copied()
                        .eq("XMAS".chars())
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
            DirectionDiag4::dir_list()
                .into_iter()
                .filter(|&dir| {
                    [dir, dir.cw()].into_iter().all(|dir| {
                        grid.point(p.pair())
                            .expect("This point better be in bounds")
                            .move_dir(dir)
                            .is_some_and(|p| {
                                p.trajectory_dir(dir.opposite(), 3).copied().eq("MAS".chars())
                            })
                    })
                })
                .count()
        })
        .sum::<usize>()
}
