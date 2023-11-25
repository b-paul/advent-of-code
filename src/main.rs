#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(exclusive_range_pattern)]
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

use clap::Parser;

fn run_part1(file: &str, day: &day::Day) -> std::io::Result<String> {
    let file = read_to_string(file)?;
    Ok((day.part1)(file))
}

fn run_part2(file: &str, day: &day::Day) -> std::io::Result<String> {
    let file = read_to_string(file)?;
    Ok((day.part2)(file))
}

/*
macro_rules! run_year {
    ($year:ident, $year_num:expr) => {
        for (i, day) in $year::DAYS.iter().enumerate() {
            let input_file = format!("input/{}/{}.txt", $year_num, i + 1);

            println!("Day {} part 1: {}", i + 1, run_part1(&input_file, day)?);
            println!("Day {} part 2: {}", i + 1, run_part2(&input_file, day)?);
        }
    };
}
*/

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 2022, value_parser = clap::value_parser!(u32).range(2015..=2022))]
    year: u32,

    /// Run part 2 of this problem (runs part 1 if this flag is not given)
    #[arg(short = '2')]
    part: bool,

    #[arg(value_parser = clap::value_parser!(u32).range(1..=25))]
    day: u32,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let day = match args.year {
        2015 => &year_2015::DAYS[args.day as usize - 1],
        2016 => &year_2016::DAYS[args.day as usize - 1],
        2017 => &year_2017::DAYS[args.day as usize - 1],
        2018 => &year_2018::DAYS[args.day as usize - 1],
        2019 => &year_2019::DAYS[args.day as usize - 1],
        2020 => &year_2020::DAYS[args.day as usize - 1],
        2021 => &year_2021::DAYS[args.day as usize - 1],
        2022 => &year_2022::DAYS[args.day as usize - 1],
        _ => unreachable!(),
    };

    let part = match args.part {
        false => day.part1,
        true => day.part2,
    };

    let input_file = format!("input/{}/{}.txt", args.year, args.day);
    let file = read_to_string(input_file)?;

    println!(
        "Day {} part {}: {}",
        args.day,
        args.part as usize + 1,
        part(file)
    );

    Ok(())
}
