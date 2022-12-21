fn remove_dupe_iter(ranges: &Vec<std::ops::Range<isize>>) -> Vec<std::ops::Range<isize>> {
    let mut new_ranges: Vec<std::ops::Range<isize>> = vec![];

    for r1 in ranges {
        let mut append = true;
        for r2 in &new_ranges {
            if r1.end <= r2.end && r1.start >= r2.start {
                append = false;
            }
        }
        if append {
            for r2 in new_ranges.iter_mut() {
                if r1.start <= r2.start && r1.end >= r2.start {
                    r2.start = r1.start;
                    append = false;
                }
                if r1.end >= r2.end && r1.start <= r2.end {
                    r2.end = r1.end;
                    append = false;
                }

                if !append {
                    break;
                }
            }
            if append {
                new_ranges.push(r1.clone());
            }
        }
    }

    new_ranges
}

pub fn part_1(input: String) -> String {
    let mut ranges = vec![];

    for line in input.lines().map(|l| l.split(' ').collect::<Vec<&str>>()) {
        let x1 = line[2][2..line[2].len() - 1].parse::<isize>().unwrap();
        let y1 = line[3][2..line[3].len() - 1].parse::<isize>().unwrap();
        let x2 = line[8][2..line[8].len() - 1].parse::<isize>().unwrap();
        let y2 = line[9][2..line[9].len()].parse::<isize>().unwrap();

        let dist = (x2 - x1).abs() + (y2 - y1).abs();

        if (2000000 - y1).abs() > dist {
            continue;
        }

        let xoff = dist - (2000000 - y1).abs();
        let range = (x1 - xoff)..(x1 + xoff);
        ranges.push(range);
    }

    let mut new_ranges = remove_dupe_iter(&ranges);
    while new_ranges != ranges {
        ranges = new_ranges;
        new_ranges = remove_dupe_iter(&ranges);
    }

    new_ranges
        .iter()
        .map(|r| r.len())
        .sum::<usize>()
        .to_string()
}

#[test]
fn testp1() {}

pub fn part_2(input: String) -> String {
    let mut sensors = vec![];

    for line in input.lines().map(|l| l.split(' ').collect::<Vec<&str>>()) {
        let x1 = line[2][2..line[2].len() - 1].parse::<isize>().unwrap();
        let y1 = line[3][2..line[3].len() - 1].parse::<isize>().unwrap();
        let x2 = line[8][2..line[8].len() - 1].parse::<isize>().unwrap();
        let y2 = line[9][2..line[9].len()].parse::<isize>().unwrap();

        let dist = (x2 - x1).abs() + (y2 - y1).abs();

        sensors.push((x1, y1, dist));
    }

    sensors.sort_by(|a, b| a.2.cmp(&b.2));
    sensors.reverse();

    for x1 in 0..=4000000 {
        let mut y1 = -1;
        while y1 <= 4000000 {
            y1 += 1;

            let mut nop = false;
            for (x2, y2, dist) in &sensors {
                let dx = (x2 - x1).abs();
                let dy = (y2 - y1).abs();
                if dx + dy <= *dist {
                    y1 = *y2 + (dist - dx);
                    nop = true;
                    break;
                }
            }
            if !nop {
                return (4000000 * x1 + y1).to_string();
            }
        }
    }

    todo!()
}

#[test]
fn testp2() {}
