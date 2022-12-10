use std::collections::HashSet;

fn mv_tail(head: (i32, i32), tail: (i32, i32)) -> bool {
    (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1
}

pub fn part_1(input: String) -> String {
    let mut head_pos = (0, 0);
    let mut old_head_pos;
    let mut tail_pos = (0, 0);

    let mut pos_map = HashSet::new();

    pos_map.insert(tail_pos);

    for line in input.lines() {
        let bytes = line.as_bytes();

        let n = line.split(' ').nth(1).unwrap().parse().unwrap();

        for _ in 0..n {
            old_head_pos = head_pos;
            match bytes[0] {
                b'U' => head_pos.1 += 1,
                b'D' => head_pos.1 -= 1,
                b'R' => head_pos.0 += 1,
                b'L' => head_pos.0 -= 1,

                _ => (),
            }
            if mv_tail(head_pos, tail_pos) {
                tail_pos = old_head_pos;
            }

            pos_map.insert(tail_pos);
        }
    }

    /*
    for y in -50..50 {
        for x in -50..50 {
            if pos_map.contains(&(x,-y)) {
                print!("x");
            } else {
                print!(".");
            }
        }
        println!();
    }
    */

    pos_map.len().to_string()
}

fn touching(k1: (i32, i32), k2: (i32, i32)) -> bool {
    (k1.0 == k2.0 && (k1.1-k2.1).abs()<=1) ||
    (k1.1 == k2.1 && (k1.0-k2.0).abs()<=1)
}

fn newpos(k1: (i32, i32), k2: (i32, i32)) -> (i32, i32) {
    if k1.1 == k2.1 {
        if k1.0 > k2.0 {
            return (k1.0 - 1, k1.1);
        } else {
            return (k1.0 + 1, k1.1);
        }
    }
    if k1.0 == k2.0 {
        if k1.1 > k2.1 {
            return (k1.0, k1.1 - 1);
        } else {
            return (k1.0, k1.1 + 1);
        }
    }
    for x in -1..=1 {
        for y in -1..=1 {
            let new = (k2.0 + x, k2.1 + y);
            if touching(k1, new) {
                return new
            }
        }
    }
    for x in -1..=1 {
        for y in -1..=1 {
            let new = (k2.0 + x, k2.1 + y);
            if !mv_tail(k1, new) {
                return new
            }
        }
    }
    unreachable!()
}

pub fn part_2(input: String) -> String {
    let mut knots_pos = [(0, 0); 10];
    let mut old_knots_pos = [(0, 0); 10];

    let mut pos_map = HashSet::new();

    let (mut miny, mut maxy, mut minx, mut maxx) = (0,0,0,0);

    pos_map.insert(knots_pos[9]);

    for line in input.lines() {
        let bytes = line.as_bytes();

        let n = line.split(' ').nth(1).unwrap().parse().unwrap();

        for _ in 0..n {
            old_knots_pos[0] = knots_pos[0];
            match bytes[0] {
                b'U' => knots_pos[0].1 += 1,
                b'D' => knots_pos[0].1 -= 1,
                b'R' => knots_pos[0].0 += 1,
                b'L' => knots_pos[0].0 -= 1,
                _ => (),
            }
            for i in 1..knots_pos.len() {
                old_knots_pos[i] = knots_pos[i];
                if mv_tail(knots_pos[i - 1], knots_pos[i]) {
                    knots_pos[i] = newpos(knots_pos[i-1], knots_pos[i]);

                    if knots_pos[i].0 > maxx {
                        maxx = knots_pos[i].0;
                    }
                    if knots_pos[i].0 < minx {
                        minx = knots_pos[i].0;
                    }
                    if knots_pos[i].1 > maxy {
                        maxy = knots_pos[i].0;
                    }
                    if knots_pos[i].1 < miny {
                        miny = knots_pos[i].0;
                    }
                }
            }
            pos_map.insert(knots_pos[9]);
        }
    }

    /* draw the thingo
    for y in -maxy..-miny {
        for x in minx..maxx {
            if pos_map.contains(&(x,-y)) {
                print!("x");
            } else {
                print!(".");
            }
        }
        println!();
    }
    */

    pos_map.len().to_string()
}
