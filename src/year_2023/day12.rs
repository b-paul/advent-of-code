use std::collections::HashMap;

fn valid(cs: &[char], ns: &[usize]) -> bool {
    let mut i = 0;
    let mut probably_false = false;
    for g in cs.chunk_by(|a, b| a == b) {
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
            return false;
        }
        if g.len() < ns[i] {
            probably_false = true;
        }
        if g.len() > ns[i] {
            return false;
        }
        i += 1;
    }
    i == ns.len() && !probably_false
}

fn solve(
    cs: &mut [char],
    idx: usize,
    ns: &[usize],
    mut hs: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if !valid(cs, ns) {
        return 0;
    }
    if idx == cs.len() {
        return 1;
    }

    if idx > 0 && cs[idx - 1] == '#' {
        hs += 1;
    }

    if cs[idx] == '?' {
        if idx > 0 && cs[idx - 1] == '.' {
            if let Some(&ans) = memo.get(&(idx, hs)) {
                return ans;
            }
        }

        cs[idx] = '#';
        let a = solve(cs, idx + 1, ns, hs, memo);
        cs[idx] = '.';
        let b = solve(cs, idx + 1, ns, hs, memo);
        cs[idx] = '?';

        let a = a + b;
        if idx > 0 && cs[idx - 1] == '.' {
            memo.insert((idx, hs), a);
        }
        a
    } else {
        if let Some(&ans) = memo.get(&(idx, hs)) {
            return ans;
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
            let mut cs = ws.next().unwrap().chars().collect::<Vec<_>>();
            let ns = ws
                .next()
                .unwrap()
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let mut memo = HashMap::new();

            solve(&mut cs, 0, &ns, 0, &mut memo)
        })
        .sum::<usize>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|l| {
            let mut ws = l.split(' ');
            let mut cs = ws.next().unwrap().chars().collect::<Vec<_>>();

            cs.push('?');
            let n = cs.len();
            let mut cs = cs.into_iter().cycle().take(n * 5 - 1).collect::<Vec<_>>();
            let ns = ws
                .next()
                .unwrap()
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let n = ns.len();
            let ns = ns.into_iter().cycle().take(n * 5).collect::<Vec<_>>();

            let mut memo = HashMap::new();

            solve(&mut cs, 0, &ns, 0, &mut memo)
        })
        .sum::<usize>()
}

#[cfg(test)]
mod benches {
    use crate::get_input;
    use crate::year_2023::day12::*;
    use test::{black_box, Bencher};

    #[bench]
    fn part1_normal(b: &mut Bencher) {
        let input = &get_input(2023, 12).unwrap();
        b.iter(|| {
            black_box(part_1(input));
        })
    }

    #[bench]
    fn part2_normal(b: &mut Bencher) {
        let input = &get_input(2023, 12).unwrap();
        b.iter(|| {
            black_box(part_2(input));
        })
    }
}
