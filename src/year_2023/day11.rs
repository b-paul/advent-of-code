use crate::helper::grid::Grid;
use crate::helper::point::{left_pu, manhattan_u, right_pu};

fn solve<const GROWTH: usize>(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();

    let empty_rows = grid
        .iter_rows()
        .map(|mut r| r.all(|&c| c == '.'))
        .collect::<Vec<_>>();
    let empty_cols = grid
        .iter_cols()
        .map(|mut r| r.all(|&c| c == '.'))
        .collect::<Vec<_>>();

    let galaxies = grid
        .iter_idx()
        .filter(|(_, &c)| c == '#')
        .map(|(p, _)| p)
        .collect::<Vec<_>>();

    let mut r = 0;

    for (i, p1) in galaxies.iter().copied().enumerate() {
        for (j, p2) in galaxies.iter().copied().enumerate() {
            if i != j {
                let (xl, yl) = left_pu(p1.pair(), p2.pair());
                let (xr, yr) = right_pu(p1.pair(), p2.pair());
                r += (xl + 1..xr).filter(|x| empty_cols[*x]).count() * (GROWTH - 1);
                r += (yl + 1..yr).filter(|y| empty_rows[*y]).count() * (GROWTH - 1);

                r += manhattan_u(p1.pair(), p2.pair());
            }
        }
    }

    r / 2
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    solve::<2>(input)
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    solve::<1000000>(input)
}
