use itertools::Itertools;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|l| {
            let mut n = 0;
            let mut ds = l.chars().filter_map(|c| c.to_digit(10)).peekable();
            n += 10 * ds.peek().unwrap();
            n += ds.last().unwrap();
            n
        })
        .sum::<u32>()
}

// lol this is like 4us faster thanks llvm
pub(crate) fn part_1_old(input: &str) -> impl std::fmt::Display {
    input.lines()
        .map(|l| {
            let mut n = 0;
            for c in l.chars() {
                if let Some(d) = c.to_digit(10) {
                    n += d;
                    break;
                }
            }
            n *= 10;
            for c in l.chars().rev() {
                if let Some(d) = c.to_digit(10) {
                    n += d;
                    break;
                }
            }
            n
        }).sum::<u32>()
}

pub(crate) fn part_1_faster(input: &str) -> impl std::fmt::Display {
    let mut sum = 0;
    let (mut start, mut end) = (None, None);
    for c in input.bytes() {
        if c == b'\n' {
            sum += start.unwrap() * 10 + end.unwrap();
            start = None;
            end = None;
        }
        if c.is_ascii_digit() {
            let m = (c - b'0') as u32;
            end = Some(m);
            start = start.or(Some(m));
        }
    }
    sum
}

pub(crate) fn part_1_fasterer(input: &str) -> impl std::fmt::Display {
    let mut sum = 0;
    let mut start = 0;
    let mut end = 0;
    for c in input.bytes() {
        if c == b'\n' {
            sum += start * 10 + end;
            start = 0;
            end = 0;
        }
        if c.is_ascii_digit() {
            let m = (c - b'0') as u32;
            end = m;
            if start == 0 {
                start = m;
            }
        }
    }
    sum
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    const NUMS: [(&str, u32); 18] = [
        ("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9),
        ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
    ];
    input
        .lines()
        .map(|l| {
            let mut ds = Vec::new();
            for i in 0..l.len() {
                for (substr, res) in NUMS {
                    if l[i..].starts_with(substr) {
                        ds.push(res);
                    }
                }
            }

            let mut n = 0;
            n += 10 * ds[0];
            n += ds[ds.len() - 1];
            n
        })
        .sum::<u32>()
}

// this is uglier too though :(
pub(crate) fn part_2_faster(input: &str) -> impl std::fmt::Display {
    const NUMS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    input
        .lines()
        .map(|l| {
            let mut n = 0;
            'end: for (i, c) in l.chars().enumerate() {
                if let Some(m) = c.to_digit(10) {
                    n += m;
                    break;
                }
                for (j, num) in NUMS.iter().enumerate() {
                    if l[i..].starts_with(num) {
                        n += j as u32 + 1;
                        break 'end;
                    }
                }
            }
            n *= 10;
            // only scan the start and end until we find the first occurance, then stop searching
            'end: for (i, c) in l.chars().rev().enumerate() {
                if let Some(m) = c.to_digit(10) {
                    n += m;
                    break;
                }
                for (j, num) in NUMS.iter().enumerate() {
                    if l[(l.len() - 1 - i)..].starts_with(num) {
                        n += j as u32 + 1;
                        break 'end;
                    }
                }
            }
            n
        })
        .sum::<u32>()
}

#[cfg(test)]
mod benches {
    use crate::get_input;
    use crate::year_2023::day1::*;
    use test::{Bencher, black_box};

    #[bench]
    fn part1_normal(b: &mut Bencher) {
        let input = &get_input(2023, 1).unwrap();
        b.iter(|| {
            black_box(part_1(input));
        })
    }

    #[bench]
    fn part1_old(b: &mut Bencher) {
        let input = &get_input(2023, 1).unwrap();
        assert_eq!(part_1_old(input).to_string(), part_1(input).to_string());
        b.iter(|| {
            black_box(part_1_old(input));
        })
    }

    #[bench]
    fn part1_faster(b: &mut Bencher) {
        let input = &get_input(2023, 1).unwrap();
        assert_eq!(part_1_faster(input).to_string(), part_1(input).to_string());
        b.iter(|| {
            black_box(part_1_faster(input));
        })
    }

    #[bench]
    fn part1_fasterer(b: &mut Bencher) {
        let input = &get_input(2023, 1).unwrap();
        assert_eq!(part_1_fasterer(input).to_string(), part_1(input).to_string());
        b.iter(|| {
            black_box(part_1_fasterer(input));
        })
    }

    #[bench]
    fn part2_normal(b: &mut Bencher) {
        let input = &get_input(2023, 1).unwrap();
        b.iter(|| {
            black_box(part_2(input));
        })
    }

    #[bench]
    fn part2_faster(b: &mut Bencher) {
        let input = &get_input(2023, 1).unwrap();
        assert_eq!(part_2_faster(input).to_string(), part_2(input).to_string());
        b.iter(|| {
            black_box(part_2_faster(input));
        })
    }
}
