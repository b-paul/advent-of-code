fn priority_of(byte: u8) -> u64 {
    if byte >= b'a' {
        1 + byte - b'a'
    } else {
        1 + byte - b'A' + 26
    }.into()
}

fn part1_calculate(line: String) -> u8 {
    let len = line.len();
    let (bytes1, bytes2) = line.as_bytes().split_at(len/2);

    // Create a bitmasks where the nth bit represents compartment 1 containing an item of priority
    // n
    let mut mask = 0u64;
    for b in bytes1 {
        // Variable left shifts are bad! apparently
        mask |= 1 << (priority_of(*b));
    }
    for b in bytes2 {
        if mask & 1 << (priority_of(*b)) != 0 {
            return *b;
        }
    }
    unreachable!()
}

#[test]
fn part1_tests() {
    assert_eq!(part1_calculate("vJrwpWtwJgWrhcsFMMfFFhFp".to_string()), b'p');
    assert_eq!(part1_calculate("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string()), b'L');
    assert_eq!(part1_calculate("PmmdzqPrVvPwwTWBwg".to_string()), b'P');
    assert_eq!(part1_calculate("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string()), b'v');
    assert_eq!(part1_calculate("ttgJtRGJQctTZtZT".to_string()), b't');
    assert_eq!(part1_calculate("CrZsJsPPZsGzwwsLwLmpwMDw".to_string()), b's');
    assert_eq!(priority_of(b'p'), 16);
    assert_eq!(priority_of(b'L'), 38);
    assert_eq!(priority_of(b'P'), 42);
    assert_eq!(priority_of(b'v'), 22);
    assert_eq!(priority_of(b't'), 20);
    assert_eq!(priority_of(b's'), 19);
}

pub fn part_1(input: String) -> String {
    input
        .lines()
        .map(|l| priority_of(part1_calculate(l.to_owned())))
        .sum::<u64>()
        .to_string()
}

fn part2_calculate(l1: String, l2: String, l3: String) -> u64 {
    let mut mask1 = 0u64;
    let mut mask2 = 0u64;
    for b in l1.as_bytes() {
        mask1 |= 1 << (priority_of(*b));
    }
    for b in l2.as_bytes() {
        mask2 |= 1 << (priority_of(*b));
    }
    let mask = mask1 & mask2;
    for b in l3.as_bytes() {
        if mask & (1 << priority_of(*b)) != 0 {
            return priority_of(*b);
        }
    }

    unreachable!()
}

#[test]
fn part2_tests() {
    assert_eq!(part2_calculate(
        "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
        "PmmdzqPrVvPwwTWBwg".to_string()), priority_of(b'r'));
    assert_eq!(part2_calculate(
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string()), priority_of(b'Z'));
}

pub fn part_2(input: String) -> String {
    input
        .lines()
        .array_chunks::<3>()
        .map(|[l1, l2, l3]| part2_calculate(l1.to_string(), l2.to_string(), l3.to_string()))
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod benches {
    use test::Bencher;
    use crate::year_2022::day3::*;

    #[bench]
    fn part1(b: &mut Bencher) {
        b.iter(|| {
            include_str!("../../input/2022/3.txt")
                .lines()
                .map(|l| priority_of(part1_calculate(l.to_string())))
                .sum::<u64>()
                .to_string()
        });
    }

    #[bench]
    fn part2(b: &mut Bencher) {
        b.iter(|| {
            include_str!("../../input/2022/3.txt")
                .split('\n')
                .array_chunks::<3>()
                .map(|[l1, l2, l3]| part2_calculate(l1.to_string(), l2.to_string(), l3.to_string()))
                .sum::<u64>()
                .to_string()
        });
    }

    #[bench]
    fn possibly_faster_part2(b: &mut Bencher) {
        b.iter(|| {
            let bytes = include_str!("../../input/2022/3.txt").as_bytes();

            let mut total = 0;
            let mut i = 0;
            loop {
                let mut mask1: u64 = 0;
                let mut mask2: u64 = 0;
                while bytes[i] != b'\n' {
                    mask1 |= 1 << priority_of(bytes[i]);
                    i += 1;
                }
                i += 1;
                while bytes[i] != b'\n' {
                    mask2 |= 1 << priority_of(bytes[i]);
                    i += 1;
                }
                i += 1;
                let mask = mask1 & mask2;
                while mask & (1 << priority_of(bytes[i])) == 0 {
                    i += 1;
                }
                total += priority_of(bytes[i]);
                while bytes[i] != b'\n' {
                    i += 1;
                }
                i += 1;
                if i >= bytes.len() {
                    break;
                }
            }
            assert_eq!(total, 2738);
            total
        });
    }
}
