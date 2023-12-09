fn difs1(nums: &[i64]) -> i64 {
    if nums.iter().all(|n| *n == 0) {
        0
    } else {
        let mut num = Vec::new();
        for i in 0..(nums.len() - 1) {
            num.push(nums[i+1] - nums[i]);
        }
        let r = num[num.len() - 1] + difs1(&num);
        r
    }
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input.lines().map(|l| {
        let nums = l.split(' ').map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>();

        nums[nums.len() - 1] + difs1(&nums)
    })
    .sum::<i64>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    input.lines().map(|l| {
        let nums = l.split(' ').map(|n| n.parse::<i64>().unwrap()).rev().collect::<Vec<_>>();

        nums[nums.len() - 1] + difs1(&nums)
    })
    .sum::<i64>()
}
