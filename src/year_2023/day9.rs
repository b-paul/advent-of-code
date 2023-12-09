fn difs(nums: &[i64]) -> i64 {
    if nums.iter().all(|n| *n == 0) {
        0
    } else {
        let diffs = nums
            .iter()
            .zip(nums.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect::<Vec<_>>();

        diffs.last().unwrap() + difs(&diffs)
    }
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|l| {
            let nums = l
                .split(' ')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            nums.last().unwrap() + difs(&nums)
        })
        .sum::<i64>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|l| {
            let nums = l
                .split(' ')
                .map(|n| n.parse::<i64>().unwrap())
                .rev()
                .collect::<Vec<_>>();

            nums.last().unwrap() + difs(&nums)
        })
        .sum::<i64>()
}
