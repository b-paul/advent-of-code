use crate::helper::prelude::*;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::*;

fn get_combo(arg: u64, a: u64, b: u64, c: u64) -> u64 {
    match arg {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!(),
    }
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut input = input.split("\n\n");
    let (mut a, mut b, mut c) = {
        let nums = input
            .next()
            .unwrap()
            .lines()
            .map(|l| p::<u64>(&l[12..]))
            .collect_vec();
        (nums[0], nums[1], nums[2])
    };
    let program = input.next().unwrap().lines().next().unwrap()[9..]
        .split(',')
        .map(p::<u64>)
        .collect_vec();

    let mut out = Vec::new();
    let mut ip = 0;

    loop {
        if ip >= program.len() - 1 {
            break;
        }
        let (op, arg) = (program[ip], program[ip + 1]);
        let combo = get_combo(arg, a, b, c);
        match op {
            0 => a >>= combo,
            1 => b ^= arg,
            2 => b = combo % 8,
            3 => {
                if a != 0 {
                    ip = (arg as usize).wrapping_sub(2);
                }
            }
            4 => b ^= c,
            5 => out.push(combo % 8),
            6 => b = a >> combo,
            7 => c = a >> combo,
            _ => panic!(),
        }
        ip = ip.wrapping_add(2);
    }

    out.iter()
        .skip(1)
        .fold(out[0].to_string(), |s, n| s + "," + &n.to_string())
}

#[test]
fn test() {
    let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
    let output = 117440;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    // this looks so funny lol
    let output = input.split("\n\n").nth(1).unwrap().lines().next().unwrap()[9..]
        .split(',')
        .map(p::<u64>)
        .collect_vec();
    let program = output.iter().copied().array_chunks::<2>().collect_vec();

    /* This solution makes many more assumptions on top of the ones given in the problem statement.
     * I will assume that the program looks something like this:
     *
     * while a != 0 {
     *     (b, c) = f(a); // Sequence of numerical operations (importantly, not out or jnz)
     *     a >>= 3;
     *     (b, c) = g(a);
     *     out ?; // Output one of the registers
     * }
     *
     * The algorithm will go as follows:
     * We will initialise a to 0, and work backwards, figuring out all possible values of a that
     * could have lead to its current one. To do this, we iterate over all 3-bit integers, shift a
     * to the left by 3, and or these 3-bit integers in. From here, we execute the instructions in
     * order, and check whether it gives the correct output.
     */

    assert!(program.iter().filter(|[a, _]| *a == 0).count() == 1);
    assert!(program[program.len() - 2][0] == 5);
    let shift = *program
        .iter()
        .filter_map(|[a, b]| (*a == 0).then_some(b))
        .next()
        .unwrap();
    assert!(shift == 3);

    let mut q = BinaryHeap::new();

    q.push((output.len() - 1, Reverse(0)));

    while let Some((i, Reverse(a))) = q.pop() {
        'guess: for a2 in 0..8 {
            let old_a = a << 3 | a2;
            let mut a = old_a;
            let mut b = 0;
            let mut c = 0;
            for &[op, arg] in program.iter() {
                let combo = get_combo(arg, a, b, c);
                match op {
                    0 => a = a >> combo,
                    1 => b ^= arg,
                    2 => b = combo % 8,
                    3 => {} // break
                    4 => b ^= c,
                    5 => {
                        if output[i] != combo % 8 {
                            continue 'guess;
                        }
                    }
                    6 => b = a >> combo,
                    7 => c = a >> combo,
                    _ => panic!(),
                }
            }
            if i == 0 {
                return old_a;
            }
            q.push((i - 1, Reverse(old_a)));
        }
    }

    0
}

#[cfg(test)]
mod benches {
    use crate::get_input;
    use crate::year_2024::day17::*;
    use test::{black_box, Bencher};

    #[bench]
    fn part1_normal(b: &mut Bencher) {
        let input = &get_input(2024, 17).unwrap();
        b.iter(|| {
            black_box(part_1(input));
        })
    }

    #[bench]
    fn part2_normal(b: &mut Bencher) {
        let input = &get_input(2024, 17).unwrap();
        b.iter(|| {
            black_box(part_2(input));
        })
    }
}
