use std::collections::{HashMap, HashSet, VecDeque};

fn has(blizzards: &HashSet<(usize, usize, i32)>, x: usize, y: usize) -> bool {
    blizzards.contains(&(x, y, 0))
        || blizzards.contains(&(x, y, 1))
        || blizzards.contains(&(x, y, 2))
        || blizzards.contains(&(x, y, 3))
}

fn print(blizzards: &HashSet<(usize, usize, i32)>, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            let mut count = 0;
            if blizzards.contains(&(x, y, 0)) {
                count += 1;
            }
            if blizzards.contains(&(x, y, 1)) {
                count += 1;
            }
            if blizzards.contains(&(x, y, 2)) {
                count += 1;
            }
            if blizzards.contains(&(x, y, 3)) {
                count += 1;
            }
            if count == 1 {
                if blizzards.contains(&(x, y, 0)) {
                    print!(">");
                } else if blizzards.contains(&(x, y, 1)) {
                    print!("<");
                } else if blizzards.contains(&(x, y, 2)) {
                    print!("v");
                } else if blizzards.contains(&(x, y, 3)) {
                    print!("^");
                }
            } else if count != 0 {
                print!("{count}");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
    println!();
}

fn step(blizzards: &mut HashSet<(usize, usize, i32)>, width: usize, height: usize) {
    let mut new_blizzards = HashSet::new();
    for (x, y, dir) in blizzards.clone().iter() {
        match dir {
            0 => {
                if *x == width - 1 {
                    new_blizzards.insert((0, *y, *dir));
                } else {
                    new_blizzards.insert((*x + 1, *y, *dir));
                }
            }
            1 => {
                if *x == 0 {
                    new_blizzards.insert((width - 1, *y, *dir));
                } else {
                    new_blizzards.insert((*x - 1, *y, *dir));
                }
            }
            2 => {
                if *y == height - 1 {
                    new_blizzards.insert((*x, 0, *dir));
                } else {
                    new_blizzards.insert((*x, *y + 1, *dir));
                }
            }
            3 => {
                if *y == 0 {
                    new_blizzards.insert((*x, height - 1, *dir));
                } else {
                    new_blizzards.insert((*x, *y - 1, *dir));
                }
            }
            _ => unreachable!(),
        }
    }

    *blizzards = new_blizzards.clone();
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut blizzards = HashSet::new();
    let mut width = 0;
    let mut height = 0;

    for (y, line) in input.lines().skip(1).enumerate() {
        height = height.max(y);
        for (x, ch) in line.chars().skip(1).enumerate() {
            width = width.max(x);
            match ch {
                '>' => {
                    blizzards.insert((x, y, 0));
                }
                '<' => {
                    blizzards.insert((x, y, 1));
                }
                'v' => {
                    blizzards.insert((x, y, 2));
                }
                '^' => {
                    blizzards.insert((x, y, 3));
                }
                _ => (),
            }
        }
    }

    let mut curtime = 0;
    //print(&blizzards, width, height);
    step(&mut blizzards, width, height);
    //print(&blizzards, width, height);
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((0, 0, 1));
    visited.insert((0, 0, 1));
    while let Some((x, y, depth)) = queue.pop_front() {
        if depth != curtime {
            curtime += 1;
            step(&mut blizzards, width, height);
            //print(&blizzards, width, height);
            //println!("{} {}", depth, curtime);

            let insert = (0, 0, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }

        //println!("{} {} {}", x, y, depth);

        if x == width - 1 && y == height - 1 {
            return (depth + 1).to_string();
        }

        if x < width - 1 && !has(&blizzards, x + 1, y) {
            let insert = (x + 1, y, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }
        if x > 0 && !has(&blizzards, x - 1, y) {
            let insert = (x - 1, y, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }
        if y < height - 1 && !has(&blizzards, x, y + 1) {
            let insert = (x, y + 1, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }
        if y > 0 && !has(&blizzards, x, y - 1) {
            let insert = (x, y - 1, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }
        if !has(&blizzards, x, y) {
            let insert = (x, y, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }
    }

    unreachable!()
}

#[test]
fn testp1() {
    assert_eq!(
        part_1(
            "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"
        ).to_string(),
        18.to_string()
    );
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut blizzards = HashSet::new();
    let mut width = 0;
    let mut height = 0;

    for (y, line) in input.lines().skip(1).enumerate() {
        height = height.max(y);
        for (x, ch) in line.chars().skip(1).enumerate() {
            width = width.max(x);
            match ch {
                '>' => {
                    blizzards.insert((x, y, 0));
                }
                '<' => {
                    blizzards.insert((x, y, 1));
                }
                'v' => {
                    blizzards.insert((x, y, 2));
                }
                '^' => {
                    blizzards.insert((x, y, 3));
                }
                _ => (),
            }
        }
    }

    let mut curtime = 0;
    //print(&blizzards, width, height);
    step(&mut blizzards, width, height);
    //print(&blizzards, width, height);

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((0, 0, 1));
    visited.insert((0, 0, 1));
    let mut travel_time = 0;
    while let Some((x, y, depth)) = queue.pop_front() {
        if depth != curtime {
            curtime += 1;
            step(&mut blizzards, width, height);
            //print(&blizzards, width, height);
            //println!("{} {}", depth, curtime);

            let insert = (0, 0, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }

        if x == width - 1 && y == height - 1 {
            travel_time = depth+1;
            break;
        }

        if x < width - 1 && !has(&blizzards, x + 1, y) {
            let insert = (x + 1, y, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }
        if x > 0 && !has(&blizzards, x - 1, y) {
            let insert = (x - 1, y, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }
        if y < height - 1 && !has(&blizzards, x, y + 1) {
            let insert = (x, y + 1, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }
        if y > 0 && !has(&blizzards, x, y - 1) {
            let insert = (x, y - 1, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }
        if !has(&blizzards, x, y) {
            let insert = (x, y, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((width-1, height-1, travel_time+1));
    visited.insert((width-1, height-1, travel_time+1));
    while let Some((x, y, depth)) = queue.pop_front() {
        if depth != curtime {
            curtime += 1;
            step(&mut blizzards, width, height);
            //print(&blizzards, width, height);
            //println!("{} {}", depth, curtime);

            let insert = (width-1, height-1, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }

        if x == 0 && y == 0 {
            travel_time = depth+1;
            break;
        }

        if x < width - 1 && !has(&blizzards, x + 1, y) {
            let insert = (x + 1, y, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }
        if x > 0 && !has(&blizzards, x - 1, y) {
            let insert = (x - 1, y, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }
        if y < height - 1 && !has(&blizzards, x, y + 1) {
            let insert = (x, y + 1, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }
        if y > 0 && !has(&blizzards, x, y - 1) {
            let insert = (x, y - 1, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }
        if !has(&blizzards, x, y) {
            let insert = (x, y, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((0, 0, travel_time+1));
    visited.insert((0, 0, travel_time+1));
    let mut travel_time = 0;
    while let Some((x, y, depth)) = queue.pop_front() {
        if depth != curtime {
            curtime += 1;
            step(&mut blizzards, width, height);
            //print(&blizzards, width, height);
            //println!("{} {}", depth, curtime);

            let insert = (0, 0, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }

        if x == width - 1 && y == height - 1 {
            travel_time = depth+1;
            break;
        }

        if x < width - 1 && !has(&blizzards, x + 1, y) {
            let insert = (x + 1, y, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }
        if x > 0 && !has(&blizzards, x - 1, y) {
            let insert = (x - 1, y, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }
        if y < height - 1 && !has(&blizzards, x, y + 1) {
            let insert = (x, y + 1, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }
        if y > 0 && !has(&blizzards, x, y - 1) {
            let insert = (x, y - 1, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }
        if !has(&blizzards, x, y) {
            let insert = (x, y, depth + 1);
            if !visited.contains(&insert) {
                visited.insert(insert);
                queue.push_back(insert);
            }
        }
    }
    travel_time.to_string()
}

#[test]
fn testp2() {}
