use crate::helper::*;
use std::collections::BTreeSet;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut schem: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut l = Vec::new();
        for c in line.chars() {
            l.push(c);
        }
        schem.push(l);
    }

    let mut mask = schem
        .iter()
        .map(|l| l.iter().map(|_| false).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for (y, l) in schem.iter().enumerate() {
        'next: for (x, c) in l.iter().enumerate() {
            if !c.is_ascii_digit() {
                continue;
            }
            for (nx, ny) in adjacent_8_u(x, y) {
                if nx >= schem[0].len() || ny >= schem.len() {
                    continue;
                }
                let d = schem[ny][nx];
                if d.is_ascii_digit() || d == '.' {
                    continue;
                }
                mask[y][x] = true;
                continue 'next;
            }
        }
    }

    let mut sum = 0;

    for (y, l) in schem.iter().enumerate() {
        let mut n = 0;
        let mut allowed = false;
        for (x, c) in l.iter().enumerate() {
            if mask[y][x] {
                allowed = true;
            }
            if let Some(d) = c.to_digit(10) {
                n *= 10;
                n += d;
            } else {
                if allowed {
                    sum += n;
                    print!(":");
                }
                allowed = false;
                n = 0;
            }
            print!("{c}");
        }
        if allowed {
            sum += n;
            print!(":");
        }
        println!();
    }

    sum
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut schem: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut l = Vec::new();
        for c in line.chars() {
            l.push(c);
        }
        schem.push(l);
    }

    let mut ids = schem
        .iter()
        .map(|l| l.iter().map(|_| (0, 0)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for (y, l) in schem.iter().enumerate() {
        let mut curstart = 0;
        for (x, c) in l.iter().enumerate() {
            if c.is_ascii_digit() {
                ids[y][x] = (curstart, y);
            } else {
                curstart = x + 1;
            }
        }
    }

    let mut sum = 0;

    for (y, l) in schem.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c != '*' {
                continue;
            }
            let mut count = 0;
            let mut set = BTreeSet::new();
            for (nx, ny) in adjacent_8_u(x, y) {
                if nx >= schem[0].len() || ny >= schem.len() {
                    continue;
                }
                let d = schem[ny][nx];
                if !d.is_ascii_digit() {
                    continue;
                }
                let (nx, ny) = ids[ny][nx];
                if !set.contains(&(nx, ny)) {
                    count += 1;
                    set.insert((nx, ny));
                }
            }
            if count == 2 {
                let mut ratio = 1;
                for (x, y) in set {
                    let mut n = 0;
                    for dx in 0.. {
                        if x + dx >= schem[0].len() || !schem[y][x + dx].is_ascii_digit() {
                            break;
                        }
                        n *= 10;
                        n += schem[y][x + dx].to_digit(10).unwrap();
                    }
                    println!("{n}");
                    ratio *= n;
                }
                sum += ratio;
            }
        }
    }

    sum
}

#[cfg(test)]
mod benches {
    use crate::get_input;
    use crate::year_2023::day3::*;
    use test::{black_box, Bencher};

    #[bench]
    fn part1_normal(b: &mut Bencher) {
        let input = &get_input(2023, 3).unwrap();
        b.iter(|| {
            black_box(part_1(input));
        })
    }

    #[bench]
    fn part2_normal(b: &mut Bencher) {
        let input = &get_input(2023, 3).unwrap();
        b.iter(|| {
            black_box(part_2(input));
        })
    }
}
