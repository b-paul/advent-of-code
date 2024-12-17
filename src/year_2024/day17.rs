use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

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
        let combo = match arg {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!(),
        };
        //println!("{ip} {op} {arg} {combo} {a} {b} {c}");
        match op {
            0 => a = a >> combo,
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

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Expr {
    A,
    Lit(u64),
    // a / 2^the expr
    Shr(Box<Expr>, Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>)
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut input = input.split("\n\n");
    let (_, b, c) = {
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

    /*
    let a = Expr::A;
    let b = Expr::Lit(b);
    let c = Expr::Lit(c);

    let mut stack = Vec::new();
    stack.push((a, b, c, 0, 0));

    while let Some((mut a, mut b, mut c, mut ip, outi)) = stack.pop() {
        loop {
            if ip >= program.len() - 1 {
                break;
            }
            let (op, arg) = (program[ip], program[ip + 1]);
            let combo = Box::new(match arg {
                0 => Expr::Lit(0),
                1 => Expr::Lit(1),
                2 => Expr::Lit(2),
                3 => Expr::Lit(3),
                4 => a.clone(),
                5 => b.clone(),
                6 => c.clone(),
                _ => panic!(),
            });
            //println!("{ip} {op} {arg} {combo} {a} {b} {c}");
            match op {
                0 => a = Expr::Shr(Box::new(a), combo),
                1 => b = Expr::Lit(arg),
                2 => b = Expr::Mod(combo),
                3 => {
                    stack.push((a.clone(), b.clone(), c.clone(), ip, outi));
                    break;
                }
                4 => b = Expr::Xor(Box::new(b), Box::new(c.clone())),
                5 => {
                    /*
                    if program[outi] != combo % 8 {
                        continue 'outer;
                    }
                    outi += 1;
                    out.push(combo % 8);
                    */
                },
                6 => b = Expr::Shr(Box::new(a.clone()), combo),
                7 => c = Expr::Shr(Box::new(a.clone()), combo),
                _ => panic!(),
            }
            ip = ip.wrapping_add(2);
        }
    }
    */

    /*
    'outer: for i in 0.. {
        let mut a = i;
        let mut b = b;
        let mut c = c;
        let mut outi = 0;
        let mut out = Vec::new();
        let mut ip = 0;

        if i % 1024 == 0 {
            println!("{i}");
        }

        loop {
            if ip >= program.len() - 1 {
                break;
            }
            let (op, arg) = (program[ip], program[ip + 1]);
            let combo = match arg {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => a,
                5 => b,
                6 => c,
                _ => panic!(),
            };
            //println!("{ip} {op} {arg} {combo} {a} {b} {c}");
            match op {
                0 => a = a >> combo,
                1 => b ^= arg,
                2 => b = combo & 7,
                3 => {
                    if a != 0 {
                        ip = (arg as usize).wrapping_sub(2);
                    }
                }
                4 => b ^= c,
                5 => {
                    if program[outi] != combo % 8 {
                        continue 'outer;
                    }
                    outi += 1;
                    out.push(combo % 8);
                },
                6 => b = a >> combo,
                7 => c = a >> combo,
                _ => panic!(),
            }
            ip = ip.wrapping_add(2);
        }

        if program == out {
            return i;
        }
    }
    */

    /*
     * while a != 0 {
     *     b = (a & 7) ^ 1;
     *     c = a >> b;
     *     a >>= 3;
     *     print b ^ 4 ^ c
     * }
     *
     * end:
     * a' = 000
     * a  = aaa
     * b  = bb0
     * c  = ccc
     * ^  = 000
     */

    let mut q = BinaryHeap::new();

    q.push((15, 0i64));

    while let Some((i, a)) = q.pop() {
        let a = -a;
        for a2 in 0..8 {
            let a = a << 3 | a2;
            let b = (a & 7) ^ 1;
            let c = a >> b;
            if (4 ^ b ^ c) & 7 == program[i as usize] as i64 {
                if i == 0 {
                    return a;
                }
                q.push((i - 1, -a));
            }
        }
    }

    0
}
