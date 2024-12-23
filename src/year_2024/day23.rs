use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let edges = input.lines().map(|l| (&l[0..2], &l[3..5])).collect_vec();
    let mut adjs = HashMap::<_, BTreeSet<_>>::new();

    let mut nodes = HashSet::new();

    for (from, to) in edges {
        nodes.insert(from);
        nodes.insert(to);
        adjs.entry(to).or_default().insert(from);
        adjs.entry(from).or_default().insert(to);
    }

    let mut sets = HashSet::new();

    for n in nodes {
        for (a, b) in adjs[n].iter().cartesian_product(adjs[n].iter()) {
            if adjs[a].contains(b) {
                let mut s = BTreeSet::new();
                s.insert(n);
                s.insert(a);
                s.insert(b);
                if s.len() == 3 {
                    sets.insert(s);
                }
            }
        }
    }

    sets.into_iter()
        .filter(|s| s.iter().any(|s| s.starts_with("t")))
        .count()
}

#[test]
fn test() {
    let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";
    let output = "co,de,ka,ta";
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let edges = input.lines().map(|l| (&l[0..2], &l[3..5])).collect_vec();
    let mut adjs = HashMap::<_, BTreeSet<_>>::new();

    let mut nodes = HashSet::new();

    for (from, to) in edges {
        nodes.insert(from);
        nodes.insert(to);
        adjs.entry(to).or_default().insert(from);
        adjs.entry(from).or_default().insert(to);
        adjs.entry(from).or_default().insert(from);
        adjs.entry(to).or_default().insert(to);
    }

    let mut best = "".to_string();
    for i in 3.. {
        let mut sets = HashSet::<BTreeSet<&str>>::new();

        for n in &nodes {
            for s in adjs[n].iter().combinations(i) {
                if s.len() == i
                    && s.clone().into_iter()
                        .cartesian_product(s.clone().into_iter())
                        .all(|(a, b)| adjs[a].contains(b))
                {
                    sets.insert(s.into_iter().cloned().collect());
                }
            }
        }

        if sets.is_empty() {
            break;
        }

        best = sets
            .into_iter()
            .next()
            .unwrap()
            .into_iter()
            .sorted()
            .fold("".to_string(), |a: String, n| (a + ",") + n);
    }
    best[1..].to_string()
}
