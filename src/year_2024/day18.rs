use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let points = input.lines().map(read_point).collect_vec();

    let bounds = Bounds::max_bounds(&points);

    let mut grid = Grid::new_filled(false, bounds);
    let exit = bounds.bottom_right();

    // Change 1024 to 12 when running the test :(
    grid.load_points(&points[..1024], true);

    let mut best_score = None;
    grid.bfs_4(
        bounds.top_left(),
        |p, _, depth| best_score = best_score.or((p == exit).then_some(depth)),
        |_, _, to| !to,
    );

    best_score.unwrap()
}

#[allow(unused)]
fn print_board(occ: [u128; 71], visited: [u128; 71]) {
    for y in 0..=70 {
        for x in 0..=70 {
            if occ[y] & (1 << x) == 0 {
                print!("O");
            } else if visited[y] & (1 << x) != 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
    println!();
}

pub fn part_1_faster(input: &str) -> impl std::fmt::Display {
    const ROW_MASK: u128 = (1 << 71) - 1;
    let mut occ = [!0u128; 71];
    let mut visited = [0u128; 71];
    let mut visited2 = [0u128; 71];

    for p in input.lines().map(read_point).take(1024) {
        occ[p.y] &= !(1 << p.x);
    }

    let mut count = 0;
    visited[0] = 1;
    while visited[70] & (1 << 70) == 0 {
        count += 1;

        for row in 0..=70 {
            visited2[row] = (visited[row] | visited[row] >> 1 | visited[row] << 1) & ROW_MASK;
            if row != 0 {
                visited2[row] |= visited[row - 1]
            }
            if row != 70 {
                visited2[row] |= visited[row + 1]
            }
            visited2[row] &= occ[row];
        }
        visited = visited2;
    }

    count
}

#[test]
fn test() {
    let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    let output = 117440;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let points = input.lines().map(read_point).collect_vec();

    let bounds = Bounds::max_bounds(&points);

    let mut grid = Grid::new_filled(false, bounds);
    let exit = bounds.bottom_right();

    for d in 0..points.len() {
        grid[points[d]] = true;
        let mut ok = false;
        grid.bfs_4(bounds.top_left(), |p, _, _| ok |= p == exit, |_, _, to| !to);
        if !ok {
            let p = points[d];
            return format!("{},{}", p.x, p.y);
        }
    }

    panic!()
}

#[cfg(test)]
mod benches {
    use crate::get_input;
    use crate::year_2024::day18::*;
    use test::{black_box, Bencher};

    #[bench]
    fn part1_normal(b: &mut Bencher) {
        let input = &get_input(2024, 18).unwrap();
        b.iter(|| {
            black_box(part_1(input));
        })
    }

    #[bench]
    fn part1_faster(b: &mut Bencher) {
        let input = &get_input(2024, 18).unwrap();
        assert_eq!(part_1_faster(input).to_string(), part_1(input).to_string());
        b.iter(|| {
            black_box(part_1_faster(input));
        })
    }
}
