use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

fn can(s: &str, towels: &[&str]) -> bool {
    if s.is_empty() {
        return true;
    }
    for towel in towels {
        if s.starts_with(towel) && can(&s[towel.len()..], towels) {
            return true;
        }
    }
    false
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut input = input.split("\n\n");
    let have = input.next().unwrap().split(", ").collect_vec();
    input
        .next()
        .unwrap()
        .lines()
        .filter(|l| can(l, &have))
        .count()
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

fn count<'a>(s: &'a str, towels: &[&str], memo: &mut HashMap<&'a str, u64>) -> u64 {
    if let Some(n) = memo.get(&s) {
        return *n;
    }
    let mut n = 0;
    for towel in towels {
        if let Some(end) = s.strip_prefix(towel) {
            n += count(end, towels, memo);
        }
    }
    memo.insert(s, n);
    n
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut input = input.split("\n\n");
    let have = input.next().unwrap().split(", ").collect_vec();
    input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut memo = HashMap::new();
            memo.insert("", 1);
            count(l, &have, &mut memo)
        })
        .sum::<u64>()
}
