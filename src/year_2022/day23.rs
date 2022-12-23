use std::collections::HashSet;

pub fn part_1(input: String) -> String {
    let mut elves = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, b) in line.bytes().enumerate() {
            if b == b'#' {
                elves.insert((x as isize, y as isize));
            }
        }
    }

    let mut new_state = HashSet::new();

    for round in 0..10 {
        for (x, y) in &elves {
            let mut b = true;
            for xoff in -1..=1 {
                for yoff in -1..=1 {
                    if xoff == 0 && yoff == 0 {
                        continue;
                    }

                    b &= !elves.contains(&(*x + xoff, *y + yoff));
                }
            }
            if b {
                continue;
            }

            for dir in 0..4 {
                match (round + dir) % 4 {
                    0 => {
                        if elves.contains(&(*x - 1, *y - 1))
                            || elves.contains(&(*x, *y - 1))
                            || elves.contains(&(*x + 1, *y - 1))
                        {
                            continue;
                        }
                        new_state.insert(((*x, *y), (*x, *y - 1)));
                        break;
                    }
                    1 => {
                        if elves.contains(&(*x - 1, *y + 1))
                            || elves.contains(&(*x, *y + 1))
                            || elves.contains(&(*x + 1, *y + 1))
                        {
                            continue;
                        }
                        new_state.insert(((*x, *y), (*x, *y + 1)));
                        break;
                    }
                    2 => {
                        if elves.contains(&(*x - 1, *y - 1))
                            || elves.contains(&(*x - 1, *y))
                            || elves.contains(&(*x - 1, *y + 1))
                        {
                            continue;
                        }
                        new_state.insert(((*x, *y), (*x - 1, *y)));
                        break;
                    }
                    3 => {
                        if elves.contains(&(*x + 1, *y - 1))
                            || elves.contains(&(*x + 1, *y))
                            || elves.contains(&(*x + 1, *y + 1))
                        {
                            continue;
                        }
                        new_state.insert(((*x, *y), (*x + 1, *y)));
                        break;
                    }
                    _ => (),
                }
            }
        }
        for ((fx, fy), (tx, ty)) in &new_state {
            if new_state
                .iter()
                .filter(|(_, (x, y))| x == tx && y == ty)
                .count()
                == 1
            {
                elves.remove(&(*fx, *fy));
                elves.insert((*tx, *ty));
            }
        }

        new_state.clear();
    }
    let mut minx = 999999;
    let mut miny = 999999;
    let mut maxx = -999999;
    let mut maxy = -999999;

    for (x, y) in &elves {
        minx = minx.min(*x);
        miny = miny.min(*y);
        maxx = maxx.max(*x);
        maxy = maxy.max(*y);
    }

    let mut total = 0;
    for x in minx..=maxx {
        for y in miny..=maxy {
            if !elves.contains(&(x, y)) {
                total += 1;
            }
        }
    }

    total.to_string()
}

#[test]
fn testp1() {
    assert_eq!(
        part_1(
            "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."
                .to_string()
        ),
        110.to_string()
    );
}

pub fn part_2(input: String) -> String {
    let mut elves = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, b) in line.bytes().enumerate() {
            if b == b'#' {
                elves.insert((x as isize, y as isize));
            }
        }
    }

    let mut new_state = HashSet::new();

    for round in 0.. {
        for (x, y) in &elves {
            let mut b = true;
            for xoff in -1..=1 {
                for yoff in -1..=1 {
                    if xoff == 0 && yoff == 0 {
                        continue;
                    }

                    b &= !elves.contains(&(*x + xoff, *y + yoff));
                }
            }
            if b {
                continue;
            }

            for dir in 0..4 {
                match (round + dir) % 4 {
                    0 => {
                        if elves.contains(&(*x - 1, *y - 1))
                            || elves.contains(&(*x, *y - 1))
                            || elves.contains(&(*x + 1, *y - 1))
                        {
                            continue;
                        }
                        new_state.insert(((*x, *y), (*x, *y - 1)));
                        break;
                    }
                    1 => {
                        if elves.contains(&(*x - 1, *y + 1))
                            || elves.contains(&(*x, *y + 1))
                            || elves.contains(&(*x + 1, *y + 1))
                        {
                            continue;
                        }
                        new_state.insert(((*x, *y), (*x, *y + 1)));
                        break;
                    }
                    2 => {
                        if elves.contains(&(*x - 1, *y - 1))
                            || elves.contains(&(*x - 1, *y))
                            || elves.contains(&(*x - 1, *y + 1))
                        {
                            continue;
                        }
                        new_state.insert(((*x, *y), (*x - 1, *y)));
                        break;
                    }
                    3 => {
                        if elves.contains(&(*x + 1, *y - 1))
                            || elves.contains(&(*x + 1, *y))
                            || elves.contains(&(*x + 1, *y + 1))
                        {
                            continue;
                        }
                        new_state.insert(((*x, *y), (*x + 1, *y)));
                        break;
                    }
                    _ => (),
                }
            }
        }

        if new_state.len() == 0 {
            return (round + 1).to_string();
        }

        for ((fx, fy), (tx, ty)) in &new_state {
            if new_state
                .iter()
                .filter(|(_, (x, y))| x == tx && y == ty)
                .count()
                == 1
            {
                elves.remove(&(*fx, *fy));
                elves.insert((*tx, *ty));
            }
        }
        // Uncomment to visualize
        /*
        let mut minx = 999999;
        let mut miny = 999999;
        let mut maxx = -999999;
        let mut maxy = -999999;

        for (x, y) in &elves {
            minx = minx.min(*x);
            miny = miny.min(*y);
            maxx = maxx.max(*x);
            maxy = maxy.max(*y);
        }

        for y in miny..=maxy {
            for x in minx..=maxx {
                if elves.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
        println!();
        */

        new_state.clear();
    }
    let mut minx = 999999;
    let mut miny = 999999;
    let mut maxx = -999999;
    let mut maxy = -999999;

    for (x, y) in &elves {
        minx = minx.min(*x);
        miny = miny.min(*y);
        maxx = maxx.max(*x);
        maxy = maxy.max(*y);
    }

    let mut total = 0;
    for x in minx..=maxx {
        for y in miny..=maxy {
            if !elves.contains(&(x, y)) {
                total += 1;
            }
        }
    }

    total.to_string()
}

#[test]
fn testp2() {}
