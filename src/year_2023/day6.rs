use itertools::Itertools;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let lines = input.lines().map(|l| { l.split_ascii_whitespace().collect_vec() }).collect_vec();
    let times = lines[0][1..].iter().map(|n| n.parse::<u32>().unwrap()).collect_vec();
    let dists = lines[1][1..].iter().map(|n| n.parse::<u32>().unwrap()).collect_vec();

    times.into_iter().zip(dists.into_iter()).map(|(t, d)| {
        let mut count = 0;
        for b in 0..t {
            if b * (t - b) > d {
                count += 1;
            }
        }
        count
    }).product::<u32>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let lines = input.lines().map(|l| { l.chars().skip(11).collect::<String>().split_ascii_whitespace().join("") }).collect_vec();
    let t = lines[0].parse::<u64>().unwrap();
    let d = lines[1].parse::<u64>().unwrap();

    let mut count = 0;
    for b in 0..t {
        if b * (t - b) > d {
            count += 1;
        }
    }
    count
}
