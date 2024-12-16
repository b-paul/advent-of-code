use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let grid = input.parse::<Grid<char>>().unwrap();

    let start = grid.find(&'S').unwrap();
    let dir = Direction4::Right;
    let end = grid.find(&'E').unwrap();

    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();

    pq.push((0, start, dir));

    while let Some((score, p, d)) = pq.pop() {
        if visited.contains(&(p, d)) {
            continue;
        }
        visited.insert((p, d));

        if p == end {
            return -score;
        }

        if p.move_dir(d).is_some_and(|p| grid[p] != '#') {
            pq.push((score - 1, p.move_dir(d).unwrap(), d));
        }
        pq.push((score - 1000, p, d.cw()));
        pq.push((score - 1000, p, d.acw()));
    }

    panic!()
}

#[test]
fn test() {
    let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
    let output = 64;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut grid = input.parse::<Grid<char>>().unwrap();

    let start = grid.find(&'S').unwrap();
    let dir = Direction4::Right;
    let end = grid.find(&'E').unwrap();

    let mut best_score = 0;

    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();

    pq.push((0, start, dir));

    while let Some((score, p, d)) = pq.pop() {
        if visited.contains(&(p, d)) {
            continue;
        }
        visited.insert((p, d));

        if p == end {
            best_score = score;
            break;
        }

        if p.move_dir(d).is_some_and(|p| grid[p] != '#') {
            pq.push((score - 1, p.move_dir(d).unwrap(), d));
        }
        pq.push((score - 1000, p, d.cw()));
        pq.push((score - 1000, p, d.acw()));
    }

    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut visit_scores: HashMap<(Point, Direction4), _> = HashMap::new();
    let mut visit_sets: HashMap<(Point, Direction4), HashSet<Point>> = HashMap::new();

    pq.push((0, (start, dir), (start, dir)));
    visit_sets.insert((start, dir), HashSet::new());
    visit_scores.insert((start, dir), 0);

    while let Some((score, (p, d), pred)) = pq.pop() {
        if visit_scores.get(&(p, d)) == Some(&score) || visit_scores.get(&(p, d)) == None {
            let s2 = visit_sets.get(&pred).unwrap().clone();
            let s = visit_sets.entry((p, d)).or_insert(HashSet::new());
            for p in s2 {
                s.insert(p);
            }
            s.insert(p);
        }
        if visited.contains(&(p, d)) {
            continue;
        }
        visited.insert((p, d));
        visit_scores.insert((p, d), score);

        if p == end && score == best_score {
            println!("best found");
            for p in visit_sets[&(p, d)].clone() {
                grid[p] = 'O';
            }
        }

        if p.move_dir(d).is_some_and(|p| grid[p] != '#') {
            pq.push((score - 1, (p.move_dir(d).unwrap(), d), (p, d)));
        }
        pq.push((score - 1000, (p, d.cw()), (p, d)));
        pq.push((score - 1000, (p, d.acw()), (p, d)));
    }

    println!("{grid}");

    grid.iter_idx().filter(|&(_, &c)| c == 'O').count()
}
