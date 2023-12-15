fn hash(s: &str) -> usize {
    s.bytes().fold(0u8, |a, c| {
        let a = a.wrapping_add(c);
        let a = a.wrapping_mul(17);
        a
    }) as usize
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input.trim().split(',').map(hash).sum::<usize>()
}

use std::simd::prelude::*;

pub(crate) fn part_1_faster(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let input = &input[0..(input.len() - 1)];

    let mut sum = 0;
    let mut cur = 0;

    for &b in input {
        let comma_mask = ((b != b',') as u32).wrapping_sub(1);

        sum += cur & comma_mask;
        cur += b as u32 & !(comma_mask >> 24);
        cur *= 17 & (!comma_mask | 1);
        cur &= 0xFF & !comma_mask;
    }

    sum + cur as u32
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut hash_map: Vec<Vec<(&str, usize)>> = (0..256).map(|_| Vec::new()).collect();

    for s in input.trim().split(',') {
        if s.contains('-') {
            let s = s.split('-').next().unwrap();
            let h = hash(s);
            for i in 0..hash_map[h].len() {
                if hash_map[h][i].0 == s {
                    hash_map[h].remove(i);
                    break;
                }
            }
        }
        if s.contains('=') {
            let mut sp = s.split('=');
            let s = sp.next().unwrap();
            let n = sp.next().unwrap().parse::<usize>().unwrap();
            let h = hash(s);
            'ex: {
                for i in 0..hash_map[h].len() {
                    if hash_map[h][i].0 == s {
                        hash_map[h][i].1 = n;
                        break 'ex;
                    }
                }
                hash_map[h].push((s, n));
            }
        }
    }

    hash_map.iter().enumerate().fold(0, |acc, (j, e)| {
        e.iter().enumerate().fold(acc, |acc, (i, (_, n))| {
            acc + (j + 1) * (i + 1) * n
        })
    })
}

#[cfg(test)]
mod benches {
    use crate::get_input;
    use crate::year_2023::day15::*;
    use test::{black_box, Bencher};

    #[bench]
    fn part1_normal(b: &mut Bencher) {
        let input = &get_input(2023, 15).unwrap();
        b.iter(|| {
            black_box(part_1(input));
        })
    }

    #[bench]
    fn part1_faster(b: &mut Bencher) {
        let input = &get_input(2023, 15).unwrap();
        assert_eq!(part_1_faster(input).to_string(), part_1(input).to_string());
        b.iter(|| {
            black_box(part_1_faster(input));
        })
    }
    #[bench]
    fn part2_normal(b: &mut Bencher) {
        let input = &get_input(2023, 15).unwrap();
        b.iter(|| {
            black_box(part_2(input));
        })
    }
}
