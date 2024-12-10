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

    let mut checksum = 0;
    let mut pos = 0;
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
    let mut checksum = 0;

    let mut pos = 0;

    let mut skip = false;

    let mut list = Vec::new();
    let mut id = 0;

    for n in input.chars().flat_map(|c| c.to_digit(10)) {
        let n = n as u64;
        list.push((skip, id, n));
        if !skip {
            id += 1;
        }
        skip = !skip;
    }

    let mut end = list.len() - 1;
    while end > 0 {
        let (skip, id, n) = list[end];
        if skip {
            end -= 1;
            continue;
        }
        for i in 0..end {
            let (skipp, _, m) = list[i];
            if skipp && m >= n {
                list[i].2 = m - n;
                list[end].0 = true;
                list.insert(i, (skip, id, n));
                end += 1;
                break;
            }
        }
        end -= 1;
    }

    for (skip, id, n) in list {
        if skip {
            pos += n;
        } else {
            checksum += id * (2 * pos + n - 1) * n / 2;
            pos += n;
        }
    }

    checksum
}
