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
        |_, _, to| to,
    );

    best_score.unwrap()
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
        grid.bfs_4(bounds.top_left(), |p, _, _| ok |= p == exit, |_, _, to| to);
        if !ok {
            println!("{}", points[d]);
            return "manual submit today :(".to_string();
        }
    }

    "manual submit today :(".to_string()
}
