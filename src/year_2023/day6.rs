pub fn part_1(input: &str) -> impl std::fmt::Display {
    let times = input[11..37]
        .split_ascii_whitespace()
        .map(|n| n.parse::<f64>().unwrap());
    let dists = input[48..]
        .split_ascii_whitespace()
        .map(|n| n.parse::<f64>().unwrap());

    times
        .zip(dists)
        .map(|(t, d)| {
            let high = ((t + (t * t - d * 4.).sqrt()) / 2. - 0.01).floor();
            let low = ((t - (t * t - d * 4.).sqrt()) / 2. + 0.01).ceil();
            high - low + 1.
        })
        .product::<f64>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut lines = input.as_bytes().chunks(37).map(|l| {
        l.iter()
            .skip(11)
            .filter(|b| b'0' <= **b && **b <= b'9')
            .map(|b| (b - b'0') as u64)
            .fold(0, |a, n| a * 10 + n) as f64
    });
    let t = lines.next().unwrap();
    let d = lines.next().unwrap();

    let high = ((t + (t * t - d * 4.).sqrt()) / 2. - 0.01).floor();
    let low = ((t - (t * t - d * 4.).sqrt()) / 2. + 0.01).ceil();
    high - low + 1.
}

mod benches {
    use crate::get_input;
    use crate::year_2023::day6::*;
    use test::{black_box, Bencher};

    #[bench]
    fn part1_normal(b: &mut Bencher) {
        let input = &get_input(2023, 6).unwrap();
        b.iter(|| {
            black_box(part_1(input));
        })
    }

    #[bench]
    fn part2_normal(b: &mut Bencher) {
        let input = &get_input(2023, 6).unwrap();
        b.iter(|| {
            black_box(part_2(input));
        })
    }
}
