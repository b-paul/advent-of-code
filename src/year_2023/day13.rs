use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input
        .split("\n\n")
        .map(|l| {
            let mut grid = l.parse::<Grid<char>>().unwrap();

            for mult in [100, 1] {
                for row in 1..grid.height() {
                    let mut left = row - 1;
                    let mut right = row;
                    loop {
                        if grid.get_row(left) != grid.get_row(right) {
                            break;
                        }
                        if left == 0 || right == grid.height() - 1 {
                            return row * mult;
                        }
                        left -= 1;
                        right += 1;
                    }
                }

                grid = grid.transpose();
            }

            panic!("no line of symmetry found")
        })
        .sum::<usize>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    input
        .split("\n\n")
        .map(|l| {
            let mut grid = l.parse::<Grid<char>>().unwrap();

            for mult in [100, 1] {
                for row in 1..grid.height() {
                    let mut left = row - 1;
                    let mut right = row;
                    let mut wrong = 0;
                    loop {
                        wrong += grid
                            .get_row(left)
                            .unwrap()
                            .iter()
                            .zip(grid.get_row(right).unwrap().iter())
                            .filter(|(a, b)| a != b)
                            .count();
                        if left == 0 || right == grid.height() - 1 || wrong > 1 {
                            break;
                        }
                        left -= 1;
                        right += 1;
                    }
                    if wrong == 1 {
                        return row * mult;
                    }
                }

                grid = grid.transpose();
            }

            panic!("no line of symmetry found")
        })
        .sum::<usize>()
}

#[cfg(test)]
mod benches {
    use crate::get_input;
    use crate::year_2023::day13::*;
    use test::{black_box, Bencher};

    #[bench]
    fn part1_normal(b: &mut Bencher) {
        let input = &get_input(2023, 13).unwrap();
        b.iter(|| {
            black_box(part_1(input));
        })
    }

    #[bench]
    fn part2_normal(b: &mut Bencher) {
        let input = &get_input(2023, 13).unwrap();
        b.iter(|| {
            black_box(part_2(input));
        })
    }
}
