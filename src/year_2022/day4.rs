pub fn part_1(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|l| {
            let mut nums = l.split(&['-', ',']);
            let (e1min, e1max, e2min, e2max) = (
                nums.next().unwrap().parse::<i64>().unwrap(),
                nums.next().unwrap().parse::<i64>().unwrap(),
                nums.next().unwrap().parse::<i64>().unwrap(),
                nums.next().unwrap().parse::<i64>().unwrap(),
            );

            u64::from((e1min >= e2min && e1max <= e2max) || (e2min >= e1min && e2max <= e1max))
        })
        .sum::<u64>()
        .to_string()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|l| {
            let mut nums = l.split(&['-', ',']);
            let (e1min, e1max, e2min, e2max) = (
                nums.next().unwrap().parse::<i64>().unwrap(),
                nums.next().unwrap().parse::<i64>().unwrap(),
                nums.next().unwrap().parse::<i64>().unwrap(),
                nums.next().unwrap().parse::<i64>().unwrap(),
            );

            u64::from((e1max >= e2min && e1min <= e2min) || (e2max >= e1min && e2min <= e1min))
        })
        .sum::<u64>()
        .to_string()
}
