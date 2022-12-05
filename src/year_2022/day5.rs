use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::collections::VecDeque;

pub fn part_1(reader: BufReader<File>) -> String {
    let lines = reader.lines().map(|l| l.unwrap());

    let mut reading_crates = true;

    let mut stack: Vec<VecDeque<char>> = vec![];

    for line in lines {
        if reading_crates {
            if line.as_bytes()[1] == b'1' {
                reading_crates = false;
            }
            for (i, bytes) in (line + " ").as_bytes().array_chunks::<4>().enumerate() {
                if stack.len() <= i {
                    stack.push(VecDeque::new())
                }
                if bytes[1] != b' ' {
                    stack[i].push_back(bytes[1] as char);
                }
            }
        } else {
            // First line will be a blank line
            if line.len() != 0 {
                let mut words = line.split_whitespace();
                words.next();
                let count = words.next().unwrap().parse::<usize>().unwrap();
                words.next();
                let start = words.next().unwrap().parse::<usize>().unwrap();
                words.next();
                let end = words.next().unwrap().parse::<usize>().unwrap();

                for _ in 0..count {
                    let item = stack[start - 1].pop_front().unwrap();
                    stack[end - 1].push_front(item);
                }
            }
        }
    }

    stack.iter().map(|v| v[0]).fold("".to_string(), |s, c| s + &c.to_string())
}

pub fn part_2(reader: BufReader<File>) -> String {
    let lines = reader.lines().map(|l| l.unwrap());

    let mut reading_crates = true;

    let mut stack: Vec<VecDeque<char>> = vec![];

    for line in lines {
        if reading_crates {
            if line.as_bytes()[1] == b'1' {
                reading_crates = false;
            }
            for (i, bytes) in (line + " ").as_bytes().array_chunks::<4>().enumerate() {
                if stack.len() <= i {
                    stack.push(VecDeque::new())
                }
                if bytes[1] != b' ' {
                    stack[i].push_back(bytes[1] as char);
                }
            }
        } else {
            // First line will be a blank line
            if line.len() != 0 {
                let mut words = line.split_whitespace();
                words.next();
                let count = words.next().unwrap().parse::<usize>().unwrap();
                words.next();
                let start = words.next().unwrap().parse::<usize>().unwrap();
                words.next();
                let end = words.next().unwrap().parse::<usize>().unwrap();

                let mut vec = VecDeque::new();
                for _ in 0..count {
                    let item = stack[start - 1].pop_front().unwrap();
                    vec.push_front(item);
                }
                for item in vec {
                    stack[end - 1].push_front(item);
                }
            }
        }
    }

    stack.iter().map(|v| v[0]).fold("".to_string(), |s, c| s + &c.to_string())
}
