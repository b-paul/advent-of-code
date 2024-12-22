use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|l| {
            let mut n = p::<u32>(l);
            for _ in 0..2000 {
                n = ((n << 6) ^ n) & 0xffffff;
                n = (n >> 5) ^ n;
                n = (n << 11) ^ n;
            }
            (n & 0xffffff) as u64
        })
        .sum::<u64>()
}

pub fn part_1_simd(input: &str) -> impl std::fmt::Display {
    use std::simd::prelude::*;
    input
        .lines()
        .chunks(8)
        .into_iter()
        .map(|ls| {
            let ls = ls.map(p::<u32>).collect_vec();
            let ns: [u32; 8] =
                std::array::try_from_fn(|i| Some(ls.get(i).copied().unwrap_or(0))).unwrap();
            let mut ns = u32x8::from_array(ns);
            let mask = u32x8::splat(0xffffff);
            for _ in 0..2000 {
                ns = ((ns << 6) ^ ns) & mask;
                ns = (ns >> 5) ^ ns;
                ns = (ns << 11) ^ ns;
            }
            ns = ns & mask;
            ns.to_array().into_iter().map(|n| n as u64).sum::<u64>()
        })
        .sum::<u64>()
}

#[test]
fn test() {
    let input = "1
2
3
2024";
    let output = 23;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut counter = HashMap::<Vec<i64>, i64>::new();
    for l in input.lines() {
        let mut n = p::<u64>(l);
        let mut v = Vec::new();
        let mut s = Vec::new();
        for _ in 0..2000 {
            let old = (n % 10) as i64;
            n = ((n << 6) ^ n) & 0xffffff;
            n = (n >> 5) ^ n;
            n = ((n << 11) ^ n) & 0xffffff;
            let new = (n % 10) as i64;
            v.push(new - old);
            s.push(new);
        }
        let mut done = HashSet::new();
        for (seq, val) in v.windows(4).zip(s.into_iter().skip(3)) {
            if !done.contains(&seq.to_vec()) {
                *counter.entry(seq.to_vec()).or_default() += val;
                done.insert(seq.to_vec());
            }
        }
    }

    counter.into_iter().map(|(_, v)| v).max().unwrap()
}

#[cfg(test)]
mod benches {
    use crate::get_input;
    use crate::year_2024::day22::*;
    use test::{black_box, Bencher};

    #[bench]
    fn part1_normal(b: &mut Bencher) {
        let input = &get_input(2024, 22).unwrap();
        b.iter(|| {
            black_box(part_1(input));
        })
    }

    #[bench]
    fn part1_simd(b: &mut Bencher) {
        let input = &get_input(2024, 22).unwrap();
        assert_eq!(part_1_simd(input).to_string(), part_1(input).to_string());
        b.iter(|| {
            black_box(part_1_simd(input));
        })
    }
}
