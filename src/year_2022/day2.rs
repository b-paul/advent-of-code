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

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(score_part1)
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

pub fn part_2(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(score_part2)
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod benches {
    use crate::test::Bencher;
    use crate::get_input;
    use crate::year_2022::day2::*;

    #[bench]
    fn part1(b: &mut Bencher) {
        let input = get_input(2022, 2).unwrap();
        b.iter(|| {
            input
                .lines()
                .map(score_part1)
                .sum::<u64>()
                .to_string()
        });
    }

    // this was ~6.6 times faster, but was also 2.75x faster on top of that when using
    // RUSTFLAGS="target-cpu=native" (I have a ryzen 5 2600)
    // I guess some simd stuff makes it faster
    #[bench]
    fn possibly_faster_part1(b: &mut Bencher) {
        let input = get_input(2022, 2).unwrap();
        b.iter(|| {
            let s = input.as_bytes();
            let mut total = 0;
            for i in 0..(s.len() / 4) {
                let them = (s[4*i + 0] - b'A') as i8;
                let us = (s[4*i + 2] - b'X') as i8;
                total += (1 + us + ((4 + us - them) % 3) * 3) as u64;
            }
            assert_eq!(total, 13809);
            total
        });
    }

    // like 2x faster again almost
    // This is only faster when you don't have access to some simd stuff it seems
    // i guess a lookup isn't very easy to do simd maybe? or maybe it's easy with avx512 or
    // something
    #[bench]
    fn possibly_even_faster_part1(b: &mut Bencher) {
        let input = get_input(2022, 2).unwrap();
        const SOLS: [[u8; 3]; 3] = [
            [4, 1, 7],
            [8, 5, 2],
            [3, 9, 6],
        ];
        b.iter(|| {
            let s = input.as_bytes();
            let mut total = 0;
            for i in 0..(s.len() / 4) {
                let them = (s[4*i + 0] - b'A') as usize;
                let us = (s[4*i + 2] - b'X') as usize;
                total += SOLS[us][them] as u64;
            }
            assert_eq!(total, 13809);
            total
        });
    }

    #[bench]
    fn part2(b: &mut Bencher) {
        let input = get_input(2022, 2).unwrap();
        b.iter(|| {
            input
                .lines()
                .map(score_part2)
                .sum::<u64>()
                .to_string()
        });
    }

    #[bench]
    fn possibly_faster_part2(b: &mut Bencher) {
        let input = get_input(2022, 2).unwrap();
        b.iter(|| {
            let s = input.as_bytes();
            let mut total = 0;
            for i in 0..(s.len() / 4) {
                let them = (s[4*i + 0] - b'A') as i8;
                let result = (s[4*i + 2] - b'X') as i8;
                total += (3 * result + 1 + (them + result + 2) % 3) as u64
            }
            assert_eq!(total, 12316);
            total
        });
    }

    #[bench]
    fn possibly_even_faster_part2(b: &mut Bencher) {
        let input = get_input(2022, 2).unwrap();
        const SOLS: [[u8; 3]; 3] = [
            [3, 1, 2],
            [4, 5, 6],
            [8, 9, 7],
        ];
        b.iter(|| {
            let s = input.as_bytes();
            let mut total = 0;
            for i in 0..(s.len() / 4) {
                let them = (s[4*i + 0] - b'A') as usize;
                let result = (s[4*i + 2] - b'X') as usize;
                total += SOLS[result][them] as u64;
            }
            assert_eq!(total, 12316);
            total
        });
    }
}
