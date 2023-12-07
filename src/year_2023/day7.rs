use itertools::Itertools;
use std::collections::HashMap;

/*
.filter_map(|n| n.parse::<u64>().ok())
*/

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    hand: String,
    win: u32,
}

fn kind(hand: &String) -> u32 {
    let mut map = HashMap::new();

    for c in hand.chars() {
        let e: &mut u32 = map.entry(c).or_default();

        *e += 1;
    }

    let set = map.into_values().collect::<Vec<_>>();

    if set.contains(&5) { 0 }
    else if set.contains(&4) { 1 }
    else if set.contains(&3) && set.contains(&2) { 2 }
    else if set.contains(&3) { 3 }
    else if set.iter().filter(|n| **n == 2).count() == 2 { 4 }
    else if set.contains(&2) { 5 }
    else { 6 }
}

fn val(c: char) -> u32 {
    let n = if c == 'A' { 14 }
    else if c == 'K' { 13 }
    else if c == 'Q' { 12 }
    else if c == 'J' { 10 }
    else if c == 'T' { 11 }
    else { c.to_digit(10).unwrap() };
    128 -n
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let hand1 = self.hand.clone();
        let k1 = kind(&hand1);
        let hand2 = other.hand.clone();
        let k2 = kind(&hand2);

        if k1 < k2 {
            Some(std::cmp::Ordering::Greater)
        } else if k1 > k2 {
            Some(std::cmp::Ordering::Less)
        } else {
            for (c1, c2) in hand1.chars().zip(hand2.chars()) {
                let c1 = val(c1);
                let c2 = val(c2);
                match c1.partial_cmp(&c2) {
                    Some(std::cmp::Ordering::Less) => return Some(std::cmp::Ordering::Greater),
                    Some(std::cmp::Ordering::Greater) => return Some(std::cmp::Ordering::Less),
                    _ => ()
                }
            }
            None
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut hands = input
        .lines()
        .map(|l| {
            let ws = l.split(' ').collect_vec();

            Hand {
                hand: ws[0].to_string(),
                win: ws[1].parse::<u32>().unwrap(),
            }
        })
        .collect_vec();

    hands.sort();

    hands.iter().enumerate().map(|(i, h)| {
        h.win as u64 * (i + 1) as u64
    }).sum::<u64>()
}

#[derive(Debug, Eq, PartialEq)]
struct Hand2 {
    hand: String,
    win: u32,
}

fn kind2(hand: &String) -> u32 {
    if hand.contains('J') {
        let mut max = 20;
        for c in "AKQT98765432".chars() {
            let hand2 = hand.clone().replacen("J", &c.to_string(), 1);
            max = max.min(kind2(&hand2));
        }
        return max;
    }

    let mut map = HashMap::new();

    for c in hand.chars() {
        let e: &mut u32 = map.entry(c).or_default();

        *e += 1;
    }

    let set = map.into_values().collect::<Vec<_>>();

    if set.contains(&5) { 0 }
    else if set.contains(&4) { 1 }
    else if set.contains(&3) && set.contains(&2) { 2 }
    else if set.contains(&3) { 3 }
    else if set.iter().filter(|n| **n == 2).count() == 2 { 4 }
    else if set.contains(&2) { 5 }
    else { 6 }
}

fn val2(c: char) -> u32 {
    let n = if c == 'A' { 14 }
    else if c == 'K' { 13 }
    else if c == 'Q' { 12 }
    else if c == 'J' { 0 }
    else if c == 'T' { 11 }
    else { c.to_digit(10).unwrap() + 1 };
    128 -n
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let hand1 = self.hand.clone();
        let k1 = kind2(&hand1);
        let hand2 = other.hand.clone();
        let k2 = kind2(&hand2);

        if k1 < k2 {
            Some(std::cmp::Ordering::Greater)
        } else if k1 > k2 {
            Some(std::cmp::Ordering::Less)
        } else {
            for (c1, c2) in hand1.chars().zip(hand2.chars()) {
                let c1 = val2(c1);
                let c2 = val2(c2);
                match c1.partial_cmp(&c2) {
                    Some(std::cmp::Ordering::Less) => return Some(std::cmp::Ordering::Greater),
                    Some(std::cmp::Ordering::Greater) => return Some(std::cmp::Ordering::Less),
                    _ => ()
                }
            }
            None
        }
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut hands = input
        .lines()
        .map(|l| {
            let ws = l.split(' ').collect_vec();

            Hand2 {
                hand: ws[0].to_string(),
                win: ws[1].parse::<u32>().unwrap(),
            }
        })
        .collect_vec();

    hands.sort();

    hands.iter().enumerate().map(|(i, h)| {
        h.win as u64 * (i + 1) as u64
    }).sum::<u64>()
}

#[test]
fn part2() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    let output = 5905;
    assert_eq!(part_2(input).to_string(),output.to_string());
}
