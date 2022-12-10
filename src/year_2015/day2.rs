use std::cmp::min;

fn solve_p1(line: String) -> u64 {
    let line = line;
    // Goes LxWxH
    let mut parts = line.split('x');
    let length: u64 = parts.next().unwrap().parse().unwrap();
    let width: u64 = parts.next().unwrap().parse().unwrap();
    let height: u64 = parts.next().unwrap().parse().unwrap();

    let a1 = length * width;
    let a2 = width * height;
    let a3 = height * length;

    let min_area = min(min(a1, a2), a3);

    let sa = 2 * (a1 + a2 + a3);

    sa + min_area
}

#[test]
fn p1_tests() {
    assert_eq!(solve_p1("2x3x4".to_string()), 58);
    assert_eq!(solve_p1("1x1x10".to_string()), 43);
}

pub fn part_1(input: String) -> String {
    let mut total: u64 = 0;

    for line in input.lines() {
        total += solve_p1(line.to_string());
    }

    total.to_string()
}

fn solve_p2(line: String) -> u64 {
    let line = line;
    // Goes LxWxH
    let mut parts = line.split('x');
    let length: u64 = parts.next().unwrap().parse().unwrap();
    let width: u64 = parts.next().unwrap().parse().unwrap();
    let height: u64 = parts.next().unwrap().parse().unwrap();

    let p1 = 2 * (length + width);
    let p2 = 2 * (width + height);
    let p3 = 2 * (height + length);

    let min_perim = min(min(p1, p2), p3);

    let vol = length * width * height;

    vol + min_perim
}

#[test]
fn p2_tests() {
    assert_eq!(solve_p2("2x3x4".to_string()), 34);
    assert_eq!(solve_p2("1x1x10".to_string()), 14);
}

pub fn part_2(input: String) -> String {
    let mut total: u64 = 0;

    for line in input.lines() {
        total += solve_p2(line.to_string());
    }

    total.to_string()
}
