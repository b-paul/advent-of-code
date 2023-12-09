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

pub(crate) fn part_1_interpolation(input: &str) -> impl std::fmt::Display {
    const INTERP: [i64; 21] = [
        1, -21, 210, -1330, 5985, -20349, 54264, -116280, 203490, -293930, 352716, -352716, 293930,
        -203490, 116280, -54264, 20349, -5985, 1330, -210, 21,
    ];
    input
        .lines()
        .map(|l| {
            l.split(' ')
                .enumerate()
                .map(|(i, n)| INTERP[i] * n.parse::<i64>().unwrap())
                .sum::<i64>()
        })
        .sum::<i64>()
}

pub(crate) fn part_1_interpolation_ugly(input: &str) -> impl std::fmt::Display {
    const INTERP: [i64; 21] = [
        1, -21, 210, -1330, 5985, -20349, 54264, -116280, 203490, -293930, 352716, -352716, 293930,
        -203490, 116280, -54264, 20349, -5985, 1330, -210, 21,
    ];
    let mut sum = 0;
    let mut i = 0;
    let mut cur = 0;
    let mut sign = 1;
    for byte in input.bytes() {
        if byte == b'\n' {
            sum += cur * sign * INTERP[i];
            sign = 1;
            cur = 0;
            i = 0;
        } else if byte == b' ' {
            sum += cur * sign * INTERP[i];
            sign = 1;
            cur = 0;
            i += 1;
        } else if byte == b'-' {
            sign = -1;
        } else {
            cur *= 10;
            cur += (byte - b'0') as i64;
        }
    }
    sum
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

pub(crate) fn part_2_interpolation(input: &str) -> impl std::fmt::Display {
    const INTERP: [i64; 21] = [
        21, -210, 1330, -5985, 20349, -54264, 116280, -203490, 293930, -352716, 352716, -293930,
        203490, -116280, 54264, -20349, 5985, -1330, 210, -21, 1,
    ];
    input
        .lines()
        .map(|l| {
            l.split(' ')
                .enumerate()
                .map(|(i, n)| INTERP[i] * n.parse::<i64>().unwrap())
                .sum::<i64>()
        })
        .sum::<i64>()
}

pub(crate) fn part_2_interpolation_ugly(input: &str) -> impl std::fmt::Display {
    const INTERP: [i64; 21] = [
        21, -210, 1330, -5985, 20349, -54264, 116280, -203490, 293930, -352716, 352716, -293930,
        203490, -116280, 54264, -20349, 5985, -1330, 210, -21, 1,
    ];
    let mut sum = 0;
    let mut i = 0;
    let mut cur = 0;
    let mut sign = 1;
    for byte in input.bytes() {
        if byte == b'\n' {
            sum += cur * sign * INTERP[i];
            sign = 1;
            cur = 0;
            i = 0;
        } else if byte == b' ' {
            sum += cur * sign * INTERP[i];
            sign = 1;
            cur = 0;
            i += 1;
        } else if byte == b'-' {
            sign = -1;
        } else {
            cur *= 10;
            cur += (byte - b'0') as i64;
        }
    }
    sum
}

#[cfg(test)]
mod benches {
    use crate::get_input;
    use crate::year_2023::day9::*;
    use test::{black_box, Bencher};

    #[bench]
    fn part1_normal(b: &mut Bencher) {
        let input = &get_input(2023, 9).unwrap();
        b.iter(|| {
            black_box(part_1(input));
        })
    }

    #[bench]
    fn part1_interpolation(b: &mut Bencher) {
        let input = &get_input(2023, 9).unwrap();
        assert_eq!(
            part_1_interpolation(input).to_string(),
            part_1(input).to_string()
        );
        b.iter(|| {
            black_box(part_1_interpolation(input));
        })
    }

    #[bench]
    fn part1_interpolation_ugly(b: &mut Bencher) {
        let input = &get_input(2023, 9).unwrap();
        assert_eq!(
            part_1_interpolation_ugly(input).to_string(),
            part_1(input).to_string()
        );
        b.iter(|| {
            black_box(part_1_interpolation_ugly(input));
        })
    }

    #[bench]
    fn part2_normal(b: &mut Bencher) {
        let input = &get_input(2023, 9).unwrap();
        b.iter(|| {
            black_box(part_2(input));
        })
    }

    #[bench]
    fn part2_interpolation(b: &mut Bencher) {
        let input = &get_input(2023, 9).unwrap();
        assert_eq!(
            part_2_interpolation(input).to_string(),
            part_2(input).to_string()
        );
        b.iter(|| {
            black_box(part_2_interpolation(input));
        })
    }

    #[bench]
    fn part2_interpolation_ugly(b: &mut Bencher) {
        let input = &get_input(2023, 9).unwrap();
        assert_eq!(
            part_2_interpolation_ugly(input).to_string(),
            part_2(input).to_string()
        );
        b.iter(|| {
            black_box(part_2_interpolation_ugly(input));
        })
    }
}
