#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(iter_array_chunks)]
#![feature(array_chunks)]
#![feature(test)]
extern crate test;

mod day;
mod year_2015;
mod year_2016;
mod year_2017;
mod year_2018;
mod year_2019;
mod year_2020;
mod year_2021;
mod year_2022;

use std::fs::read_to_string;
use std::io::Read;

fn run_part1(file: &str, day: &day::Day) -> std::io::Result<String> {
    let file = read_to_string(file)?;
    Ok((day.part1)(file))
}

fn run_part2(file: &str, day: &day::Day) -> std::io::Result<String> {
    let file = read_to_string(file)?;
    Ok((day.part2)(file))
}

macro_rules! run_year {
    ($year:ident, $year_num:expr) => {
        for (i, day) in $year::DAYS.iter().enumerate() {
            let input_file = format!("input/{}/{}.txt", $year_num, i + 1);

            println!("Day {} part 1: {}", i + 1, run_part1(&input_file, day)?);
            println!("Day {} part 2: {}", i + 1, run_part2(&input_file, day)?);
        }
    };
}

fn main() -> std::io::Result<()> {
    run_year!(year_2022, 2022);

    Ok(())
}
