#[macro_export]
macro_rules! make_year {
    () => {
        pub mod day1;
        pub mod day2;
        pub mod day3;
        pub mod day4;
        pub mod day5;
        pub mod day6;
        pub mod day7;
        pub mod day8;
        pub mod day9;
        pub mod day10;
        pub mod day11;
        pub mod day12;
        pub mod day13;
        pub mod day14;
        pub mod day15;
        pub mod day16;
        pub mod day17;
        pub mod day18;
        pub mod day19;
        pub mod day20;
        pub mod day21;
        pub mod day22;
        pub mod day23;
        pub mod day24;
        pub mod day25;
    }
}

#[macro_export]
macro_rules! run_year {
    ($file:expr, $year:ident, $day:expr, $part:expr) => {
        match $day {
            1 => run_part!($file, $year, day1, $part),
            2 => run_part!($file, $year, day2, $part),
            3 => run_part!($file, $year, day3, $part),
            4 => run_part!($file, $year, day4, $part),
            5 => run_part!($file, $year, day5, $part),
            6 => run_part!($file, $year, day6, $part),
            7 => run_part!($file, $year, day7, $part),
            8 => run_part!($file, $year, day8, $part),
            9 => run_part!($file, $year, day9, $part),
            10 => run_part!($file, $year, day10, $part),
            11 => run_part!($file, $year, day11, $part),
            12 => run_part!($file, $year, day12, $part),
            13 => run_part!($file, $year, day13, $part),
            14 => run_part!($file, $year, day14, $part),
            15 => run_part!($file, $year, day15, $part),
            16 => run_part!($file, $year, day16, $part),
            17 => run_part!($file, $year, day17, $part),
            18 => run_part!($file, $year, day18, $part),
            19 => run_part!($file, $year, day19, $part),
            20 => run_part!($file, $year, day20, $part),
            21 => run_part!($file, $year, day21, $part),
            22 => run_part!($file, $year, day22, $part),
            23 => run_part!($file, $year, day23, $part),
            24 => run_part!($file, $year, day24, $part),
            25 => run_part!($file, $year, day25, $part),
            _ => unreachable!()
        }
    }
}

#[macro_export]
macro_rules! run_part {
    ($file:expr, $year:ident, $day:ident, $part:expr) => {
        match $part {
            false => $year::$day::part_1($file).to_string(),
            true => $year::$day::part_2($file).to_string(),
        }
    }
}
