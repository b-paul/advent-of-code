#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(iter_array_chunks)]
#![feature(iter_next_chunk)]
#![feature(array_chunks)]
#![feature(array_try_from_fn)]
#![feature(test)]
#![feature(portable_simd)]
extern crate test;

mod helper;
mod year;
mod year_2015;
mod year_2016;
mod year_2017;
mod year_2018;
mod year_2019;
mod year_2020;
mod year_2021;
mod year_2022;
mod year_2023;
mod year_2024;
mod year_2025;

use std::env;
use std::fs;
use std::time::Instant;

use clap::Parser;

fn get_input(year: u32, day: u32) -> anyhow::Result<String> {
    let path = format!("input/{year}/{day}.txt");
    fs::read_to_string(&path).or_else(|_| {
        let url = format!("https://adventofcode.com/{year}/day/{day}/input");
        let aoc_cookie = env::var("AOCCOOKIE")?;
        let response = ureq::get(&url)
            .set("Cookie", &aoc_cookie)
            .call()?
            .into_string()?;

        fs::write(path, &response)?;

        Ok(response)
    })
}

fn submit_answer(year: u32, day: u32, part: u32, answer: String) -> anyhow::Result<String> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/answer");
    let aoc_cookie = env::var("AOCCOOKIE")?;
    let response = ureq::post(&url)
        .set("Cookie", &aoc_cookie)
        .set("Content-Type", "application/x-www-form-urlencoded")
        .send_string(&format!("level={part}&answer={answer}"))?
        .into_string()?;
    Ok(response)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 2025, value_parser = clap::value_parser!(u32).range(2015..=2025))]
    year: u32,

    /// Run part 2 of this problem (runs part 1 if this flag is not given)
    #[arg(short = '2')]
    part: bool,

    /// Print how long the program took to execute
    #[arg(short = 't')]
    time: bool,

    /// Submit the answer with a http(s) request
    #[arg(short = 's')]
    submit: bool,

    #[arg(value_parser = clap::value_parser!(u32).range(1..=25))]
    day: u32,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let file = &get_input(args.year, args.day)?;

    let now = Instant::now();

    let result = match args.year {
        2015 => run_year!(file, year_2015, args.day, args.part),
        2016 => run_year!(file, year_2016, args.day, args.part),
        2017 => run_year!(file, year_2017, args.day, args.part),
        2018 => run_year!(file, year_2018, args.day, args.part),
        2019 => run_year!(file, year_2019, args.day, args.part),
        2020 => run_year!(file, year_2020, args.day, args.part),
        2021 => run_year!(file, year_2021, args.day, args.part),
        2022 => run_year!(file, year_2022, args.day, args.part),
        2023 => run_year!(file, year_2023, args.day, args.part),
        2024 => run_year!(file, year_2024, args.day, args.part),
        2025 => run_year!(file, year_2025, args.day, args.part),
        _ => unreachable!(),
    };

    let elapsed = now.elapsed();

    println!(
        "Day {} part {}: {}",
        args.day,
        args.part as usize + 1,
        result,
    );

    if args.time {
        println!("Ran for {elapsed:?}");
    }

    // TODO some mechanism to check whether the solution should be automatically submitted or not
    if args.submit {
        println!("submitting");
        let response = submit_answer(args.year, args.day, args.part as u32 + 1, result)?;
        let (_, response) = response
            .split_once("<main>")
            .expect("Response did not have a main body?!");
        let (response, _) = response
            .split_once("</main>")
            .expect("Response did not have a main body?!");
        println!("Response: {response}");
    }

    Ok(())
}
