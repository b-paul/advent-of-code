use itertools::Itertools;

fn callories(str: &str) -> u64 {
    str.lines().filter_map(|l| l.parse::<u64>().ok()).sum()
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input
        .split("\n\n")
        .map(callories)
        .max()
        .unwrap_or_default()
        .to_string()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    input
        .split("\n\n")
        .map(callories)
        .sorted()
        .rev()
        .take(3)
        .sum::<u64>()
        .to_string()
}
