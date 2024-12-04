fn find_idx<const WIDTH: usize>(buf: &[u8]) -> usize {
    let mut mask: u64 = 0;
    for byte in buf.iter().take(WIDTH) {
        mask ^= 1 << (byte - b'a');
    }
    if mask.count_ones() == WIDTH as u32 {
        return 1;
    }

    for i in WIDTH..buf.len() {
        // This compiles to a btc instruction
        mask ^= 1 << (buf[i - WIDTH] - b'a');
        mask ^= 1 << (buf[i] - b'a');
        if mask.count_ones() == WIDTH as u32 {
            return i + 1;
        }
    }
    unreachable!()
}

#[test]
fn header_test() {
    assert_eq!(find_idx::<4>("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes()), 5);
    assert_eq!(find_idx::<4>("nppdvjthqldpwncqszvftbrmjlhg".as_bytes()), 6);
    assert_eq!(find_idx::<4>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes()), 10);
    assert_eq!(find_idx::<4>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes()), 11);
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let bytes: Vec<u8> = input.bytes().collect();

    find_idx::<4>(&bytes).to_string()
}

#[test]
fn message_test() {
    assert_eq!(find_idx::<14>("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes()), 23);
    assert_eq!(find_idx::<14>("nppdvjthqldpwncqszvftbrmjlhg".as_bytes()), 23);
    assert_eq!(find_idx::<14>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes()), 29);
    assert_eq!(find_idx::<14>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes()), 26);
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let bytes: Vec<u8> = input.bytes().collect();

    find_idx::<14>(&bytes).to_string()
}

#[cfg(test)]
mod benches {
    use test::Bencher;
    use crate::get_input;
    use crate::year_2022::day6::*;

    #[bench]
    fn part1(b: &mut Bencher) {
        let input = get_input(2022, 6).unwrap();
        b.iter(|| {
            find_idx::<4>(input.as_bytes())
        })
    }

    #[bench]
    fn part2(b: &mut Bencher) {
        let input = get_input(2022, 6).unwrap();
        b.iter(|| {
            find_idx::<14>(input.as_bytes())
        })
    }
}
