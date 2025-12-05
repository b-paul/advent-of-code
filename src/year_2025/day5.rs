use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut input = input.split("\n\n");

    let rs = input
        .next()
        .unwrap()
        .lines()
        .map(|s| {
            let (l, h) = s.split_once('-').unwrap();
            p::<u64>(l)..=p::<u64>(h)
        })
        .collect_vec();

    input
        .next()
        .unwrap()
        .lines()
        .filter(|&n| {
            let n = p::<u64>(n);
            rs.iter().any(|r| r.contains(&n))
        })
        .count()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut input = input.split("\n\n");

    let mut rs = input
        .next()
        .unwrap()
        .lines()
        .map(|s| {
            let (l, h) = s.split_once('-').unwrap();
            p::<u64>(l)..p::<u64>(h)
        })
        .collect_vec();
    rs.sort_by_key(|a| a.start);

    let mut i = 1;
    while i < rs.len() {
        if rs[i - 1].end >= rs[i].start {
            rs[i] = rs[i - 1].end + 1..rs[i].end;
        }
        if rs[i].end < rs[i].start {
            rs.remove(i);
        } else {
            i += 1;
        }
    }

    rs.into_iter().map(|r| r.end - r.start + 1).sum::<u64>()
}
