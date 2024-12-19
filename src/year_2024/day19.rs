use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

fn can(s: &str, towels: &[&str]) -> bool {
    if s == "" {
        return true;
    }
    for towel in towels {
        if s.starts_with(towel) {
            if can(&s[towel.len()..], towels) {
                return true;
            }
        }
    }
    false
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut input = input.split("\n\n");
    let have = input.next().unwrap().split(", ").collect_vec();
    input.next().unwrap().lines().filter(|l| {
        can(l, &have)
    }).count()
}

#[test]
fn test() {
    let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    let output = 16;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

fn count(s: &str, towels: &[&str], memo: &mut HashMap<usize, u64>) -> u64 {
    if s == "" {
        return 1;
    }
    if let Some(n) = memo.get(&s.len()) {
        return *n;
    }
    let mut n = 0;
    for towel in towels {
        if s.starts_with(towel) {
            n += count(&s[towel.len()..], towels, memo);
        }
    }
    memo.insert(s.len(), n);
    n
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut input = input.split("\n\n");
    let have = input.next().unwrap().split(", ").collect_vec();
    input.next().unwrap().lines().map(|l| {
        let mut memo = HashMap::new();
        count(l, &have, &mut memo)
    }).sum::<u64>()
}
