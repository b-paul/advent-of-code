use std::ops::IndexMut;
use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input
        .trim()
        .split(',')
        .map(|l| {
            l.bytes().fold(0u8, |a, c| {
                let a = a.wrapping_add(c);
                let a = a.wrapping_mul(17);
                a
            }) as usize
        })
        .sum::<usize>()
}

fn hash(s: &str) -> usize {
    s.bytes().fold(0u8, |a, c| {
        let a = a.wrapping_add(c);
        let a = a.wrapping_mul(17);
        a
    }) as usize
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut hash_map: Vec<Vec<(&str, usize)>> = Vec::new();
    for _ in 0..256 {
        hash_map.push(Vec::new());
    }

    for s in input.trim().split(',') {
        if s.contains('-') {
            let s = s.split('-').next().unwrap();
            let h = hash(s);
            let e = hash_map.index_mut(h);
            for i in 0..e.len() {
                if e[i].0 == s {
                    e.remove(i);
                    break;
                }
            }
        } if s.contains('=') {
            let mut sp = s.split('=');
            let s = sp.next().unwrap();
            let n = sp.next().unwrap().parse::<usize>().unwrap();
            let h = hash(s);
            let e = hash_map.index_mut(h);
            'ex: {
                for i in 0..e.len() {
                    if e[i].0 == s {
                        e[i].1 = n;
                        break 'ex;
                    }
                }
                e.push((s, n));
            }
        }
    }

    hash_map.iter().enumerate().fold(0, |acc, (j, e)| {
        e.iter().enumerate().fold(acc, |acc, (i, (_, n))| {
            println!("{}", (j+1)*(i+1) * n);
            acc + (j+1)*(i+1) * n
        })
    })
}
