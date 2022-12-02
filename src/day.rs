use std::fs::File;
use std::io::BufReader;

pub struct Day {
    pub part1: fn(BufReader<File>) -> String,
    pub part2: fn(BufReader<File>) -> String,
}

#[macro_export]
macro_rules! make_day {
    ($day:ident) => {
        Day {
            part1: $day::part_1,
            part2: $day::part_2,
        }
    };
}
