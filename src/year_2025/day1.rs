use crate::helper::shorthand::p;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .fold((0, 50), |(n, t), s| {
            let ticks = p::<i32>(&s[1..]);
            let t = match &s[0..1] {
                "L" => t - ticks,
                "R" => t + ticks,
                _ => unreachable!(),
            };
            (n + (t % 100 == 0) as i32, t)
        })
        .0
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .fold((0, 50i32), |(n, t), s| {
            let ticks = p::<i32>(&s[1..]);
            let (dist, t) = match &s[0..1] {
                "L" => (t.rem_euclid(100), t - ticks),
                "R" => ((-t).rem_euclid(100), t + ticks),
                _ => unreachable!(),
            };
            (n + ticks / 100 + (0 < dist && dist <= ticks % 100) as i32, t)
        })
        .0
}
