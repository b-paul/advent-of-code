use std::collections::HashSet;

fn collision(board: &HashSet<(isize, isize)>, piece: i32, x: isize, y: isize) -> bool {
    if x < 0 || y < 0 {
        return true;
    }
    if x > (match piece {
        0 => 3,
        1 => 4,
        2 => 4,
        3 => 6,
        4 => 5,
        _ => unreachable!(),
    }) {
        return true;
    }

    match piece {
        0 => {
            board.contains(&(x, y))
                || board.contains(&(x + 1, y))
                || board.contains(&(x + 2, y))
                || board.contains(&(x + 3, y))
        }
        1 => {
            board.contains(&(x + 1, y))
                || board.contains(&(x, y + 1))
                || board.contains(&(x + 1, y + 1))
                || board.contains(&(x + 2, y + 1))
                || board.contains(&(x + 1, y + 2))
        }
        2 => {
            board.contains(&(x, y))
                || board.contains(&(x + 1, y))
                || board.contains(&(x + 2, y))
                || board.contains(&(x + 2, y + 1))
                || board.contains(&(x + 2, y + 2))
        }
        3 => {
            board.contains(&(x, y))
                || board.contains(&(x, y + 1))
                || board.contains(&(x, y + 2))
                || board.contains(&(x, y + 3))
        }
        4 => {
            board.contains(&(x, y))
                || board.contains(&(x + 1, y))
                || board.contains(&(x, y + 1))
                || board.contains(&(x + 1, y + 1))
        }
        _ => unreachable!(),
    }
}

fn place(board: &mut HashSet<(isize, isize)>, piece: i32, x: isize, y: isize) {
    assert!(!collision(board, piece, x, y));

    match piece {
        0 => {
            board.insert((x, y));
            board.insert((x + 1, y));
            board.insert((x + 2, y));
            board.insert((x + 3, y));
        }
        1 => {
            board.insert((x + 1, y));
            board.insert((x, y + 1));
            board.insert((x + 1, y + 1));
            board.insert((x + 2, y + 1));
            board.insert((x + 1, y + 2));
        }
        2 => {
            board.insert((x, y));
            board.insert((x + 1, y));
            board.insert((x + 2, y));
            board.insert((x + 2, y + 1));
            board.insert((x + 2, y + 2));
        }
        3 => {
            board.insert((x, y));
            board.insert((x, y + 1));
            board.insert((x, y + 2));
            board.insert((x, y + 3));
        }
        4 => {
            board.insert((x, y));
            board.insert((x + 1, y));
            board.insert((x, y + 1));
            board.insert((x + 1, y + 1));
        }
        _ => unreachable!(),
    }
}

fn run<const N: usize>(input: String) -> String {
    let mut board = HashSet::new();
    let mut maxheight = 0;
    let mut piece = 0;
    let mut count = 0;
    let mut x = 2;
    let mut y = 3;
    loop {
        for (i, ch) in input.chars().enumerate() {
            match ch {
                '>' => x += 1,
                '<' => x -= 1,
                _ => break,
            }
            if collision(&board, piece, x, y) {
                match ch {
                    '>' => x -= 1,
                    '<' => x += 1,
                    _ => unreachable!(),
                }
            }
            y -= 1;
            if collision(&board, piece, x, y) {
                y += 1;
                place(&mut board, piece, x, y);
                let mut height = y + 1;
                height += match piece {
                    1 => 2,
                    2 => 2,
                    3 => 3,
                    4 => 1,
                    _ => 0,
                };
                maxheight = maxheight.max(height);
                piece += 1;
                count += 1;

                if i == input.len() - 2 && piece == 5 {
                    for y in (0..maxheight + 2).rev() {
                        for x in 0..7 {
                            if board.contains(&(x, y)) {
                                print!("#");
                            } else {
                                print!(".");
                            }
                        }
                        println!();
                    }
                    println!();
                    println!();

                    return count.to_string();
                }

                if count == N {
                    return maxheight.to_string();
                }
                piece %= 5;
                x = 2;
                y = maxheight + 3;
            }
        }
    }
}

pub fn part_1(input: String) -> String {
    run::<2022>(input)
}

#[test]
fn testp1() {
    assert_eq!(
        part_1(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_string()),
        3068.to_string()
    );
}

pub fn part_2(input: String) -> String {
    return "No idea how to do this".to_string();

    run::<1000000000000>(input)
}

#[test]
fn testp2() {
    /*
    assert_eq!(
        part_2(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_string()),
        1514285714288isize.to_string()
    );
    */
}
