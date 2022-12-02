use std::fs::File;
use std::io::{BufReader, Read};

pub fn part_1(mut reader: BufReader<File>) -> String {
    let mut input = vec![];
    reader.read_to_end(&mut input).unwrap();

    let mut floor = 0;

    for ch in input {
        if ch == b'(' {
            floor += 1;
        } else if ch == b')' {
            floor -= 1;
        }
    }

    floor.to_string()
}

pub fn part_2(mut reader: BufReader<File>) -> String {
    let mut input = vec![];
    reader.read_to_end(&mut input).unwrap();

    let mut floor = 0;

    for (i, ch) in input.iter().enumerate() {
        if *ch == b'(' {
            floor += 1;
        } else if *ch == b')' {
            floor -= 1;
        }
        if floor < 0 {
            // Answer is 1 indexed so we need to add 1 to the answer
            return (i + 1).to_string();
        }
    }

    floor.to_string()
}
