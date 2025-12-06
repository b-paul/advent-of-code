use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut v = Vec::new();

    for s in input.lines() {
        v.push(s.split_whitespace().collect_vec());
    }

    (0..v[0].len())
        .map(|i| match v[v.len() - 1][i] {
            "*" => v[0..v.len() - 1].iter().fold(1, |a, n| a * p::<u64>(n[i])),
            "+" => v[0..v.len() - 1].iter().fold(0, |a, n| a + p::<u64>(n[i])),
            _ => unreachable!(),
        })
        .sum::<u64>()
}

#[test]
fn test() {
    let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    let output = 3263827;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let lines = input.lines().map(|s| s.as_bytes()).collect_vec();
    let ops = lines.last().unwrap();

    // ugly
    let mut sizes = Vec::new();
    let mut len = 1;
    let mut i = 1;
    while i < ops.len() {
        if ops[i] != b' ' {
            sizes.push(len - 1);
            len = 0;
        }
        len += 1;
        i += 1;
    }
    sizes.push(len);

    let mut count = 0;
    let mut off = 0;

    for size in sizes {
        let ns = (0..size).map(|i| {
            (0..lines.len() - 1).fold(0, |a, j| match lines[j][off + i] {
                c @ b'0'..=b'9' => 10 * a + (c - b'0') as u64,
                _ => a,
            })
        });

        count += match ops[off] {
            b'*' => ns.product::<u64>(),
            b'+' => ns.sum::<u64>(),
            _ => unreachable!(),
        };

        off += size + 1;
    }

    count
}
