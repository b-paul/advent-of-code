use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut va = Vec::new();
    let mut vb = Vec::new();
    for (a, b) in input.lines().map(|str| {
        let ns = str.split_whitespace().map(p::<u32>).collect::<Vec<_>>();
        (ns[0], ns[1])
    }) {
        va.push(a);
        vb.push(b);
    }
    va.sort();
    vb.sort();

    va.into_iter().zip(vb.into_iter()).map(|(a, b)| a.abs_diff(b)).sum::<u32>()
}

#[test]
fn test() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";
    let output = 31;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut va = Vec::new();
    let mut vb = Vec::new();
    for (a, b) in input.lines().map(|str| {
        let ns = str.split_whitespace().map(p::<u32>).collect::<Vec<_>>();
        (ns[0], ns[1])
    }) {
        va.push(a);
        vb.push(b);
    }

    va.into_iter().map(|n| n * vb.iter().filter(|&&m| m == n).count() as u32).sum::<u32>()
}
