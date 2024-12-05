use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut input = input.split("\n\n");

    let mut orders = HashMap::new();

    for l in input.next().unwrap().lines() {
        let n = l.split('|').map(p::<u32>).collect_vec();
        orders.entry(n[0]).or_insert(HashSet::new()).insert(n[1]);
    }

    input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let line = l.split(',').map(p::<u32>).collect_vec();

            for (i, a) in line.iter().enumerate() {
                for b in line[i + 1..].iter() {
                    if !orders.get(a).is_some_and(|s| s.contains(b)) {
                        return 0;
                    }
                }
            }

            line[line.len() / 2]
        })
        .sum::<u32>()
}

#[test]
fn test() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    let output = 123;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut input = input.split("\n\n");

    let mut orders = HashMap::new();

    for l in input.next().unwrap().lines() {
        let n = l.split('|').map(p::<u32>).collect_vec();
        orders.entry(n[0]).or_insert(HashSet::new()).insert(n[1]);
    }

    input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut line = l.split(',').map(p::<u32>).collect_vec();

            for (i, a) in line.iter().enumerate() {
                for b in line[i + 1..].iter() {
                    if !orders.get(a).is_some_and(|s| s.contains(b)) {
                        // Sort
                        for i in 0usize..line.len() {
                            for j in i..line.len() {
                                if !orders.get(&line[i]).is_some_and(|s| s.contains(&line[j])) {
                                    line.swap(i, j);
                                }
                            }
                        }
                        return line[line.len() / 2];
                    }
                }
            }
            0
        })
        .sum::<u32>()
}
