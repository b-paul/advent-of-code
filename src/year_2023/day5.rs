pub fn part_1(input: &str) -> impl std::fmt::Display {
    let data = input.split("\n\n").collect::<Vec<_>>();

    let mut seeds = data[0].split(' ').skip(1).filter_map(|n| n.parse::<usize>().ok()).collect::<Vec<_>>();

    for data in &data[1..] {
        let data = data.lines().skip(1);
        let ranges = data.map(|l| {
            let xs = l.split(' ').filter_map(|n| n.parse::<usize>().ok()).collect::<Vec<_>>();
            (xs[0], xs[1], xs[2])
        }).collect::<Vec<_>>();
        for seed in seeds.iter_mut() {
            for (dest, source, len) in ranges.iter().cloned() {
                if source <= *seed && *seed < source + len {
                    *seed = *seed + dest - source;
                    break;
                }
            }
        }
    }

    seeds.into_iter().min().unwrap()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let data = input.split("\n\n").collect::<Vec<_>>();

    let seed_data = data[0].split(' ').skip(1).filter_map(|n| n.parse::<usize>().ok()).collect::<Vec<_>>();
    let mut seeds = Vec::new();
    for i in 0..seed_data.len()/2 {
        let start = seed_data[2*i];
        let len = seed_data[2*i + 1];
        seeds.push(start..start + len);
    }

    for data in &data[1..] {
        let mut new_seeds = Vec::new();

        let data = data.lines().skip(1);
        let ranges = data.map(|l| {
            let xs = l.split(' ').filter_map(|n| n.parse::<usize>().ok()).collect::<Vec<_>>();
            (xs[0], xs[1], xs[2])
        }).collect::<Vec<_>>();
        'next: while let Some(seed) = seeds.pop() {
            for (dest, source, len) in ranges.iter().cloned() {
                let end = source + len;
                if (source..end).contains(&seed.start) {
                    let new_len = len.min(seed.len());
                    if new_len == seed.len() {
                        let start = seed.start + dest - source;
                        let end = seed.end + dest - source;
                        new_seeds.push(start..end);
                    } else {
                        seeds.push((seed.start + len)..seed.end);

                        let start = seed.start + dest - source;
                        let end = start + len;
                        new_seeds.push(start..end);
                    }
                } else if (source+1..end).contains(&seed.end) {
                    let len = seed.end - source;
                    seeds.push(seed.start..source);
                    new_seeds.push(dest..dest+len);
                } else if source > seed.start && seed.end >= end {
                    seeds.push(seed.start..source);
                    seeds.push(source + len..seed.end);
                    new_seeds.push(dest..dest+len);
                } else {
                    continue;
                }
                continue 'next;
            }
            new_seeds.push(seed);
        }

        seeds = new_seeds.clone();
    }
    seeds.iter().map(|r| r.start).min().unwrap()
}

mod benches {
    use crate::get_input;
    use crate::year_2023::day5::*;
    use test::{black_box, Bencher};

    #[bench]
    fn part1_normal(b: &mut Bencher) {
        let input = &get_input(2023, 5).unwrap();
        b.iter(|| {
            black_box(part_1(input));
        })
    }

    #[bench]
    fn part2_normal(b: &mut Bencher) {
        let input = &get_input(2023, 5).unwrap();
        b.iter(|| {
            black_box(part_2(input));
        })
    }
}
