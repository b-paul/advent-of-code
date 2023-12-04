use itertools::Itertools;
use std::collections::VecDeque;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input.lines().map(|l| {
        let ns = l.split(": ").collect_vec()[1];
        let s = ns.split(" | ").collect_vec();
        let wins: Vec<_> = s[0].trim().split(" ").filter_map(|n| n.parse::<u32>().ok()).collect();
        let ours: Vec<_> = s[1].trim().split(" ").filter_map(|n| n.parse::<u32>().ok()).collect();
        let mut pow = 0;
        for n in ours {
            if wins.contains(&n) { pow += 1; }
        }

        if pow == 0 { 0 } else { 1 << (pow - 1) }
    }).sum::<u32>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let cards = input.lines().enumerate().map(|(i, l)| {
        let ns = l.split(": ").collect_vec()[1];
        let s = ns.split(" | ").collect_vec();
        let wins: Vec<_> = s[0].trim().split(" ").filter_map(|n| n.parse::<u32>().ok()).collect();
        let ours: Vec<_> = s[1].trim().split(" ").filter_map(|n| n.parse::<u32>().ok()).collect();

        let mut cars = Vec::new();

        let mut count = i + 1;

        for n in ours.iter() {
            if wins.contains(&n) {
                cars.push(count);
                count += 1;
            }
        }

        (wins, ours, cars)
    }).collect_vec();

    let mut sum = 0;
    let mut queue = VecDeque::new();

    for i in 0..cards.len() {
        queue.push_back(i);
    }

    while let Some(i) = queue.pop_front() {
        println!("{:?}", queue.len());
        sum += 1;
        for c in cards[i].2.iter() {
            queue.push_back(*c);
        }
    }

    sum
}
