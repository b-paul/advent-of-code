use std::collections::HashSet;
use std::collections::VecDeque;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut startpos = (0, 0);
    let mut bestpos = (0, 0);
    let mut grid = vec![];
    for (y, iline) in input.lines().enumerate() {
        let mut line = vec![];
        for (x, byte) in iline.bytes().enumerate() {
            if byte == 10 {
                continue;
            }

            if byte == b'S' {
                startpos = (x, y);
                line.push(0);
            } else if byte == b'E' {
                bestpos = (x, y);
                line.push(25);
            } else {
                line.push((byte - b'a') as isize);
            }
        }
        grid.push(line);
    }

    let mut stack = VecDeque::new();
    let mut visited = HashSet::new();
    stack.push_back((startpos, 0));

    let mut bdepth = 0;
    while let Some(((x, y), depth)) = stack.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }

        if (x, y) == bestpos {
            bdepth = depth;
            break;
        }

        visited.insert((x, y));

        if x != 0 && grid[y][x] - grid[y][x - 1] >= -1 {
            stack.push_back(((x - 1, y), depth + 1));
        }
        if x != (grid[0].len() - 1) && grid[y][x] - grid[y][x + 1] >= -1 {
            stack.push_back(((x + 1, y), depth + 1));
        }
        if y != 0 && grid[y][x] - grid[y - 1][x] >= -1 {
            stack.push_back(((x, y - 1), depth + 1));
        }
        if y != (grid.len() - 1) && grid[y][x] - grid[y + 1][x] >= -1 {
            stack.push_back(((x, y + 1), depth + 1));
        }
    }

    bdepth.to_string()
}

#[test]
fn testp1() {
    assert_eq!(
        part_1(
            "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
        )
        .to_string(),
        31.to_string()
    );
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut bestpos = (0, 0);
    let mut grid = vec![];
    for (y, iline) in input.lines().enumerate() {
        let mut line = vec![];
        for (x, byte) in iline.bytes().enumerate() {
            if byte == 10 {
                continue;
            }

            if byte == b'S' {
                line.push(0);
            } else if byte == b'E' {
                bestpos = (x, y);
                line.push(25);
            } else {
                line.push((byte - b'a') as isize);
            }
        }
        grid.push(line);
    }

    let mut stack = VecDeque::new();
    let mut visited = HashSet::new();
    stack.push_back((bestpos, 0));

    let mut bdepth = 0;
    while let Some(((x, y), depth)) = stack.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }

        if grid[y][x] == 0 {
            bdepth = depth;
            break;
        }

        visited.insert((x, y));

        if x != 0 && grid[y][x] - grid[y][x - 1] <= 1 {
            stack.push_back(((x - 1, y), depth + 1));
        }
        if x != (grid[0].len() - 1) && grid[y][x] - grid[y][x + 1] <= 1 {
            stack.push_back(((x + 1, y), depth + 1));
        }
        if y != 0 && grid[y][x] - grid[y - 1][x] <= 1 {
            stack.push_back(((x, y - 1), depth + 1));
        }
        if y != (grid.len() - 1) && grid[y][x] - grid[y + 1][x] <= 1 {
            stack.push_back(((x, y + 1), depth + 1));
        }
    }

    bdepth.to_string()
}

#[test]
fn testp2() {
    assert_eq!(
        part_2(
            "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
        )
        .to_string(),
        29.to_string()
    );
}
