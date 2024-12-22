use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|l| {
            let mut n = p::<u64>(l);
            for _ in 0..2000 {
                n = ((n << 6) ^ n) % 16777216;
                n = ((n >> 5) ^ n) % 16777216;
                n = ((n << 11) ^ n) % 16777216;
            }
            n
        })
        .sum::<u64>()
}

#[test]
fn test() {
    let input = "1
2
3
2024";
    let output = 23;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut counter = HashMap::<Vec<i64>, i64>::new();
    for l in input.lines() {
        let mut n = p::<u64>(l);
        let mut v = Vec::new();
        let mut s = Vec::new();
        for _ in 0..2000 {
            let old = (n % 10) as i64;
            n = ((n << 6) ^ n) % 16777216;
            n = ((n >> 5) ^ n) % 16777216;
            n = ((n << 11) ^ n) % 16777216;
            let new = (n % 10) as i64;
            v.push(new - old);
            s.push(new);
        }
        let mut done = HashSet::new();
        for (i, seq) in v.windows(4).enumerate() {
            let val = s[i + 3];
            if !done.contains(&seq.to_vec()) {
                *counter.entry(seq.to_vec()).or_default() += val;
                done.insert(seq.to_vec());
            }
        }
    }

    counter.into_iter().map(|(_, v)| v).max().unwrap()
}
