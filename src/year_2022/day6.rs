use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn find_header_idx(buf: &[u8]) -> usize {
    let mut mask: u64 = (1 << (buf[0] - b'a'))
        ^ (1 << (buf[1] - b'a'))
        ^ (1 << (buf[2] - b'a'))
        ^ (1 << (buf[3] - b'a'));
    if mask.count_ones() == 4 {
        return 1;
    }

    for i in 4..buf.len() {
        mask ^= 1 << (buf[i - 4] - b'a');
        mask ^= 1 << (buf[i] - b'a');
        if mask.count_ones() == 4 {
            return i + 1;
        }
    }
    unreachable!()
}

#[test]
fn header_test() {
    assert_eq!(find_header_idx("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes()), 5);
    assert_eq!(find_header_idx("nppdvjthqldpwncqszvftbrmjlhg".as_bytes()), 6);
    assert_eq!(find_header_idx("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes()), 10);
    assert_eq!(find_header_idx("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes()), 11);
}

pub fn part_1(reader: BufReader<File>) -> String {
    let bytes: Vec<u8> = reader.bytes().map(|b| b.unwrap()).collect();

    find_header_idx(&bytes).to_string()
}

fn find_message_idx(buf: &[u8]) -> usize {
    let mut mask: u64 = 0;
    for i in 0..14 {
        mask ^= 1 << (buf[i] - b'a');
    }
    if mask.count_ones() == 14 {
        return 1;
    }

    for i in 14..buf.len() {
        mask ^= 1 << (buf[i - 14] - b'a');
        mask ^= 1 << (buf[i] - b'a');
        if mask.count_ones() == 14 {
            return i + 1;
        }
    }
    unreachable!()
}

#[test]
fn message_test() {
    assert_eq!(find_message_idx("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes()), 23);
    assert_eq!(find_message_idx("nppdvjthqldpwncqszvftbrmjlhg".as_bytes()), 23);
    assert_eq!(find_message_idx("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes()), 29);
    assert_eq!(find_message_idx("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes()), 26);
}

pub fn part_2(reader: BufReader<File>) -> String {
    let bytes: Vec<u8> = reader.bytes().map(|b| b.unwrap()).collect();

    find_message_idx(&bytes).to_string()
}

#[cfg(test)]
mod benches {
    use test::Bencher;
    use crate::year_2022::day6::*;

    #[bench]
    fn part1(b: &mut Bencher) {
        b.iter(|| {
            find_header_idx(include_str!("../../input/2022/6.txt").as_bytes())
        })
    }

    #[bench]
    fn part2(b: &mut Bencher) {
        b.iter(|| {
            find_message_idx(include_str!("../../input/2022/6.txt").as_bytes())
        })
    }
}
