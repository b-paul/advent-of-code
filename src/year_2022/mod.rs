use crate::day::Day;
use crate::make_day;

// Amount of days that have problems so far this year
pub const DAYS_NUM: usize = 3;

pub const DAYS: [Day; DAYS_NUM] = [make_day!(day1), make_day!(day2), make_day!(day3)];

mod day1;
mod day2;
mod day3;
