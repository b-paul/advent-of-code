use std::cmp::min;
use std::collections::BTreeSet;

fn solve_p1(line: String) -> u64 {
    // 1 starting at the currect house (0,0)
    let mut total = 1;

    let mut x: i64 = 0;
    let mut y: i64 = 0;

    let mut visited_set = BTreeSet::new();
    visited_set.insert((0, 0));

    for ch in line.chars() {
        match ch {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => (),
        }

        if !visited_set.contains(&(x, y)) {
            visited_set.insert((x, y));
            total += 1;
        }
    }

    total
}

#[test]
fn p1_tests() {
    assert_eq!(solve_p1(">".to_string()), 2);
    assert_eq!(solve_p1("^>v<".to_string()), 4);
    assert_eq!(solve_p1("^v^v^v^v^v".to_string()), 2);
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    solve_p1(input.lines().next().unwrap().to_string()).to_string()
}

fn solve_p2(line: String) -> u64 {
    // The starting house receives at least one present
    let mut total = 1;

    // Santa
    let mut x1: i64 = 0;
    let mut y1: i64 = 0;
    // Robo-Santa
    let mut x2: i64 = 0;
    let mut y2: i64 = 0;

    let mut santa_turn = true;

    let mut visited_set = BTreeSet::new();
    visited_set.insert((0, 0));

    for ch in line.chars() {
        match ch {
            '^' => if santa_turn { y1 += 1 } else { y2 += 1 },
            'v' => if santa_turn { y1 -= 1 } else { y2 -= 1 },
            '>' => if santa_turn { x1 += 1 } else { x2 += 1 },
            '<' => if santa_turn { x1 -= 1 } else { x2 -= 1 },
            _ => (),
        }

        if santa_turn {
            if !visited_set.contains(&(x1, y1)) {
                visited_set.insert((x1, y1));
                total += 1;
            }
        } else if !visited_set.contains(&(x2, y2)) {
            visited_set.insert((x2, y2));
            total += 1;
        }
        santa_turn = !santa_turn;
    }

    total
}

#[test]
fn p2_tests() {
    assert_eq!(solve_p2("^v".to_string()), 3);
    assert_eq!(solve_p2("^>v<".to_string()), 3);
    assert_eq!(solve_p2("^v^v^v^v^v".to_string()), 11);
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    solve_p2(input.lines().next().unwrap().to_string()).to_string()
}
