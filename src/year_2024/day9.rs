use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut checksum = 0;

    let mut pos = 0;

    let mut skip = false;

    let mut deque = VecDeque::new();
    let mut id = 0;

    for c in input.chars() {
        let Some(n) = c.to_digit(10) else { continue; };
        if !skip {
            for _ in 0..n {
                deque.push_back(id);
            }
            id += 1;
        }
        skip = !skip;
    }

    skip = false;

    for c in input.chars() {
        let Some(n) = c.to_digit(10) else { continue; };
        for _ in 0..n {
            if deque.len() == 0 {
                break;
            }

            if skip {
                checksum += pos * deque.pop_back().unwrap() as u64;
            } else {
                checksum += pos * deque.pop_front().unwrap() as u64;
            }

            pos += 1;
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

    let mut deque = VecDeque::new();
    let mut id = 0;

    for c in input.chars() {
        let Some(n) = c.to_digit(10) else { continue; };
        deque.push_back((skip, id, n));
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
