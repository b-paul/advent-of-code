use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut s = input.split("\n\n");

    let p1 = s.next().unwrap().lines();

    let rules = p1
        .map(|l| {
            let mut s = l.split('{');
            let name = s.next().unwrap().to_string();
            let end = s.next().unwrap().split('}').next().unwrap();

            let rules = end.split(',').collect_vec();

            let last = rules[rules.len() - 1].to_string();
            let rules = rules[0..rules.len() - 1].to_vec();

            let mut rulev = Vec::new();

            for rule in rules.into_iter() {
                let mut ps = rule.chars();
                let cat = ps.next().unwrap();
                let op = ps.next().unwrap();

                let rule = &rule[2..];

                let mut s = rule.split(':');
                let n = s.next().unwrap().parse::<i64>().unwrap();
                let name = s.next().unwrap().to_string();

                rulev.push((cat, op, n, name));
            }

            (name, (rulev, last))
        })
        .collect::<HashMap<_, _>>();

    let p2 = s.next().unwrap().lines();

    let mut ans = 0;

    for l in p2 {
        let l = l[1..].split('}').next().unwrap();
        let s = l.split(',');
        let mut cats = Vec::new();
        for cat in s {
            cats.push(cat[2..].parse::<i64>().unwrap());
        }

        let mut name = "in".to_string();

        'next: while name != "A".to_string() && name != "R".to_string() {
            let (rulev, last) = rules.get(&name).unwrap();

            for (cat, op, n, name2) in rulev {
                let i = match cat {
                    'x' => 0,
                    'm' => 1,
                    'a' => 2,
                    's' => 3,
                    _ => panic!(),
                };
                let m = cats[i];
                match op {
                    '<' => {
                        if m < *n {
                            name = name2.clone();
                            continue 'next;
                        }
                    }
                    '>' => {
                        if m > *n {
                            name = name2.clone();
                            continue 'next;
                        }
                    }
                    _ => panic!(),
                }
            }

            name = last.clone();
        }

        if name == "A".to_string() {
            ans += cats.iter().sum::<i64>();
        }
    }

    ans
}

#[test]
fn test() {
    let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    let output = 167409079868000u64;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut s = input.split("\n\n");

    let p1 = s.next().unwrap().lines();

    let rules = p1
        .map(|l| {
            let mut s = l.split('{');
            let name = s.next().unwrap().to_string();
            let end = s.next().unwrap().split('}').next().unwrap();

            let rules = end.split(',').collect_vec();

            let last = rules[rules.len() - 1].to_string();
            let rules = rules[0..rules.len() - 1].to_vec();

            let mut rulev = Vec::new();

            for rule in rules.into_iter() {
                let mut ps = rule.chars();
                let cat = ps.next().unwrap();
                let op = ps.next().unwrap();

                let rule = &rule[2..];

                let mut s = rule.split(':');
                let n = s.next().unwrap().parse::<i64>().unwrap();
                let name = s.next().unwrap().to_string();

                rulev.push((cat, op, n, name));
            }

            (name, (rulev, last))
        })
        .collect::<HashMap<_, _>>();

    let mut stack = Vec::new();
    stack.push(("in".to_string(), [1..4001, 1..4001, 1..4001, 1..4001]));

    let mut ans = 0;

    while let Some((name, mut cats)) = stack.pop() {
        if name == "R".to_string() {
            continue;
        }
        if name == "A".to_string() {
            ans += cats.into_iter().map(|r| r.end-r.start).product::<i64>();
            continue;
        }

        let (rulev, last) = rules.get(&name).unwrap();

        for (cat, op, n, name) in rulev {
            let n = *n;
            let i = match cat {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                _ => panic!(),
            };
            match op {
                '<' => {
                    if n <= cats[i].start+1 {
                        ()
                    } else if n <= cats[i].end {
                        let mut cats2 = cats.clone();
                        cats2[i] = cats[i].start..n;
                        stack.push((name.clone(), cats2));
                        cats[i] = n..cats[i].end;
                    } else {
                        stack.push((name.clone(), cats.clone()));
                    }
                }
                '>' => {
                    if n >= cats[i].end-1 {
                        ()
                    } else if n >= cats[i].start {
                        let mut cats2 = cats.clone();
                        cats2[i] = n+1..cats[i].end;
                        stack.push((name.clone(), cats2));
                        cats[i] = cats[i].start..n+1;
                    } else {
                        stack.push((name.clone(), cats.clone()));
                    }
                }
                _ => panic!(),
            }
        }

        stack.push((last.to_string(), cats));
    }

    ans
}
