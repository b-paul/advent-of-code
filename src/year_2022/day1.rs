use std::fs::File;
use std::io::{BufReader, Read};

fn elf_callories(str: &str) -> u64 {
    let mut total = 0;

    for line in str.split('\n') {
        let num: u64 = line.parse().unwrap();

        total += num;
    }

    total
}

pub fn part_1(mut reader: BufReader<File>) -> String {
    let mut str = String::new();
    reader.read_to_string(&mut str).unwrap();

    let mut str = str.chars();
    str.next_back();
    let elves = str.as_str().split("\n\n");

    let most_callories = elves.into_iter().map(elf_callories).max().unwrap();

    most_callories.to_string()
}

pub fn part_2(mut reader: BufReader<File>) -> String {
    let mut str = String::new();
    reader.read_to_string(&mut str).unwrap();

    let mut str = str.chars();
    str.next_back();
    let elves = str.as_str().split("\n\n");

    // elves with the most callories ranked
    let mut elf1 = 0;
    let mut elf2 = 0;
    let mut elf3 = 0;

    for elf in elves {
        let elf_callories = elf_callories(elf);
        if elf_callories > elf1 {
            elf3 = elf2;
            elf2 = elf1;
            elf1 = elf_callories;
        } else if elf_callories > elf2 {
            elf2 = elf_callories;
            elf3 = elf2;
        } else if elf_callories > elf3 {
            elf3 = elf_callories;
        }
    }

    (elf1 + elf2 + elf3).to_string()
}
