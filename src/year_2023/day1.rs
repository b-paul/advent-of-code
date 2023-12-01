pub fn part_1(input: &str) -> impl std::fmt::Display {
    input.lines()
        .map(|l| {
            let mut n = 0;
            for c in l.chars() {
                if c.is_digit(10) {
                    n += c.to_digit(10).unwrap();
                    break;
                }
            }
            n *= 10;
            for c in l.chars().rev() {
                if c.is_digit(10) {
                    n += c.to_digit(10).unwrap();
                    break;
                }
            }
            n
        }).sum::<u32>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    input.lines()
        .map(|l| {
            let nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
            let mut n = 0;
            'end: for (i, c) in l.chars().enumerate() {
                if c.is_digit(10) {
                    n += c.to_digit(10).unwrap();
                    break;
                }
                for (j, num) in nums.iter().enumerate() {
                    if l[i..].starts_with(num) {
                        n += j as u32 + 1;
                        break 'end;
                    }
                }
            }
            n *= 10;
            'end: for (i, c) in l.chars().rev().enumerate() {
                if c.is_digit(10) {
                    n += c.to_digit(10).unwrap();
                    break;
                }
                for (j, num) in nums.iter().enumerate() {
                    if l[(l.len() - 1 - i)..].starts_with(num) {
                        n += j as u32 + 1;
                        break 'end;
                    }
                }
            }
            n
        }).sum::<u32>()
}
