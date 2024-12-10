use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut deque = VecDeque::new();
    let mut id = 0;
    let mut skip = false;

    for n in input.chars().flat_map(|c| c.to_digit(10)) {
        if !skip {
            deque.push_back((id, n as u64));
            id += 1;
        }
        skip = !skip;
    }

    let mut checksum = 0u64;
    let mut pos = 0u64;
    skip = false;

    for n in input.chars().flat_map(|c| c.to_digit(10)) {
        let mut n = n as u64;
        if !skip {
            let Some((id, m)) = deque.pop_front() else { break; };
            checksum += id * (2 * pos + m - 1) * m / 2;
            pos += m;
        } else {
            while n > 0 {
                let Some((id, m)) = deque.back_mut() else { break; };
                if n >= *m {
                    checksum += *id * (2 * pos + *m - 1) * *m / 2;
                    pos += *m;
                    n -= *m;
                    deque.pop_back();
                } else {
                    *m -= n;
                    checksum += *id * (2 * pos + n - 1) * n / 2;
                    pos += n;
                    n = 0;
                }
            }
        }

        skip = !skip;
    }

    checksum
}

#[test]
fn test() {
    let input = "2333133121414131402";
    let output = 2858;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut checksum: u64 = 0;

    let mut pos = 0;

    let mut skip = false;

    let mut deque = Vec::new();
    let mut id = 0;

    for c in input.chars() {
        let Some(n) = c.to_digit(10) else { continue; };
        deque.push((skip, id, n));
        if !skip {
            id += 1;
        }
        skip = !skip;
    }

    let mut end = deque.len() - 1;
    while end > 0 {
        let (skip, id, n) = deque[end];
        if skip {
            end -= 1;
            continue;
        }
        for i in 0..end {
            let (skipp, _, m) = deque[i];
            if skipp && m >= n {
                deque[i].2 = m - n;
                deque[end].0 = true;
                deque.insert(i, (skip, id, n));
                end += 1;
                break;
            }
        }
        end -= 1;
    }

    for (skip, id, n) in deque {
        if skip {
            pos += n;
        } else {
            for _ in 0..n {
                checksum += id as u64 * pos as u64;
                pos += 1;
            }
        }
    }

    checksum
}
