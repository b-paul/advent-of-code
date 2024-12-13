use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input
        .split("\n\n")
        .map(|l| {
            let mut l = l.lines();
            let a = {
                let l = l.next().unwrap().split_whitespace().collect_vec();
                (
                    p::<i64>(&l[2][2..l[2].len() - 1]),
                    p::<i64>(&l[3][2..l[3].len()]),
                )
            };
            let b = {
                let l = l.next().unwrap().split_whitespace().collect_vec();
                (
                    p::<i64>(&l[2][2..l[2].len() - 1]),
                    p::<i64>(&l[3][2..l[3].len()]),
                )
            };
            let pr = {
                let l = l.next().unwrap().split_whitespace().collect_vec();
                (
                    p::<i64>(&l[1][2..l[1].len() - 1]),
                    p::<i64>(&l[2][2..l[2].len()]),
                )
            };

            let d0 = pr.0 * b.1 - pr.1 * b.0;
            let d1 = a.0 * pr.1 - a.1 * pr.0;
            let d = a.0 * b.1 - a.1 * b.0;
            let x0 = d0 / d;
            let x1 = d1 / d;
            if d0 % d != 0 || d1 % d != 0 {
                0
            } else {
                3 * x0 + x1
            }
        })
        .sum::<i64>()
}

#[test]
fn test() {
    let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    let output = 480;
    assert_eq!(part_1(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    input
        .split("\n\n")
        .map(|l| {
            let mut l = l.lines();
            let a = {
                let l = l.next().unwrap().split_whitespace().collect_vec();
                (
                    p::<i128>(&l[2][2..l[2].len() - 1]),
                    p::<i128>(&l[3][2..l[3].len()]),
                )
            };
            let b = {
                let l = l.next().unwrap().split_whitespace().collect_vec();
                (
                    p::<i128>(&l[2][2..l[2].len() - 1]),
                    p::<i128>(&l[3][2..l[3].len()]),
                )
            };
            let pr = {
                let l = l.next().unwrap().split_whitespace().collect_vec();
                (
                    10000000000000 + p::<i128>(&l[1][2..l[1].len() - 1]),
                    10000000000000 + p::<i128>(&l[2][2..l[2].len()]),
                )
            };

            let d0 = pr.0 * b.1 - pr.1 * b.0;
            let d1 = a.0 * pr.1 - a.1 * pr.0;
            let d = a.0 * b.1 - a.1 * b.0;
            let x0 = d0 / d;
            let x1 = d1 / d;
            if d0 % d != 0 || d1 % d != 0 {
                0
            } else {
                3 * x0 + x1
            }
        })
        .sum::<i128>()
}
