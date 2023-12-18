use std::collections::*;
use itertools::Itertools;
use crate::helper::prelude::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let size = (1000, 1000);

    let mut grid = Grid::new_filled('.', size.0, size.1);

    let mut pos = (size.0 / 2, size.1 / 2);

    for line in input.lines() {
        let is = line.split(' ').collect_vec();
        let (dir, count, _) = (is[0], is[1].parse::<usize>().unwrap(), is[2]);
        let dir = match dir.chars().next().unwrap() {
            'R' => Direction4::Right,
            'L' => Direction4::Left,
            'U' => Direction4::Up,
            'D' => Direction4::Down,
            d => panic!("Invalid direction {d}"),
        };
        for _ in 0..count {
            grid[pos] = '#';
            pos = dir.moveub(pos, size).unwrap();
            grid[pos] = '#';
        }
    }

    grid.floodfill_4('.', '8', (0, 0));

    grid.iter_idx().filter(|(_, &c)| c != '8').count()
}

#[test]
fn test() {
    let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";
    let output = 952408144115usize;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut vertices = Vec::new();

    let mut cur = (0, 0);
    vertices.push(cur);

    for line in input.lines() {
        let is = line.split(' ').collect_vec();
        let (_, _, col) = (is[0], is[1].parse::<usize>().unwrap(), is[2]);
        let dir = &col[7..8];
        let dir = match dir.chars().next().unwrap() {
            '0' => Direction4::Right,
            '2' => Direction4::Left,
            '3' => Direction4::Up,
            '1' => Direction4::Down,
            d => panic!("Invalid direction {d}"),
        };
        let mut colour = 0;
        let col = &col[2..7];
        for c in col.chars() {
            colour *= 16;
            colour += c.to_digit(16).unwrap() as isize;
        }

        cur = dir.moveic(cur, colour);

        vertices.push(cur);
    }

    let mut area = 0;
    let n = vertices.len();
    for i in 0..vertices.len() {
        let ip = (n + i + 1) % n;
        // shoelace formula
        area += vertices[i].0 * vertices[ip].1 - vertices[ip].0 * vertices[i].1;

        // Add the edges
        area += vertices[i].0.abs_diff(vertices[ip].0) as isize;
        area += vertices[i].1.abs_diff(vertices[ip].1) as isize;
    }
    // abs diff misses one corner, but then the edges get joined together so only one corner is
    // missed overall (i think ?!?!)
    area / 2 + 1
}
