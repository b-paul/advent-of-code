use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();
    let mut squares = HashSet::new();

    let mut pos = grid.find(&'^').unwrap();
    let mut dir = Direction4::Up;

    loop {
        squares.insert(pos);
        let Some(posp) = dir.moveub(pos, (grid.width(), grid.height())) else {
            break;
        };
        if grid.get(posp) == Some(&'#') {
            dir = dir.cw();
        } else {
            pos = posp;
        }
    }

    squares.len()
}

#[test]
fn test() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    let output = 6;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

fn part_2_slower(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();
    let squares = {
        let mut squares = HashSet::new();

        let mut pos = grid.find(&'^').unwrap();
        let mut dir = Direction4::Up;

        loop {
            squares.insert(pos);
            let Some(posp) = dir.moveub(pos, (grid.width(), grid.height())) else {
                break;
            };
            if grid.get(posp) == Some(&'#') {
                dir = dir.cw();
            } else {
                pos = posp;
            }
        }

        squares
    };
    squares
        .into_iter()
        .filter(|pos| {
            let mut grid = grid.clone();
            grid[*pos] = '#';
            let mut squares = HashSet::new();

            let Some(mut pos) = grid.find(&'^') else {
                return false;
            };
            let mut dir = Direction4::Up;

            loop {
                if squares.contains(&(pos, dir)) {
                    return true;
                }
                squares.insert((pos, dir));
                let Some(posp) = dir.moveub(pos, (grid.width(), grid.height())) else {
                    return false;
                };
                if grid.get(posp) == Some(&'#') {
                    dir = dir.cw();
                } else {
                    pos = posp;
                }
            }
        })
        .count()
}

fn adjust_pos(
    (col, row): (usize, usize),
    dir: Direction4,
    width: usize,
    height: usize,
) -> (usize, usize) {
    match dir {
        Direction4::Up => (col, row),
        Direction4::Right => (height - 1 - row, col),
        Direction4::Down => (width - 1 - col, height - 1 - row),
        Direction4::Left => (row, width - 1 - col),
    }
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();
    // Because we know that there will not be a cycle, we can represent the path instead of the set
    // of visited squares.
    let squares = {
        let mut squares = Vec::new();

        let mut pos = grid.find(&'^').unwrap();
        let mut dir = Direction4::Up;

        loop {
            squares.push(pos);
            let Some(posp) = dir.moveub(pos, (grid.width(), grid.height())) else {
                break;
            };
            if grid.get(posp) == Some(&'#') {
                dir = dir.cw();
            } else {
                pos = posp;
            }
        }

        squares
    };

    // We compute, per square, and per direction on that square, the square that you would reach
    // after moving forwards continuously. The three cases we have to consider is when we stay put
    // by running into an obstacle, when we move out of bounds, and when we are currently an
    // obstacle. This can be represented as a Grid<[Option<(usize, usize)>; 4]. If we stay put,
    // store this square as the destination. If we are an obstacle, we store some uninitialised
    // value, and just always check beforehand. If we are moving out of bounds, store a None. In
    // this representation, finding cycles/exits is much faster. The idea is that we can place an
    // obstacle on the grid, and recompute only the row and column that that square is in.
    // Hopefully this will be faster!
    let mut move_grid: Grid<[Option<(usize, usize)>; 4]> = grid.clone().map(|_| [None; 4]);

    let width = grid.width();
    let height = grid.height();

    for dir in Direction4::dir_list() {
        // Assume we are going upwards (accounted for by rotating the grid).
        // We will scroll downwards from the top, using the square above us inductively to
        // determine where we will be after moving. The inductive hypothesis would be "The square
        // above us knows where we'll end up if we kept moving upwards", and we simply just use
        // this result.
        for col in 0..width {
            // Row 0 will always have None, since we are moving upwards (and if we are an obstacle
            // then the value doesn't matter). Thus we skip and start at row 1.
            for row in 1..height {
                let pos = (col, row);
                let adjusted_pos = adjust_pos(pos, dir, width, height);
                let square = grid
                    .get(adjusted_pos)
                    .expect("This position should be inside the grid.");
                if square == &'#' {
                    // If we are an obstacle, do nothing.
                    continue;
                }
                let above = Direction4::Up
                    .moveu(pos)
                    .expect("We should not be able to go out of bounds at row >= 1 moving up.");
                let adjusted_above = adjust_pos(above, dir, width, height);
                if grid.get(adjusted_above) == Some(&'#') {
                    // If the above square is an obstacle, we will stay put. We have to rewrite
                    // the coordinate in terms of the original unrotated grid.
                    move_grid[pos][dir as usize] = Some(adjusted_pos);
                } else {
                    // If the above square is not an obstacle, set the target to be the above
                    // square's target.
                    move_grid[pos][dir as usize] = move_grid
                        .get(above)
                        .expect("The above square should be in bounds")[dir as usize];
                }
            }
        }
        move_grid = move_grid.rotate_acw();
    }

    // Now that we have computed this new representation, we iterate over each square in the path
    // of our original search, and recompute the grid placing an obstacle on that square. We then
    // determine whether there is a cycle.
    let faster = squares
        .clone()
        .into_iter()
        .filter(|&pos| grid.get(pos) != Some(&'^'))
        .filter(|&pos| {
            assert!(grid.get(pos) == Some(&'.'));

            let mut move_grid = move_grid.clone();

            for dir in Direction4::dir_list() {
                let Some(mut point) = grid
                    .point(pos)
                    .expect("This point should be in bounds.")
                    .move_dir(dir.opposite())
                else {
                    continue;
                };
                let pos = point.pos();
                while point.val() != &'#' {
                    move_grid[point.pos()][dir as usize] = Some(pos);
                    // If we move out of bounds, break.
                    point = match point.move_dir(dir.opposite()) {
                        Some(p) => p,
                        None => break,
                    };
                }
            }

            let Some(mut pos) = grid.find(&'^') else {
                return false;
            };
            let mut dir = Direction4::Up;

            let mut visited = HashSet::new();

            loop {
                if visited.contains(&(pos, dir)) {
                    return true;
                }
                visited.insert((pos, dir));

                let new_pos = match move_grid[pos][dir as usize] {
                    Some(p) => p,
                    None => {
                        return false;
                    },
                };
                if new_pos == pos {
                    dir = dir.cw();
                } else {
                    pos = new_pos;
                }
            }
        }).collect::<HashSet<_>>();

    faster.len()
}

/*
#[cfg(test)]
mod benches {
    use crate::get_input;
    use crate::year_2024::day6::*;
    use test::{black_box, Bencher};

    #[bench]
    fn part1_normal(b: &mut Bencher) {
        let input = &get_input(2024, 6).unwrap();
        b.iter(|| {
            black_box(part_1(input));
        })
    }

    #[bench]
    fn part2_normal(b: &mut Bencher) {
        let input = &get_input(2024, 6).unwrap();
        b.iter(|| {
            black_box(part_2(input));
        })
    }
}
*/
