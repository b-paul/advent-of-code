use itertools::Itertools;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input.lines().map(|l| {
        let s = l.split(": ").nth(1).unwrap().split(" | ").collect_vec();
        let wins = s[0].trim().split(' ').filter_map(|n| n.parse::<u32>().ok()).collect_vec();
        let ours = s[1].trim().split(' ').filter_map(|n| n.parse::<u32>().ok());
        let mut pow = 0;
        for n in ours {
            if wins.contains(&n) { pow += 1; }
        }

        if pow == 0 { 0 } else { 1 << (pow - 1) }
    }).sum::<u32>()
}

pub(crate) fn part_1_faster(input: &str) -> impl std::fmt::Display {
    input.lines().map(|l| {
        let mut s = l.split(": ").nth(1).unwrap().split(" | ");

        let mut wins = 0u128;
        for n in s.next().unwrap().trim().split(' ').filter_map(|n| n.parse::<u8>().ok()) {
            wins |= 1 << n
        }

        let mut ours = 0u128;
        for n in s.next().unwrap().trim().split(' ').filter_map(|n| n.parse::<u8>().ok()) {
            ours |= 1 << n
        }

        let pow = (wins & ours).count_ones();

        1 << pow >> 1
    }).sum::<u32>()
}

fn read_num(b: &[u8]) -> u8 {
    let a = b[0];
    let b = b[1];
    if b'0' <= a && a <= b'9' {
        (a - b'0') * 10 + b - b'0'
    } else {
        b - b'0'
    }
}

pub(crate) fn part_1_fasterer(input: &str) -> impl std::fmt::Display {
    // Observation: the format of each line is exactly the same
    input.as_bytes().chunks_exact(117)
        .map(|l| {
            let mut wins = 0u128;
            for i in 0..10 {
                let i = 10 + 3 * i;
                let n = read_num(&l[i..=i+1]);
                wins |= 1 << n;
            }
            let mut ours = 0u128;
            for i in 0..25 {
                let i = 42 + 3 * i;
                let n = read_num(&l[i..=i+1]);
                ours |= 1 << n;
            }
            let pow = (wins & ours).count_ones();
            1 << pow >> 1
        })
        .sum::<u32>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let cards = input.lines().enumerate().map(|(i, l)| {
        let s = l.split(": ").nth(1).unwrap().split(" | ").collect_vec();
        let wins = s[0].trim().split(' ').filter_map(|n| n.parse::<u32>().ok()).collect_vec();
        let ours = s[1].trim().split(' ').filter_map(|n| n.parse::<u32>().ok());

        let mut count = i;
        ours.filter(|n| wins.contains(n)).map(|_| {count += 1; count}).collect_vec()
    }).collect_vec();

    let mut counts = cards.iter().map(|_| 1).collect_vec();
    for (i, card) in cards.into_iter().enumerate() {
        for c in card {
            counts[c] += counts[i];
        }
    }

    counts.iter().sum::<u32>()
}

pub(crate) fn part_2_faster(input: &str) -> impl std::fmt::Display {
    let cards = input.lines().map(|l| {
        let mut s = l.split(": ").nth(1).unwrap().split(" | ");

        let mut wins = 0u128;
        for n in s.next().unwrap().trim().split(' ').filter_map(|n| n.parse::<u8>().ok()) {
            wins |= 1 << n
        }

        let mut ours = 0u128;
        for n in s.next().unwrap().trim().split(' ').filter_map(|n| n.parse::<u8>().ok()) {
            ours |= 1 << n
        }

        (wins & ours).count_ones() as usize
    });

    let mut counts = [1; 197];
    for (i, cards) in cards.enumerate() {
        for c in (i+1)..(i + cards + 1) {
            counts[c] += counts[i];
        }
    }

    counts.iter().sum::<u32>()
}

pub(crate) fn part_2_fasterer(input: &str) -> impl std::fmt::Display {
    // Observation: the format of each line is exactly the same
    let cards = input.as_bytes().chunks_exact(117)
        .map(|l| {
            let mut wins = 0u128;
            for i in 0..10 {
                let i = 10 + 3 * i;
                let n = read_num(&l[i..=i+1]);
                wins |= 1 << n;
            }
            let mut ours = 0u128;
            for i in 0..25 {
                let i = 42 + 3 * i;
                let n = read_num(&l[i..=i+1]);
                ours |= 1 << n;
            }
            (wins & ours).count_ones() as usize
        });

    let mut counts = [1; 197];
    for (i, cards) in cards.enumerate() {
        for c in (i+1)..(i + cards + 1) {
            counts[c] += counts[i];
        }
    }

    counts.iter().sum::<u32>()
}

#[cfg(test)]
mod benches {
    use crate::get_input;
    use crate::year_2023::day4::*;
    use test::{black_box, Bencher};

    #[bench]
    fn part1_normal(b: &mut Bencher) {
        let input = &get_input(2023, 4).unwrap();
        b.iter(|| {
            black_box(part_1(input));
        })
    }

    #[bench]
    fn part1_faster(b: &mut Bencher) {
        let input = &get_input(2023, 4).unwrap();
        assert_eq!(part_1_faster(input).to_string(), part_1(input).to_string());
        b.iter(|| {
            black_box(part_1_faster(input));
        })
    }

    #[bench]
    fn part1_fasterer(b: &mut Bencher) {
        let input = &get_input(2023, 4).unwrap();
        assert_eq!(part_1_fasterer(input).to_string(), part_1(input).to_string());
        b.iter(|| {
            black_box(part_1_fasterer(input));
        })
    }

    #[bench]
    fn part2_normal(b: &mut Bencher) {
        let input = &get_input(2023, 4).unwrap();
        b.iter(|| {
            black_box(part_2(input));
        })
    }

    #[bench]
    fn part2_faster(b: &mut Bencher) {
        let input = &get_input(2023, 4).unwrap();
        assert_eq!(part_2_faster(input).to_string(), part_2(input).to_string());
        b.iter(|| {
            black_box(part_2_faster(input));
        })
    }

    #[bench]
    fn part2_fasterer(b: &mut Bencher) {
        let input = &get_input(2023, 4).unwrap();
        assert_eq!(part_2_fasterer(input).to_string(), part_2(input).to_string());
        b.iter(|| {
            black_box(part_2_fasterer(input));
        })
    }
}
