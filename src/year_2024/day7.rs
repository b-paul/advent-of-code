use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input.lines().map(|l| {
        let (goal, nums) = l.split_once(": ").unwrap();

        let goal = p::<u64>(goal);
        let nums = nums.split_whitespace().map(p::<u64>).collect_vec();

        let mut stack = Vec::new();
        stack.push((nums[0], 1));

        while let Some((num, i)) = stack.pop() {
            if i == nums.len() && num == goal {
                if num == goal {
                    return goal;
                } else {
                    continue;
                }
            }
            stack.push((num + nums[i], i+1));
            stack.push((num * nums[i], i+1));
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
        stack.push((nums[0], 1));

        while let Some((num, i)) = stack.pop() {
            if i == nums.len() {
                if num == goal {
                    return goal;
                } else {
                    continue;
                }
            }
            if num > goal {
                continue;
            }
            stack.push((num + nums[i], i+1));
            stack.push((num * nums[i], i+1));
            let n = nums[i];
            stack.push((num * 10u64.pow(n.ilog10()+1) + n, i+1));
        }

        0
    }).sum::<u64>()
}
