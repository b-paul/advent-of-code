use std::fs::File;
use std::io::{BufRead, BufReader};

fn score_part1(game: &str) -> u64 {
    let bytes = game.as_bytes();
    let them = (bytes[0] - b'A') as i8;
    let us = (bytes[2] - b'X') as i8;

    // Them = 0 when they choose rock, 1 when they choose paper and 2 when scissors
    // Same with us
    // Consider the cases that we win:
    // us = 0, them = 2
    // us = 1, them = 0,
    // us = 2, them = 1,
    // Us is always one larger than them!
    // This means that (us - them) mod 3 == 1
    // (We have to add 3 to this so that negative modulos dont act weird)
    // When there is a draw, (us - them) mod 3 == 0
    // When there is a loss, (us - them) mod 3 == 2
    // If we add one to this modulo expression, we get the following:
    // When there is a win,  (1 + us - them) mod 3 == 2
    // When there is a draw, (1 + us - them) mod 3 == 1
    // When there is a loss, (1 + us - them) mod 3 == 0
    // Again, note that there is a 4 because of the weird negative modulo thing
    // With this formula, we can multiply the modulo result by 3 to get the score of a
    // win/loss/draw!
    // When there is a win,  ((1 + us - them) mod 3)*3 == 6
    // When there is a draw, ((1 + us - them) mod 3)*3 == 3
    // When there is a loss, ((1 + us - them) mod 3)*3 == 0
    // Then we just add the score that comes from whatever us was
    // (+1 for rock, +2 for paper, +3 for scissors), and that is the answer!!

    (1 + us + ((4 + us - them) % 3) * 3) as u64
}

#[test]
fn scoring_part1_test() {
    assert_eq!(score_part1("A Y"), 8);
    assert_eq!(score_part1("B X"), 1);
    assert_eq!(score_part1("C Z"), 6);
}

pub fn part_1(reader: BufReader<File>) -> String {
    reader
        .lines()
        .map(|line| score_part1(line.unwrap().as_str()))
        .sum::<u64>()
        .to_string()
}

fn score_part2(game: &str) -> u64 {
    let bytes = game.as_bytes();
    let them = (bytes[0] - b'A') as i8;
    let result = (bytes[2] - b'X') as i8;

    // You can solve for the us variable by using the fact that:
    // (us - them == result - 1) mod 3
    // then math blahblah

    (3 * result + 1 + (them + result + 2) % 3) as u64
}

#[test]
fn scoring_part2_test() {
    assert_eq!(score_part2("A Y"), 4);
    assert_eq!(score_part2("B X"), 1);
    assert_eq!(score_part2("C Z"), 7);
}

pub fn part_2(reader: BufReader<File>) -> String {
    reader
        .lines()
        .map(|line| score_part2(line.unwrap().as_str()))
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod benches {
    use crate::test::Bencher;
    use crate::year_2022::day2::*;

    #[bench]
    fn part1(b: &mut Bencher) {
        b.iter(|| {
            include_str!("../../input/2022/2.txt").to_string()
                .lines()
                .map(|line| score_part1(line))
                .sum::<u64>()
                .to_string()
        });
    }

    #[bench]
    fn part2(b: &mut Bencher) {
        b.iter(|| {
            include_str!("../../input/2022/2.txt").to_string()
                .lines()
                .map(|line| score_part2(line))
                .sum::<u64>()
                .to_string()
        });
    }
}
