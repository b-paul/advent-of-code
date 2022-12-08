use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::collections::VecDeque;

#[derive(Debug, Default)]
struct Directory {
    name: String,
    dirs: Vec<Directory>,
    size: u64,
}

fn add_dir(dir: &mut Directory, mut path: Vec<String>, name: &str) {
    if path.is_empty() {
        dir.dirs.push(Directory {
            name: name.to_string(),
            dirs: vec![],
            size: 0,
        })
    } else {
        for dir in dir.dirs.iter_mut() {
            if dir.name == path[0] {
                path.remove(0);
                add_dir(dir, path, name);
                break;
            }
        }
    }
}

fn add_file(dir: &mut Directory, mut path: Vec<String>, size: u64) {
    if path.is_empty() {
        dir.size += size;
    } else {
        for dir in dir.dirs.iter_mut() {
            if dir.name == path[0] {
                path.remove(0);
                add_file(dir, path, size);
                break;
            }
        }
    }
}

impl Directory {
    fn size(&self) -> u64 {
        let mut size = self.size;

        for dir in &self.dirs {
            size += dir.size();
        }

        size
    }
}

fn sum_small_dirs(dir: &Directory) -> u64 {
    let mut total = 0;
    if dir.size() < 100000 {
        total += dir.size();
    }
    for dir in &dir.dirs {
        total += sum_small_dirs(dir);
    }
    total
}

pub fn part_1(reader: BufReader<File>) -> String {
    let mut root_dir = Directory::default();
    let mut current_dir = vec![];

    for line in reader.lines().skip(1).map(|l| l.unwrap()) {
        let parts = line.split(' ').collect::<Vec<&str>>();
        if parts[0].starts_with('$') {
            if parts[1] == "cd" {
                if parts[2] == ".." {
                    current_dir.pop();
                } else {
                    current_dir.push(parts[2].to_string());
                }
            }
            // We don't care if the command says ls because output that doesn't start with a $ can
            // only be from calling an ls
        } else if parts[0] == "dir" {
            add_dir(&mut root_dir, current_dir.clone(), parts[1]);
        } else {
            add_file(&mut root_dir, current_dir.clone(), parts[0].parse().unwrap());
        }
    }

    sum_small_dirs(&root_dir).to_string()
}

fn smallest_big_enough_dir(dir: &Directory, needed: u64) -> u64 {
    let mut min = 99999999999999999;

    if dir.size() > needed {
        min = dir.size();
    }

    for dir in &dir.dirs {
        let new = smallest_big_enough_dir(dir, needed);
        if new < min {
            min = new;
        }
    }

    min
}

pub fn part_2(reader: BufReader<File>) -> String {
    let mut root_dir = Directory::default();
    let mut current_dir = vec![];

    for line in reader.lines().skip(1).map(|l| l.unwrap()) {
        let parts = line.split(' ').collect::<Vec<&str>>();
        if parts[0].starts_with('$') {
            if parts[1] == "cd" {
                if parts[2] == ".." {
                    current_dir.pop();
                } else {
                    current_dir.push(parts[2].to_string());
                }
            }
            // We don't care if the command says ls because output that doesn't start with a $ can
            // only be from calling an ls
        } else if parts[0] == "dir" {
            add_dir(&mut root_dir, current_dir.clone(), parts[1]);
        } else {
            add_file(&mut root_dir, current_dir.clone(), parts[0].parse().unwrap());
        }
    }

    let needed = 30000000 - (70000000 - root_dir.size());
    smallest_big_enough_dir(&root_dir, needed).to_string()
}
