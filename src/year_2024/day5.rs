use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut input = input.split("\n\n");
    let o = input.next().unwrap().lines().map(|l| {
        let n = l.split('|').map(p::<u32>).collect_vec();
        (n[0], n[1])
    }).collect_vec();

    let mut orders = HashMap::new();

    for (k, v) in o {
        orders.entry(k).or_insert(HashSet::new()).insert(v);
    }

    input.next().unwrap().lines().map(|l| {
        let line = l.split(',').map(p::<u32>).collect_vec();

        for (i, a) in line.iter().enumerate() {
            for (_, b) in line[i+1..].iter().enumerate() {
                if !orders.get(a).is_some_and(|s| s.contains(b)) {
                    return 0;
                }
            }
        }

        line[line.len()/2]
    }).sum::<u32>()
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
    let o = input.next().unwrap().lines().map(|l| {
        let n = l.split('|').map(p::<u32>).collect_vec();
        (n[0], n[1])
    }).collect_vec();

    let mut orders = HashMap::new();

    for (k, v) in o {
        orders.entry(k).or_insert(HashSet::new()).insert(v);
    }

    input.next().unwrap().lines().map(|l| {
        let line = l.split(',').map(p::<u32>).collect_vec();

        for (i, a) in line.iter().enumerate() {
            for (_, b) in line[i+1..].iter().enumerate() {
                if !orders.get(a).is_some_and(|s| s.contains(b)) {
                    return Err(line);
                }
            }
        }
        Ok(line)
    }).map(|l| {
        match l {
            Ok(_) => 0,
            Err(mut l) => {
                for i in 0usize..l.len() {
                    for j in i..l.len() {
                        if !orders.get(&l[i]).is_some_and(|s| s.contains(&l[j])) {
                            l.swap(i, j);
                        }
                    }
                }
                l[l.len()/2]
            },
        }
    }).sum::<u32>()
}
