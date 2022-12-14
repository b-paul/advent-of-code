use std::collections::HashSet;

pub fn part_1(input: String) -> String {
    let mut grid: HashSet<(isize, isize)> = HashSet::new();
    let mut maxy = 0;

    for line in input.lines() {
        let points = line
            .split(" -> ")
            .map(|p| {
                p.split(',')
                    .map(|n| n.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
            })
            .map(|v| {
                if v[1] > maxy {
                    maxy = v[1];
                }
                (v[0], v[1])
            })
            .collect::<Vec<(isize, isize)>>();
        for i in 0..points.len() - 1 {
            let p1 = points[i];
            let p2 = points[i + 1];
            if p1.0 == p2.0 {
                if p2.1 > p1.1 {
                    for y in p1.1..=p2.1 {
                        grid.insert((p1.0, y));
                    }
                } else {
                    for y in p2.1..=p1.1 {
                        grid.insert((p1.0, y));
                    }
                }
            } else {
                if p2.0 > p1.0 {
                    for x in p1.0..=p2.0 {
                        grid.insert((x, p1.1));
                    }
                } else {
                    for x in p2.0..=p1.0 {
                        grid.insert((x, p1.1));
                    }
                }
            }
        }
    }

    for num in 0.. {
        let mut x = 500;
        let mut y = 0;
        loop {
            if y > maxy {
                return num.to_string();
            }
            if !grid.contains(&(x, y + 1)) {
                y += 1;
            } else if !grid.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else if !grid.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                grid.insert((x, y));
                break;
            }
        }
    }

    unreachable!()
}

#[test]
fn testp1() {
    assert_eq!(
        part_1(
            "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
                .to_string()
        ),
        24.to_string()
    );
}

pub fn part_2(input: String) -> String {
    let mut grid: HashSet<(isize, isize)> = HashSet::new();
    let mut maxy = 0;

    for line in input.lines() {
        let points = line
            .split(" -> ")
            .map(|p| {
                p.split(',')
                    .map(|n| n.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
            })
            .map(|v| {
                if v[1] > maxy {
                    maxy = v[1];
                }
                (v[0], v[1])
            })
            .collect::<Vec<(isize, isize)>>();
        for i in 0..points.len() - 1 {
            let p1 = points[i];
            let p2 = points[i + 1];
            if p1.0 == p2.0 {
                if p2.1 > p1.1 {
                    for y in p1.1..=p2.1 {
                        grid.insert((p1.0, y));
                    }
                } else {
                    for y in p2.1..=p1.1 {
                        grid.insert((p1.0, y));
                    }
                }
            } else {
                if p2.0 > p1.0 {
                    for x in p1.0..=p2.0 {
                        grid.insert((x, p1.1));
                    }
                } else {
                    for x in p2.0..=p1.0 {
                        grid.insert((x, p1.1));
                    }
                }
            }
        }
    }

    for num in 0.. {
        let mut x = 500;
        let mut y = 0;
        loop {
            if grid.contains(&(500, 0)) {
                return num.to_string();
            }
            if y == maxy + 1 {
                grid.insert((x, y));
                break;
            }
            if !grid.contains(&(x, y + 1)) {
                y += 1;
            } else if !grid.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else if !grid.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                grid.insert((x, y));
                break;
            }
        }
    }

    unreachable!()
}

#[test]
fn testp2() {
    assert_eq!(
        part_2(
            "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
                .to_string()
        ),
        93.to_string()
    );
}
