pub fn part_1(input: &str) -> impl std::fmt::Display {
    input.lines().map(|l| {
        let n = l.parse::<i32>().unwrap();
        (n / 3) - 2
    }).sum::<i32>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    input.lines().map(|l| {
        let mut n = l.parse::<i32>().unwrap();
        let mut r = 0;
        loop {
            n /= 3;
            n -= 2;
            if n <= 0 {
                break;
            }
            r += n;
        }
        r
    }).sum::<i32>()
}
