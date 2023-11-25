use itertools::Itertools;

fn push1(maze: &[[i8; 300]; 300], pos: &mut (usize, usize), facing: usize) {
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
                push1(&maze, &mut pos, facing);
                if maze[pos.1][pos.0] == 1 {
                    // go backwards one
                    push1(&maze, &mut pos, (facing + 2) % 4);
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
        push1(&maze, &mut pos, facing);
        if maze[pos.1][pos.0] == 1 {
            // go backwards one
            push1(&maze, &mut pos, (facing + 2) % 4);
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

fn push2(pos: &mut (isize, isize), facing: &mut usize) {
    let face = match pos.1 {
        0..50 => match pos.0 {
            50..100 => 1,
            100..150 => 2,
            _ => unreachable!()
        },
        50..100 => 3,
        100..150 => match pos.0 {
            0..50 => 5,
            50..100 => 4,
            _ => unreachable!()
        }
        150..200 => 6,
        _ => unreachable!()
    };

    match facing {
        0 => pos.0 += 1,
        1 => pos.1 += 1,
        2 => pos.0 -= 1,
        3 => pos.1 -= 1,
        _ => (),
    }

    match face {
        1 => {
            let rel_x = pos.0 - 50;
            let rel_y = pos.1;
            if rel_x < 0 {
                // move to 5
                pos.0 = 0;
                pos.1 = 149 - rel_y;
                *facing = 0;
            }
            if rel_y < 0 {
                // move to 6
                pos.0 = 0;
                pos.1 = 150 + rel_x;
                *facing = 0;
            }
        },
        2 => {
            let rel_x = pos.0 - 100;
            let rel_y = pos.1;
            if rel_y < 0 {
                // move to 6
                pos.0 = rel_x;
                pos.1 = 199;
                *facing = 3;
            }
            if rel_x >= 50 {
                // move to 4
                pos.0 = 99;
                pos.1 = 149 - rel_y;
                *facing = 2;
            }
            if rel_y >= 50 {
                // move to 3
                pos.0 = 99;
                pos.1 = 50 + rel_x;
                *facing = 2;
            }
        },
        3 => {
            let rel_x = pos.0 - 50;
            let rel_y = pos.1 - 50;
            if rel_x < 0 {
                // move to 5
                pos.0 = rel_y;
                pos.1 = 100;
                *facing = 1;
            }
            if rel_x >= 50 {
                // move to 2
                pos.0 = 100 + rel_y;
                pos.1 = 49;
                *facing = 3;
            }
        },
        4 => {
            let rel_x = pos.0 - 50;
            let rel_y = pos.1 - 100;
            if rel_x >= 50 {
                // move to 2
                pos.0 = 149;
                pos.1 = 49 - rel_y;
                *facing = 2;
            }
            if rel_y >= 50 {
                // move to 6
                pos.0 = 49;
                pos.1 = 150 + rel_x;
                *facing = 2;
            }
        },
        5 => {
            let rel_x = pos.0;
            let rel_y = pos.1 - 100;
            if rel_x < 0 {
                // move to 1
                pos.0 = 50;
                pos.1 = 49 - rel_y;
                *facing = 0;
            }
            if rel_y < 0 {
                // move to 3
                pos.0 = 50;
                pos.1 = 50 + rel_x;
                *facing = 0;
            }
        },
        6 => {
            let rel_x = pos.0;
            let rel_y = pos.1 - 150;
            if rel_x < 0 {
                // move to 1
                pos.0 = 50 + rel_y;
                pos.1 = 0;
                *facing = 1
            }
            if rel_x >= 50 {
                // move to 4
                pos.0 = 50 + rel_y;
                pos.1 = 149;
                *facing = 3;
            }
            if rel_y >= 50 {
                // move to 2
                pos.0 = 100 + rel_x;
                pos.1 = 0;
                *facing = 1;
            }
        },
        _ => unreachable!()
    }
}

pub fn part_2(input: String) -> String {
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
            pos.0 = x as isize;
            break;
        }
    }
    let mut facing = 0;
    let mut mv = 0;
    for ch in inputs.bytes() {
        if ch == b'L' || ch == b'R' {
            // Move
            for _ in 0..mv {
                push2(&mut pos, &mut facing);
                if maze[pos.1 as usize][pos.0 as usize] == 1 {
                    // go backwards one
                    facing ^= 2;
                    push2(&mut pos, &mut facing);
                    facing ^= 2;
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
        push2(&mut pos, &mut facing);
        if maze[pos.1 as usize][pos.0 as usize] == 1 {
            // go backwards one
            facing ^= 2;
            push2(&mut pos, &mut facing);
            facing ^= 2;
            break;
        }
    }

    ((pos.1 + 1) * 1000 + (pos.0 + 1) * 4 + facing as isize).to_string()
}

#[test]
fn testp2() {
    /*
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
        */
}
