use itertools::Itertools;

fn kind(hand: &str) -> u32 {
    let mut counts = hand.bytes().counts().into_values().collect::<Vec<_>>();
    counts.sort();
    match counts[..] {
        [.., 5] => 6,
        [.., 4] => 5,
        [.., 2, 3] => 4,
        [.., 3] => 3,
        [.., 2, 2] => 2,
        [.., 2] => 1,
        _ => 0,
    }
}

fn val(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        c => c.to_digit(10).unwrap(),
    }
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|l| {
            let ws = l.split(' ').collect_vec();
            (ws[0].to_string(), ws[1].parse::<usize>().unwrap())
        })
        .sorted_by_key(|(hand, _)| (kind(hand), hand.chars().map(val).collect_vec()))
        .enumerate()
        .map(|(i, (_, win))| win * (i + 1))
        .sum::<usize>()
}

fn kind2(hand: &str) -> u32 {
    if hand.contains('J') {
        "AKQT98765432"
            .chars()
            .map(|c| kind2(&hand.replacen('J', &c.to_string(), 1)))
            .max()
            .unwrap()
    } else {
        kind(hand)
    }
}

fn val2(c: char) -> u32 {
    match c {
        'J' => 1,
        c => val(c),
    }
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|l| {
            let ws = l.split(' ').collect_vec();
            (ws[0].to_string(), ws[1].parse::<usize>().unwrap())
        })
        .sorted_by_key(|(hand, _)| (kind2(hand), hand.chars().map(val2).collect_vec()))
        .enumerate()
        .map(|(i, (_, win))| win * (i + 1))
        .sum::<usize>()
}
