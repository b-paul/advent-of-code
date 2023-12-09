use crate::helper::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut lines = input.lines();
    let pattern = lines.next().unwrap();
    let matches = lines
        .skip(1)
        .map(|l| {
            let ws = l.split(" = ").collect_vec();
            let name = ws[0].to_string();

            let a = ws[1].split(", ").collect_vec();
            let l = a[0].split('(').nth(1).unwrap().to_string();
            let r = a[1].split(')').next().unwrap().to_string();

            (name, (l, r))
        })
        .collect::<HashMap<_, _>>();

    let mut cur = "AAA".to_string();
    let mut r = 0;
    for c in pattern.chars().cycle() {
        if cur == "ZZZ".to_string() {
            break;
        }
        r += 1;
        cur = match c {
            'L' => matches.get(&cur).unwrap().0.clone(),
            'R' => matches.get(&cur).unwrap().1.clone(),
            _ => cur,
        };
    }

    r
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut lines = input.lines();
    let pattern = lines.next().unwrap();
    let matchesv = lines
        .skip(1)
        .map(|l| {
            let ws = l.split(" = ").collect_vec();
            let name = ws[0].to_string();

            let a = ws[1].split(", ").collect_vec();
            let l = a[0].split('(').nth(1).unwrap().to_string();
            let r = a[1].split(')').next().unwrap().to_string();

            (name, (l, r))
        })
        .collect::<Vec<_>>();

    let mut counts = HashMap::new();

    let mut mappings = HashMap::new();

    for (name, _) in matchesv.iter() {
        let name2 = name.chars().last().unwrap();
        let count = counts.entry(name2).or_default();

        mappings.insert(name.clone(), (name2, *count));
        *count += 1;
    }

    let matches = matchesv
        .iter()
        .map(|(name, (l, r))| {
            (
                mappings.get(name).unwrap().clone(),
                (
                    mappings.get(l).unwrap().clone(),
                    mappings.get(r).unwrap().clone(),
                ),
            )
        })
        .collect::<HashMap<_, _>>();

    let mut curs = Vec::new();
    for i in 0.. {
        let cur = ('A', i);
        if !matches.contains_key(&cur) {
            break;
        }
        curs.push(cur);
    }
    let counts = curs
        .clone()
        .into_iter()
        .map(|mut cur| {
            let mut res = 0;
            for c in pattern.chars().cycle() {
                if cur.0 == 'Z' {
                    break;
                }

                res += 1;
                cur = match c {
                    'L' => matches.get(&cur).unwrap().0.clone(),
                    'R' => matches.get(&cur).unwrap().1.clone(),
                    _ => cur,
                };
            }
            res
        })
        .collect_vec();

    format!("Ask wolfram alpha for the lcm of {counts:?}")
}
