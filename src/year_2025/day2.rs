use crate::helper::shorthand::p;
use itertools::Itertools;

fn pow10(n: usize) -> u64 {
    (0..n).fold(1, |a, _| a * 10)
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input
        .split_whitespace()
        .next()
        .unwrap()
        .split(',')
        .flat_map(|s| {
            let (ls, hs) = s.split_once('-').unwrap();
            let (l, h) = (p::<u64>(ls), p::<u64>(hs));

            // We count all multiples of 11 then 101 then 1001, 10001 etc, but we only count the
            // multiples less than 10, 100, 1000, 10000 etc, so that we don't include stuff like
            // 110 as an invalid id. When we use 101 up to 100...1 we also don't include stuff like
            // 101 to 909 or 1001 to 9990999 as they have trailing zeros... so we clamp the lower
            // end of the range and upper end of the range !
            // We loop up until the 10000000...001 is bigger than l i guess?

            (0..).take_while(|&gap| ls.len() > gap).map(move |gap| {
                let m = 1 + pow10(gap + 1);

                let l = (l.next_multiple_of(m) / m).max(pow10(gap));
                let h = (((h + 1).next_multiple_of(m) - m) / m).min(m - 2);

                if h < l {
                    0
                } else {
                    (h * (h + 1) - l * (l - 1)) / 2 * m
                }
            })
        })
        .sum::<u64>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    input
        .split_whitespace()
        .next()
        .unwrap()
        .split(',')
        .flat_map(|s| {
            let (ls, hs) = s.split_once('-').unwrap();
            let (l, h) = (p::<u64>(ls), p::<u64>(hs));

            // We apply the same strategy as with part 1 but we also "cartesian product" in
            // repetitions of digits (so we get 10101 etc). We do run into a problem though! Take
            // the id 222222 for example (from the test). This id is invalid in three ways! It is
            // 222 222, and 22 22 22, and 2 2 2 2 2 2 2... So we have to not double or triple count
            // our answers. We dodge the problem and just put all of the numbers into a set
            // instead!!! After all, the list can't be that big, a single elf entered all of those
            // items manually!!

            (0..)
                .take_while(|&gap| ls.len() > gap)
                .flat_map(move |gap| {
                    (1..)
                        .map(move |rep| ((0..rep).fold(1, |n, _| n * pow10(gap + 1) + 1), gap))
                        .take_while(move |&(m, _)| m < h)
                })
                .flat_map(move |(m, gap)| {
                    let l = (l.next_multiple_of(m) / m).max(pow10(gap));
                    let h = (((h + 1).next_multiple_of(m) - m) / m).min(pow10(gap + 1) - 1);

                    (l..=h).map(move |n| n * m)
                })
        })
        .sorted()
        .dedup_by(|a, b| a == b)
        .sum::<u64>()
}
