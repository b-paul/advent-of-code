use crate::helper::prelude::*;
use itertools::Itertools;

fn increasing(a: &[u32]) -> bool {
    a.iter().zip(a.iter().skip(1)).all(|(l, r)| l < r)
}

fn decreasing(a: &[u32]) -> bool {
    a.iter().zip(a.iter().skip(1)).all(|(l, r)| l > r)
}

fn valid_dist<const MIN: u32, const MAX: u32>(a: &[u32]) -> bool {
    a.iter().zip(a.iter().skip(1)).all(|(&l, &r)| {
        let dist = l.abs_diff(r);
        MIN <= dist && dist <= MAX
    })
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input.lines().filter(|line| {
        let line = line.split_whitespace().map(p::<u32>).collect_vec();

        (increasing(&line) || decreasing(&line)) && valid_dist::<1, 3>(&line)
    }).count()
}

#[test]
fn test() {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    let output = 4;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    input.lines().filter(|line| {
        let line = line.split_whitespace().map(p::<u32>).collect_vec();

        // The lines are never long so it's perfectly fine to just check every combination.
        (0..line.len()).any(|i| {
            let mut line = line.clone();
            line.remove(i);
            (increasing(&line) || decreasing(&line)) && valid_dist::<1, 3>(&line)
        })
    }).count()
}
