use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

fn try_push(grid: &mut Grid<char>, point: Point, dir: Direction4, replace: char) -> Option<()> {
    match grid[point] {
        '.' => Some(()),
        '#' => None,
        _ => {
            let next = point.move_dir(dir)?;
            try_push2(grid, next, dir, grid[point])?;
            grid[next] = grid[point];
            grid[point] = replace;
            Some(())
        }
    }
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut input = input.split("\n\n");
    let mut grid = input.next().unwrap().parse::<Grid<char>>().unwrap();
    let moves = input
        .next()
        .unwrap()
        .chars()
        .flat_map(read_dir)
        .collect_vec();

    let mut robot = grid.find(&'@').unwrap();

    for dir in moves {
        let mut new_grid = grid.clone();
        if try_push(&mut new_grid, robot, dir, '.').is_some() {
            robot = robot.move_off(dir.offset()).unwrap();
            grid = new_grid;
        }
    }

    println!("{grid}");

    grid.iter_idx()
        .filter_map(|(p, &c)| (c == 'O').then_some(p.y * 100 + p.x))
        .sum::<usize>()
}

#[test]
fn test() {
    let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
    let output = 9021;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

fn try_push2(grid: &mut Grid<char>, point: Point, dir: Direction4, replace: char) -> Option<()> {
    match grid[point] {
        '.' => Some(()),
        '#' => None,
        _ => {
            let next = point.move_dir(dir)?;
            match dir {
                Direction4::Up | Direction4::Down => {
                    if let Some(dir2) = match grid[next] {
                        '[' => Some(Direction4::Right),
                        ']' => Some(Direction4::Left),
                        _ => None,
                    } {
                        try_push2(grid, point.move_dir(dir2)?.move_dir(dir)?, dir, '.')?;
                    }
                }
                Direction4::Right | Direction4::Left => {}
            }
            try_push2(grid, next, dir, grid[point])?;
            grid[next] = grid[point];
            grid[point] = replace;
            Some(())
        }
    }
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut input = input.split("\n\n");
    let read_grid = input.next().unwrap().parse::<Grid<char>>().unwrap();
    let moves = input
        .next()
        .unwrap()
        .chars()
        .flat_map(read_dir)
        .collect_vec();

    let mut grid = Grid::new_filled(
        '.',
        Bounds {
            width: read_grid.width() * 2,
            height: read_grid.height(),
        },
    );

    for (p, &c) in read_grid.iter_idx() {
        let p2 = Point { x: p.x * 2, ..p };
        let p3 = Point { x: p2.x + 1, ..p2 };
        let s = match c {
            '#' => ['#', '#'],
            'O' => ['[', ']'],
            '.' => ['.', '.'],
            '@' => ['@', '.'],
            _ => panic!(),
        };
        grid[p2] = s[0];
        grid[p3] = s[1];
    }

    let mut robot = grid.find(&'@').unwrap();

    for dir in moves {
        let mut new_grid = grid.clone();
        if try_push2(&mut new_grid, robot, dir, '.').is_some() {
            robot = robot.move_off(dir.offset()).unwrap();
            grid = new_grid;
        }
    }

    println!("{grid}");

    grid.iter_idx()
        .filter_map(|(p, &c)| (c == '[').then_some(p.y * 100 + p.x))
        .sum::<usize>()
}
