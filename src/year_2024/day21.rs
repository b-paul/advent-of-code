use crate::helper::prelude::*;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::*;

const NUMPAD: &str = "789
456
123
 0A";
const DIRPAD: &str = " ^A
<v>";

/// The const to move from from to to on the kth pad, with all previous pads on 'A'.
fn cost<'a>(
    from: GridEntry<'a, char>,
    to: GridEntry<'a, char>,
    k: usize,
    dirpad: &'a Grid<char>,
    memo: &mut HashMap<(usize, GridEntry<'a, char>, GridEntry<'a, char>), u64>,
) -> u64 {
    // The 0th dirpad is controlled by us, so moving between buttons costs nothing.
    if k == 0 {
        return 0;
    }

    if let Some(&n) = memo.get(&(k, from, to)) {
        return n;
    }

    let base = Point { x: 2, y: 0 };
    let mut ans = 0;

    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();
    pq.push((Reverse(0), from, base));
    while let Some((Reverse(cost1), cur, prev)) = pq.pop() {
        if visited.contains(&(cur, prev)) {
            continue;
        }
        visited.insert((cur, prev));

        if cur == to && prev == base {
            ans = cost1;
            break;
        }

        // Try move the previous cursor to all of the points it can
        for (target, &c) in dirpad.iter_idx().filter(|(_, &c)| c != ' ') {
            if let Some(dir) = read_dir(c) {
                if let Some(next) = cur.move_dir(dir) {
                    if next.pos().dist(to.pos()) > cur.pos().dist(to.pos()) {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            let cost2 = cost(
                dirpad.point(prev).unwrap(),
                dirpad.point(target).unwrap(),
                k - 1,
                dirpad,
                memo,
            );
            if !visited.contains(&(cur, target)) {
                pq.push((Reverse(cost1 + cost2), cur, target));
            }
        }

        // Press the previous cursor's button if it makes sense to
        if let Some(dir) = read_dir(dirpad[prev]) {
            if let Some(cur) = cur.move_dir(dir) {
                if *cur.val() != ' ' && !visited.contains(&(cur, prev)) {
                    pq.push((Reverse(cost1 + 1), cur, prev));
                }
            }
        }
    }

    memo.insert((k, from, to), ans);
    ans
}

fn solve(input: &str, pads: usize) -> u64 {
    let numpad = NUMPAD.parse::<Grid<char>>().unwrap();
    let dirpad = DIRPAD.parse::<Grid<char>>().unwrap();

    let mut memo = HashMap::new();

    input
        .lines()
        .map(|l| {
            let mut numpad_pos = numpad.point(Point { x: 2, y: 3 }).unwrap();
            let mut len = 0;

            for c in l.chars() {
                let target = numpad.point(numpad.find(&c).unwrap()).unwrap();
                len += cost(numpad_pos, target, pads + 1, &dirpad, &mut memo) + 1;
                numpad_pos = target;
            }

            let numpart = p::<u64>(&l[0..=2]);

            len * numpart
        })
        .sum::<u64>()
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    solve(input, 2)
}

#[test]
fn test() {
    let input = "029A
980A
179A
456A
379A";
    let output = 126384;
    assert_eq!(part_1(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    solve(input, 25)
}
