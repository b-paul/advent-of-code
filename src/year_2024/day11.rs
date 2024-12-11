use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

fn solve(n: u64, steps: usize, memo: &mut HashMap<(u64, usize), u64>) -> u64 {
    if steps == 0 {
        return 1;
    }
    if let Some(&ans) = memo.get(&(n, steps)) {
        return ans;
    }

    if n == 0 {
        let ans = solve(1, steps-1, memo);
        memo.insert((n, steps), ans);
        ans
    } else if n.to_string().len() % 2 == 0 {
        let s = n.to_string();
        let a = p::<u64>(&s[..s.len()/2]);
        let b = p::<u64>(&s[s.len()/2..]);
        let ans = solve(a, steps-1, memo) + solve(b, steps-1, memo);
        memo.insert((n, steps), ans);
        ans
    } else {
        let ans = solve(n * 2024, steps-1, memo);
        memo.insert((n, steps), ans);
        ans
    }
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut memo = HashMap::new();

    input.split_whitespace().map(p::<u64>).map(|n| {
        solve(n, 25, &mut memo)
    }).sum::<u64>()
}

#[test]
fn test() {
    let input = "125 17";
    let output = 55312;
    assert_eq!(part_1(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut memo = HashMap::new();

    input.split_whitespace().map(p::<u64>).map(|n| {
        solve(n, 75, &mut memo)
    }).sum::<u64>()
}
