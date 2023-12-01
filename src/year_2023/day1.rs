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
                if c.is_digit(10) {
                    n += c.to_digit(10).unwrap();
                    break;
                }
            }
            n *= 10;
            for c in l.chars().rev() {
                if c.is_digit(10) {
                    n += c.to_digit(10).unwrap();
                    break;
                }
            }
            n
        }).sum::<u32>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    const NUMS: [(&'static str, u32); 18] = [
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
    const NUMS: [&'static str; 9] = [
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
    use test::Bencher;

    #[bench]
    fn part1_normal(b: &mut Bencher) {
        let input = &get_input(2023, 1).unwrap();
        b.iter(|| {
            part_1(input);
        })
    }

    #[bench]
    fn part1_old(b: &mut Bencher) {
        let input = &get_input(2023, 1).unwrap();
        assert_eq!(part_1_old(input).to_string(), part_1(input).to_string());
        b.iter(|| {
            part_1_old(input);
        })
    }

    #[bench]
    fn part2_normal(b: &mut Bencher) {
        let input = &get_input(2023, 1).unwrap();
        b.iter(|| {
            part_2(input);
        })
    }

    #[bench]
    fn part2_faster(b: &mut Bencher) {
        let input = &get_input(2023, 1).unwrap();
        assert_eq!(part_2_faster(input).to_string(), part_2(input).to_string());
        b.iter(|| {
            part_2_faster(input);
        })
    }

    #[bench]
    fn test(b: &mut Bencher) {
        let input = &get_input(2023, 1).unwrap();
        b.iter(|| {
            input.lines().map(|l| {
                test::black_box(l);
                l.len() as u32
            }).sum::<u32>();
        })
    }
}
