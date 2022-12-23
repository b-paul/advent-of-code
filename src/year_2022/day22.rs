use itertools::Itertools;

fn push(maze: &[[i8; 300]; 300], pos: &mut (usize, usize), facing: usize) {
    match facing {
        0 => pos.0 += 1,
        1 => pos.1 += 1,
        2 => pos.0 -= 1,
        3 => pos.1 -= 1,
        _ => (),
    }
    if pos.0 == 300 {
        pos.0 = 0;
    }
    if pos.0 == !0 {
        pos.0 = 299;
    }
    if pos.1 == 300 {
        pos.1 = 0;
    }
    if pos.1 == !0 {
        pos.1 = 299;
    }
    if maze[pos.1][pos.0] == -1 {
        match facing {
            0 => {
                for x in 0..300 {
                    if maze[pos.1][x] != -1 {
                        pos.0 = x;
                        break;
                    }
                }
            }
            1 => {
                for y in 0..300 {
                    if maze[y][pos.0] != -1 {
                        pos.1 = y;
                        break;
                    }
                }
            }
            2 => {
                for x in (0..300).rev() {
                    if maze[pos.1][x] != -1 {
                        pos.0 = x;
                        break;
                    }
                }
            }
            3 => {
                for y in (0..300).rev() {
                    if maze[y][pos.0] != -1 {
                        pos.1 = y;
                        break;
                    }
                }
            }
            _ => (),
        }
    }
}

pub fn part_1(input: String) -> String {
    let mut maze = [[-1i8; 300]; 300];

    for (y, line) in input.lines().enumerate() {
        if line.is_empty() {
            break;
        }
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' => {
                    maze[y][x] = 0;
                }
                '#' => {
                    maze[y][x] = 1;
                }
                _ => (),
            }
        }
    }
    let inputs = input.lines().last().unwrap();

    let mut pos = (0, 0);
    for (x, cell) in maze[0].iter().enumerate() {
        if cell != &-1 {
            pos.0 = x;
            break;
        }
    }
    let mut facing = 0;
    let mut mv = 0;
    for ch in inputs.bytes() {
        if ch == b'L' || ch == b'R' {
            // Move
            for _ in 0..mv {
                push(&maze, &mut pos, facing);
                if maze[pos.1][pos.0] == 1 {
                    // go backwards one
                    push(&maze, &mut pos, (facing + 2) % 4);
                    break;
                }
            }

            if ch == b'L' {
                facing += 3;
            } else {
                facing += 1;
            }
            facing %= 4;

            mv = 0;
        } else {
            mv *= 10;
            mv += ch - b'0';
        }
    }
    // do the last move
    for _ in 0..mv {
        push(&maze, &mut pos, facing);
        if maze[pos.1][pos.0] == 1 {
            // go backwards one
            push(&maze, &mut pos, (facing + 2) % 4);
            break;
        }
    }

    ((pos.1 + 1) * 1000 + (pos.0 + 1) * 4 + facing).to_string()
}

#[test]
fn testp1() {
    assert_eq!(
        part_1(
            "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"
                .to_string()
        ),
        6032.to_string()
    );
    assert_eq!(
        part_1(
            "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

LL1"
            .to_string()
        ),
        1038.to_string()
    );
}

fn push2(maze: &[Vec<Vec<i8>>; 6], pos: &mut (usize, usize, usize), facing: &mut usize) {
    match facing {
        0 => pos.0 += 1,
        1 => pos.1 += 1,
        2 => pos.0 -= 1,
        3 => pos.1 -= 1,
        _ => (),
    }
}

pub fn part_2(input: String) -> String {
    let mut net = [[-1i8; 512]; 512];

    for (y, line) in input.lines().enumerate() {
        if line.is_empty() {
            break;
        }
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' => {
                    net[y][x] = 0;
                }
                '#' => {
                    net[y][x] = 1;
                }
                _ => (),
            }
        }
    }
    let inputs = input.lines().last().unwrap();

    let mut width = 4;

    let mut maze = [vec![], vec![], vec![], vec![], vec![], vec![]];

    let mut i = 0;
    for nx in 0..6 {
        for ny in 0..6 {
            if net[ny * width][nx * width] == -1 {
                continue;
            }
            for y in 0..width {
                let mut v = vec![];
                for x in 0..width {
                    v.push(net[ny*width + y][nx*width + x]);
                }
                maze[i].push(v);
            }
            i += 1;
        }
    }
    return "Not done yet".to_string();

    println!("{:?}", maze);
    return todo!();

    let mut pos = (0, 0, 0);
    let mut facing = 0;
    let mut mv = 0;
    for ch in inputs.bytes() {
        if ch == b'L' || ch == b'R' {
            // Move
            for _ in 0..mv {
                push2(&maze, &mut pos, &mut facing);
                if maze[pos.2][pos.1][pos.0] == 1 {
                    // go backwards one
                    facing += 2;
                    facing %= 4;
                    push2(&maze, &mut pos, &mut facing);
                    facing += 2;
                    facing %= 4;
                    break;
                }
            }

            if ch == b'L' {
                facing += 3;
            } else {
                facing += 1;
            }
            facing %= 4;

            mv = 0;
        } else {
            mv *= 10;
            mv += ch - b'0';
        }
    }
    // do the last move
    for _ in 0..mv {
        push2(&maze, &mut pos, &mut facing);
        if maze[pos.2][pos.1][pos.0] == 1 {
            // go backwards one
            facing += 2;
            facing %= 4;
            push2(&maze, &mut pos, &mut facing);
            facing += 2;
            facing %= 4;
            break;
        }
    }

    ((pos.1 + 1) * 1000 + (pos.0 + 1) * 4 + facing).to_string()
}

#[test]
fn testp2() {
    assert_eq!(
        part_2(
            "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"
                .to_string()
        ),
        5031.to_string()
    );
}
