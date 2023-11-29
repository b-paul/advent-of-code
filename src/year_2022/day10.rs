use std::collections::HashSet;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut x = 1;
    let mut cycle = 0;
    let mut total = 0;

    for line in input.lines() {
        let stuff: Vec<&str> = line.split(' ').collect();
        if stuff[0] == "addx" {
            for _ in 0..2 {
                cycle += 1;
                if cycle % 40 == 20 {
                    total += x * cycle;
                }
            }

            x += stuff[1].parse::<i32>().unwrap();
        } else {
            cycle += 1;
            if cycle % 40 == 20 {
                total += x * cycle;
            }
        }
    }

    total.to_string()
}

#[test]
fn testp1() {
    assert_eq!(part_1("addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop".to_string()), 13140.to_string());
}

fn draw_pixel(x: i32, cycle: i32) {
    let xpos = (cycle+39) % 40;
    //let ypos = cycle/40;

    if (xpos - x).abs() <= 1 {
        print!("#");
    } else {
        print!(".");
    }
    if xpos == 39 {
        println!();
    }
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut x = 1;
    let mut cycle = 0;

    for line in input.lines() {
        let stuff: Vec<&str> = line.split(' ').collect();
        if stuff[0] == "addx" {
            for _ in 0..2 {
                cycle += 1;
                draw_pixel(x, cycle);
            }

            x += stuff[1].parse::<i32>().unwrap();
        } else {
            cycle += 1;
            draw_pixel(x, cycle);
        }
    }

    "Read the ascii art above ^".to_string()
}
