use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    const CONNECTIONS: usize = 1000;

    let points = input
        .lines()
        .map(|s| s.split(',').map(p::<u64>).collect_vec())
        .collect_vec();

    let junctions = points
        .iter()
        .cartesian_product(points.iter())
        .filter(|&(a, b)| a != b)
        .sorted_by_key(|(a, b)| {
            a.iter()
                .zip(b.iter())
                .map(|(&x, &y)| x.abs_diff(y) * x.abs_diff(y))
                .sum::<u64>()
        })
        .step_by(2);

    // i totally thought a union find ds would be way harder to implement than it was... oops! also
    // i totally should have made a library for this
    let mut heads = points.iter().map(|p| (p, p)).collect::<HashMap<_, _>>();
    let mut components = points.iter().map(|p| (p, 1)).collect::<HashMap<_, _>>();

    let find = |p, heads: &mut HashMap<_, _>| {
        let mut p2 = p;
        while heads[p2] != p2 {
            p2 = heads[p2];
        }
        let mut p3 = p;
        while heads[p3] != p2 {
            let t = heads[p3];
            heads.insert(p3, p2);
            p3 = t
        }
        p2
    };

    for (a, b) in junctions.take(CONNECTIONS) {
        // join
        let a2 = find(a, &mut heads);
        let b2 = find(b, &mut heads);
        if b2 != a2 {
            heads.insert(b2, a2);
            components.insert(a2, components[a2] + components[b2]);
            components.remove(b2);
        }
    }

    components.into_values().sorted().rev().take(3).product::<u64>()
}

#[test]
fn test() {
    let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    let output = 25272;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let points = input
        .lines()
        .map(|s| s.split(',').map(p::<u64>).collect_vec())
        .collect_vec();

    let junctions = points
        .iter()
        .cartesian_product(points.iter())
        .filter(|&(a, b)| a != b)
        .sorted_by_key(|(a, b)| {
            a.iter()
                .zip(b.iter())
                .map(|(&x, &y)| x.abs_diff(y) * x.abs_diff(y))
                .sum::<u64>()
        })
        .step_by(2);

    let mut heads = points.iter().map(|p| (p, p)).collect::<HashMap<_, _>>();
    let mut components = points.iter().map(|p| (p, 1)).collect::<HashMap<_, _>>();

    let find = |p, heads: &mut HashMap<_, _>| {
        let mut p2 = p;
        while heads[p2] != p2 {
            p2 = heads[p2];
        }
        let mut p3 = p;
        while heads[p3] != p2 {
            let t = heads[p3];
            heads.insert(p3, p2);
            p3 = t
        }
        p2
    };

    for (a, b) in junctions {
        // join
        let a2 = find(a, &mut heads);
        let b2 = find(b, &mut heads);
        if b2 != a2 {
            heads.insert(b2, a2);
            components.insert(a2, components[a2] + components[b2]);
            components.remove(b2);
        }

        if components.len() == 1 {
            return a[0] * b[0];
        }
    }

    unreachable!()
}
