use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input.lines().map(|l| {
        let (goal, nums) = l.split_once(": ").unwrap();

        let goal = p::<u64>(goal);
        let nums = nums.split_whitespace().map(p::<u64>).collect_vec();

        let mut stack = Vec::new();
        stack.push((goal, nums.len() - 1));

        while let Some((num, i)) = stack.pop() {
            if i == 0 {
                if num == nums[0] {
                    return goal;
                } else {
                    continue;
                }
            }
            if let Some(sub) = num.checked_sub(nums[i]) {
                stack.push((sub, i-1));
            }
            if num.is_multiple_of(nums[i]) {
                stack.push((num / nums[i], i-1));
            }
        }

        0
    }).sum::<u64>()
}

#[test]
fn test() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    let output = 11387;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    input.lines().map(|l| {
        let (goal, nums) = l.split_once(": ").unwrap();

        let goal = p::<u64>(goal);
        let nums = nums.split_whitespace().map(p::<u64>).collect_vec();

        let mut stack = Vec::new();
        stack.push((goal, nums.len() - 1));

        while let Some((num, i)) = stack.pop() {
            if i == 0 {
                if num == nums[0] {
                    return goal;
                } else {
                    continue;
                }
            }
            if let Some(sub) = num.checked_sub(nums[i]) {
                stack.push((sub, i-1));
            }
            if num.is_multiple_of(nums[i]) {
                stack.push((num / nums[i], i-1));
            }
            let n = nums[i];
            let digits = (n as f64).log10().floor() as u32 + 1;
            if num % 10u64.pow(digits) == n {
                stack.push((num / 10u64.pow(digits), i-1));
            }
        }

        0
    }).sum::<u64>()
}

#[cfg(test)]
mod benches {
    use crate::get_input;
    use crate::year_2024::day7::*;
    use test::{black_box, Bencher};

    #[bench]
    fn part1_normal(b: &mut Bencher) {
        let input = &get_input(2024, 7).unwrap();
        b.iter(|| {
            black_box(part_1(input));
        })
    }

    #[bench]
    fn part2_normal(b: &mut Bencher) {
        let input = &get_input(2024, 7).unwrap();
        b.iter(|| {
            black_box(part_2(input));
        })
    }
}
