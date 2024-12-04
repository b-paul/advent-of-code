use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut modules: HashMap<String, (i64, bool, Vec<&str>)> = HashMap::new();
    let mut recvs: HashMap<String, HashMap<String, bool>> = HashMap::new();
    for l in input.lines() {
        let mut s = l.split(" -> ");

        let n = s.next().unwrap();
        let vals = s.next().unwrap().split(", ").collect_vec();
        if n == "broadcaster" {
            let entry = modules.entry("broadcaster".to_string()).or_default();
            entry.0 = 1;
            entry.2 = vals.clone();
            let name = "broadcaster";
            for val in vals {
                *recvs.entry(val.to_string()).or_default().entry(name.to_string()).or_default() = false;
            }
        } else {
            let name = n[1..].to_string();
            match n.chars().next().unwrap() {
                '%' => {
                    let entry = modules.entry(name.clone()).or_default();
                    entry.0 = 2;
                    entry.2 = vals.clone();
                }
                '&' => {
                    let entry = modules.entry(name.clone()).or_default();
                    entry.0 = 3;
                    entry.2 = vals.clone();
                }
                a => panic!("{a}"),
            }
            for val in vals {
                *recvs.entry(val.to_string()).or_default().entry(name.to_string()).or_default() = false;
            }
        }
    }

    println!("{modules:?}");

    let mut lows = 0i64;
    let mut highs = 0i64;

    for _ in 0..1000 {
        let mut queue = VecDeque::new();

        // send low to broadcast
        queue.push_back((false, "broadcaster".to_string()));

        while let Some((pulse, name)) = queue.pop_front() {
            if pulse {
                highs += 1;
            } else {
                lows += 1;
            }

            let Some((typ, level, outs)) = modules.get_mut(&name) else { continue; };

            match typ {
                0 => {}
                1 => {
                    for val in outs {
                        queue.push_back((pulse, val.to_string()));
                        *recvs.entry(val.to_string()).or_default().entry(name.clone()).or_default() = pulse;
                    }
                }
                2 => {
                    if !pulse {
                        *level = !*level;
                        for val in outs {
                            queue.push_back((*level, val.to_string()));
                            *recvs.entry(val.to_string()).or_default().entry(name.clone()).or_default() = *level;
                        }
                    }
                }
                3 => {
                    *level = true;
                    for val in recvs.entry(name.clone()).or_default().values() {
                        *level &= val;
                    }
                    for val in outs {
                        queue.push_back((!*level, val.to_string()));
                        *recvs.entry(val.to_string()).or_default().entry(name.clone()).or_default() = !*level;
                    }
                }
                _ => panic!(),
            }
        }
    }

    println!("{lows} {highs}");

    lows * highs
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut modules: HashMap<String, (i64, bool, Vec<&str>)> = HashMap::new();
    let mut recvs: HashMap<String, HashMap<String, bool>> = HashMap::new();
    for l in input.lines() {
        let mut s = l.split(" -> ");

        let n = s.next().unwrap();
        let vals = s.next().unwrap().split(", ").collect_vec();
        if n == "broadcaster" {
            let entry = modules.entry("broadcaster".to_string()).or_default();
            entry.0 = 1;
            entry.2 = vals.clone();
            let name = "broadcaster";
            for val in vals {
                *recvs.entry(val.to_string()).or_default().entry(name.to_string()).or_default() = false;
            }
        } else {
            let name = n[1..].to_string();
            match n.chars().next().unwrap() {
                '%' => {
                    let entry = modules.entry(name.clone()).or_default();
                    entry.0 = 2;
                    entry.2 = vals.clone();
                }
                '&' => {
                    let entry = modules.entry(name.clone()).or_default();
                    entry.0 = 3;
                    entry.2 = vals.clone();
                }
                a => panic!("{a}"),
            }
            for val in vals {
                *recvs.entry(val.to_string()).or_default().entry(name.to_string()).or_default() = false;
            }
        }
    }

    println!("{modules:?}");

    let mut lows = 0i64;
    let mut highs = 0i64;

    for i in 1.. {
        let mut queue = VecDeque::new();

        // send low to broadcast
        queue.push_back((false, "broadcaster".to_string()));

        while let Some((pulse, name)) = queue.pop_front() {
            if pulse {
                highs += 1;
            } else {
                lows += 1;
            }

            if name == *"rx" && !pulse {
                return i;
            }

            let Some((typ, level, outs)) = modules.get_mut(&name) else { continue; };

            match typ {
                0 => {}
                1 => {
                    for val in outs {
                        queue.push_back((pulse, val.to_string()));
                        *recvs.entry(val.to_string()).or_default().entry(name.clone()).or_default() = pulse;
                    }
                }
                2 => {
                    if !pulse {
                        *level = !*level;
                        for val in outs {
                            if *val == "mf" && *level {
                                println!("{i} {name}");
                            }
                            queue.push_back((*level, val.to_string()));
                            *recvs.entry(val.to_string()).or_default().entry(name.clone()).or_default() = *level;
                        }
                    }
                }
                3 => {
                    *level = true;
                    for val in recvs.entry(name.clone()).or_default().values() {
                        *level &= val;
                    }
                    for val in outs {
                        if *val == "mf" && !*level {
                            println!("{i} {name}");
                        }
                        queue.push_back((!*level, val.to_string()));
                        *recvs.entry(val.to_string()).or_default().entry(name.clone()).or_default() = !*level;
                    }
                }
                _ => panic!(),
            }
        }
    }

    println!("{lows} {highs}");

    lows * highs
}
