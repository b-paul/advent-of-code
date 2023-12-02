pub fn part_1(input: &str) -> impl std::fmt::Display {
    input.lines().enumerate().filter_map(|(i, l)| {
        let ws: Vec<_> = l.split(' ').collect();
        
        for i in 1.. {
            if ws.len() < 2*i + 1 {
                break;
            }
            let n = ws[2*i].parse::<u32>().unwrap();
            let c = ws[2*i+1].chars().next().unwrap();
            if c == 'b' && n > 14 { return None; }
            if c == 'g' && n > 13 { return None; }
            if c == 'r' && n > 12 { return None; }
        }

        Some(i + 1)
    }).sum::<usize>()
}

pub(crate) fn part_1_faster(input: &str) -> impl std::fmt::Display {
    let mut id = 1;
    let mut sum = 0;
    let bytes = input.as_bytes();
    let mut i = 6;
    let mut bad = false;
    loop {
        if i >= bytes.len() || bytes[i] == b'\n' {
            if !bad {
                println!("{id}");
                sum += id;
            }
            id += 1;
            if id > 100 { break; }
            bad = false;
            i += 7;
            if id >= 10 {
                i += 1;
            }
            if id >= 100 {
                i += 1;
            }
        }
        if i >= bytes.len() { break; }
        i += 2;
        // 48 = b'0' but as a u32
        let n = if bytes[i+1] == b' ' {
            let n = bytes[i] as u32 - 48;
            i += 2;
            n
        } else {
            let n = (bytes[i] as u32 - 48) * 10 + bytes[i + 1] as u32 - 48;
            i += 3;
            n
        };
        match bytes[i] {
            b'b' => { if n > 14 { bad = true; } i += 4; },
            b'g' => { if n > 13 { bad = true; } i += 5; },
            b'r' => { if n > 12 { bad = true; } i += 3; },
            _ => panic!("input bad uh oh"),
        }
    }
    sum
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    input.lines().map(|l| {
        let ws: Vec<_> = l.split(' ').collect();

        let mut bs = 0;
        let mut gs = 0;
        let mut rs = 0;
        
        for i in 1.. {
            if ws.len() < 2*i + 1 {
                break;
            }
            let n = ws[2*i].parse::<u32>().unwrap();
            let c = ws[2*i+1].chars().next().unwrap();
            if c == 'b' { bs = bs.max(n) }
            if c == 'g' { gs = gs.max(n) }
            if c == 'r' { rs = rs.max(n) }
        }

        bs * gs * rs
    }).sum::<u32>()
}

pub(crate) fn part_2_faster(input: &str) -> impl std::fmt::Display {
    let mut id = 1;
    let mut sum = 0;
    let bytes = input.as_bytes();
    let mut i = 6;
    let mut blue = 1;
    let mut green = 1;
    let mut red = 1;
    loop {
        if i >= bytes.len() || bytes[i] == b'\n' {
            sum += blue * green * red;
            id += 1;
            if id > 100 { break; }
            blue = 1;
            green = 1;
            red = 1;
            i += 7;
            if id >= 10 {
                i += 1;
            }
            if id >= 100 {
                i += 1;
            }
        }
        if i >= bytes.len() { break; }
        i += 2;
        // 48 = b'0' but as a u32
        let n = if bytes[i+1] == b' ' {
            let n = bytes[i] as u32 - 48;
            i += 2;
            n
        } else {
            let n = (bytes[i] as u32 - 48) * 10 + bytes[i + 1] as u32 - 48;
            i += 3;
            n
        };
        match bytes[i] {
            b'b' => { blue = blue.max(n); i += 4; },
            b'g' => { green = green.max(n); i += 5; },
            b'r' => { red = red.max(n); i += 3; },
            _ => panic!("input bad uh oh"),
        }
    }
    sum
}

#[cfg(test)]
mod benches {
    use crate::get_input;
    use crate::year_2023::day2::*;
    use test::{Bencher, black_box};

    #[bench]
    fn part1_normal(b: &mut Bencher) {
        let input = &get_input(2023, 2).unwrap();
        b.iter(|| {
            black_box(part_1(input));
        })
    }

    #[bench]
    fn part1_faster(b: &mut Bencher) {
        let input = &get_input(2023, 2).unwrap();
        assert_eq!(part_1_faster(input).to_string(), part_1(input).to_string());
        b.iter(|| {
            black_box(part_1_faster(input));
        })
    }

    #[bench]
    fn part2_normal(b: &mut Bencher) {
        let input = &get_input(2023, 2).unwrap();
        b.iter(|| {
            black_box(part_2(input));
        })
    }

    #[bench]
    fn part2_faster(b: &mut Bencher) {
        let input = &get_input(2023, 2).unwrap();
        assert_eq!(part_2_faster(input).to_string(), part_2(input).to_string());
        b.iter(|| {
            black_box(part_2_faster(input));
        })
    }
}
