use crate::day::Day;
use crate::make_day;

// Amount of days that have problems so far this year
pub const DAYS_NUM: usize = 6;

pub const DAYS: [Day; DAYS_NUM] = [
    make_day!(day1),
    make_day!(day2),
    make_day!(day3),
    make_day!(day4),
    make_day!(day5),
    make_day!(day6),
];

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
