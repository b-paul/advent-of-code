use crate::helper::prelude::*;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut input = input.split("\n\n");
    let mut map: HashMap<&str, u64> = input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let n = p::<u64>(&l[5..]);
            (&l[0..3], n)
        })
        .collect();

    let mut instructions = input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let words = l.split_whitespace().collect_vec();
            (words[0], words[1], words[2], words[4])
        })
        .collect::<HashSet<_>>();

    loop {
        let mut step = false;

        for (a, op, b, out) in &instructions.clone() {
            if map.contains_key(a) && map.contains_key(b) {
                match *op {
                    "AND" => {
                        map.insert(out, map[a] & map[b]);
                    }
                    "OR" => {
                        map.insert(out, map[a] | map[b]);
                    }
                    "XOR" => {
                        map.insert(out, map[a] ^ map[b]);
                    }
                    _ => panic!(),
                }
                step = true;
                let cur = instructions.len();
                instructions.remove(&(a, op, b, out));
                assert!(cur == instructions.len() + 1);
            }
        }

        if !step {
            break;
        }
    }

    map.into_iter()
        .filter(|(k, _)| k.starts_with("z"))
        .sorted_by_key(|(k, _)| Reverse(k.to_string()))
        .map(|(_, v)| v)
        .fold(0, |a, b| a << 1 | b)
}

#[test]
fn test() {
    let input = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";
    let output = "z00,z01,z02,z05";
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(_input: &str) -> impl std::fmt::Display {
    "Sorry i literally just did this by hand"
}
