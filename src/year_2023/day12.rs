use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

fn valid(cs: &[char], ns: &[usize]) -> bool {
    let mut i = 0;
    let mut probably_false = false;
    for g in cs.group_by(|a, b| a == b) {
        if g[0] == '.' {
            if probably_false {
                return false;
            }
            continue;
        }
        probably_false = false;
        if g[0] == '?' {
            return true;
        }
        if i >= ns.len() {
            //println!("a");
            return false;
        }
        if g.len() < ns[i] {
            //println!("b");
            probably_false = true;
        }
        if g.len() > ns[i] {
            return false;
        }
        i += 1;
    }
    i == ns.len() && !probably_false
}

fn can_be_valid(cs: &mut [char], idx: usize, ns: &[usize]) -> bool {
    if !valid(cs, ns) {
        return false;
    }
    if idx == cs.len() {
        return true;
    }
    if cs[idx] == '?' {
        cs[idx] = '#';
        let a = can_be_valid(cs, idx+1, ns);
        let b = if a {
            true
        } else {
            cs[idx] = '.';
            can_be_valid(cs, idx+1, ns)
        };
        cs[idx] = '?';
        a || b
    } else {
        can_be_valid(cs, idx + 1, ns)
    }
}

fn solve(cs: &mut [char], idx: usize, ns: &[usize], mut hs: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    //let qs = cs.iter().collect::<String>();
    //println!("{qs} {idx}");
    if !valid(cs, ns) {
        return 0;
    }
    if idx == cs.len() {
        return 1;
    }

    if idx > 0 && cs[idx-1] == '#' {
        hs += 1;
    }

    if cs[idx] == '?' {
        if idx > 0 && cs[idx-1] == '.' {
            if let Some(&ans) = memo.get(&(idx, hs)) {
                return ans;
            }
        }

        cs[idx] = '#';
        let a = solve(cs, idx + 1, ns, hs, memo);
        cs[idx] = '.';
        let b =  solve(cs, idx + 1, ns, hs, memo);
        cs[idx] = '?';

        let a = a + b;
        if idx > 0 && cs[idx-1] == '.' {
            memo.insert((idx, hs), a);
        }
        a
    } else {
        if let Some(&ans) = memo.get(&(idx, hs)) {
        //if can_be_valid(cs, idx, ns) {
            return ans;
        //} else {
            //let qs = cs.iter().collect::<String>();
            //println!("{qs} {idx}");
            //return 0;
        //}
        }
        let a = solve(cs, idx + 1, ns, hs, memo);
        memo.insert((idx, hs), a);
        a
    }
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|l| {
            let mut ws = l.split(' ');
            let mut cs = ws.next().unwrap().chars().collect_vec();
            let ns = ws
                .next()
                .unwrap()
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec();

            let qs = cs.iter().collect::<String>();
            println!("{qs}");
            println!("{ns:?}");

            let mut memo = HashMap::new();

            solve(&mut cs, 0, &ns, 0, &mut memo)
        })
        .sum::<usize>()
}

#[test]
fn test() {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    //let input = ".??..??...?##. 1,1,3";
    let output = 525152;
    let a = ".#........?##.";
    assert!(!can_be_valid(&mut a.chars().collect_vec(), 0, &[1, 1, 3]));
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|l| {
            let mut ws = l.split(' ');
            let mut cs = ws.next().unwrap().chars().collect_vec();

            cs.push('?');
            let n = cs.len();
            let mut cs = cs.into_iter().cycle().take(n * 5 - 1).collect_vec();
            let ns = ws
                .next()
                .unwrap()
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec();
            let n = ns.len();
            let ns = ns.into_iter().cycle().take(n * 5).collect_vec();

            let mut memo = HashMap::new();

            let a = solve(&mut cs, 0, &ns, 0, &mut memo);

            //println!("{i} {a}");

            a
        })
        .sum::<usize>()
}
