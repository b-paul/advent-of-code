use crate::helper::prelude::*;
use itertools::Itertools;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let (mut va, mut vb): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|str| {
            let ns = str.split_whitespace().map(p::<u32>).collect::<Vec<_>>();
            (ns[0], ns[1])
        })
        .unzip();
    va.sort();
    vb.sort();

    va.into_iter()
        .zip(vb.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum::<u32>()
}

pub fn part_1_faster(input: &str) -> impl std::fmt::Display {
    // Notice that the input has exactly 1000 lines, and each line is of the format
    // "xxxxx   yyyyy\n"
    // where xxxxx and yyyyy are the digits of the two numbers.
    // This solution assumes that the input will follow this format (although it isn't specified in
    // the question)!
    let (mut va, mut vb): (Vec<_>, Vec<_>) = input
        .bytes()
        .array_chunks::<14>()
        .map(|line| {
            // This is dumb :( idk how else to do it though
            (
                (line[0] - b'0') as u32 * 10000
                    + (line[1] - b'0') as u32 * 1000
                    + (line[2] - b'0') as u32 * 100
                    + (line[3] - b'0') as u32 * 10
                    + (line[4] - b'0') as u32,
                (line[8] - b'0') as u32 * 10000
                    + (line[9] - b'0') as u32 * 1000
                    + (line[10] - b'0') as u32 * 100
                    + (line[11] - b'0') as u32 * 10
                    + (line[12] - b'0') as u32,
            )
        })
        .unzip();

    va.sort_unstable();
    vb.sort_unstable();

    va.into_iter()
        .zip(vb.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum::<u32>()
}

#[test]
fn test() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";
    let output = 31;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let (va, vb): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|str| {
            let ns = str.split_whitespace().map(p::<u32>).collect::<Vec<_>>();
            (ns[0], ns[1])
        })
        .unzip();
    let count = vb.into_iter().counts();

    va.into_iter()
        .map(|n| n * count.get(&n).copied().unwrap_or_default() as u32)
        .sum::<u32>()
}

pub fn part_2_faster(input: &str) -> impl std::fmt::Display {
    // See part_1_faster
    let (va, vb): (Vec<_>, Vec<_>) = input
        .bytes()
        .array_chunks::<14>()
        .map(|line| {
            (
                (line[0] - b'0') as u32 * 10000
                    + (line[1] - b'0') as u32 * 1000
                    + (line[2] - b'0') as u32 * 100
                    + (line[3] - b'0') as u32 * 10
                    + (line[4] - b'0') as u32,
                (line[8] - b'0') as u32 * 10000
                    + (line[9] - b'0') as u32 * 1000
                    + (line[10] - b'0') as u32 * 100
                    + (line[11] - b'0') as u32 * 10
                    + (line[12] - b'0') as u32,
            )
        })
        .unzip();

    let count = vb.into_iter().counts();

    va.into_iter()
        .map(|n| n * count.get(&n).copied().unwrap_or_default() as u32)
        .sum::<u32>()
}

#[cfg(test)]
mod benches {
    use crate::get_input;
    use crate::year_2024::day1::*;
    use test::{black_box, Bencher};

    #[bench]
    fn part1_normal(b: &mut Bencher) {
        let input = &get_input(2024, 1).unwrap();
        b.iter(|| {
            black_box(part_1(input));
        })
    }

    #[bench]
    fn part1_faster(b: &mut Bencher) {
        let input = &get_input(2024, 1).unwrap();
        assert_eq!(part_1_faster(input).to_string(), part_1(input).to_string());
        b.iter(|| {
            black_box(part_1_faster(input));
        })
    }

    #[bench]
    fn part2_normal(b: &mut Bencher) {
        let input = &get_input(2024, 1).unwrap();
        b.iter(|| {
            black_box(part_2(input));
        })
    }

    #[bench]
    fn part2_faster(b: &mut Bencher) {
        let input = &get_input(2024, 1).unwrap();
        assert_eq!(part_2_faster(input).to_string(), part_2(input).to_string());
        b.iter(|| {
            black_box(part_2_faster(input));
        })
    }
}
