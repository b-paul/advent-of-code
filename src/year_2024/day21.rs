use crate::helper::prelude::*;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::*;

const NUMPAD: &'static str = "789
456
123
 0A";
const DIRPAD: &'static str = " ^A
<v>";

/// The const to move from from to to on the kth pad and all previous pads move to 'A'
fn cost<'a>(
    from: GridEntry<'a, char>,
    to: GridEntry<'a, char>,
    k: usize,
    dirpad: &'a Grid<char>,
    memo: &mut HashMap<(usize, GridEntry<'a, char>, GridEntry<'a, char>), u64>,
) -> u64 {
    // If there are no dirpads before us, this is the human (me) pressing the button.
    if k == 0 {
        return 0;
    }
    if let Some(&n) = memo.get(&(k, from, to)) {
        return n;
    }

    let curpad = from.grid();
    let base = dirpad.point(Point { x: 2, y: 0 }).unwrap();

    let mut ans = 0;
    // Invariant, 0..k dirpads are all sitting at 'A'
    // We want to press some buttons on the k-1th dirpad, to
    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();
    pq.push((Reverse(0), from.pos(), base.pos()));
    while let Some((Reverse(cost1), cur, prev)) = pq.pop() {
        if visited.contains(&(cur, prev)) {
            continue;
        }
        visited.insert((cur, prev));

        let cur = curpad.point(cur).unwrap();

        if cur == to && prev == base.pos() {
            ans = cost1;
            break;
        }

        // Try move the previous cursor to all of the points it can
        for c in "<^>vA".chars() {
            // TODO don't do this every step
            let target = dirpad.find(&c).unwrap();
            // Calc however much it consts to move the previous pad to this button.
            let cost2 = cost(
                dirpad.point(prev).unwrap(),
                dirpad.point(target).unwrap(),
                k - 1,
                dirpad,
                memo,
            );
            pq.push((Reverse(cost1 + cost2), cur.pos(), target));
        }
        // Press the previous cursor's button if it makes sense to
        if let Some(dir) = read_dir(dirpad[prev]) {
            if let Some(cur) = cur.move_dir(dir) {
                if *cur.val() != ' ' {
                    pq.push((Reverse(cost1 + 1), cur.pos(), prev));
                }
            }
        }
    }

    assert!(from.pos() == to.pos() || ans != 0);

    memo.insert((k, from, to), ans);
    ans
}

fn solve(input: &str, pads: usize) -> Option<u64> {
    let numpad = NUMPAD.parse::<Grid<char>>().unwrap();
    let dirpad = DIRPAD.parse::<Grid<char>>().unwrap();

    // I control a dirpad, which controls a dirpad, which controls the numpad.

    let mut numpad_pos = numpad.point(Point { x: 2, y: 3 })?;
    //let base_dirpos = dirpad.point(Point { x: 2, y: 0 })?;

    let mut soln = 0;
    let mut memo = HashMap::new();

    for c in input.chars() {
        let target = numpad.point(numpad.find(&c).unwrap()).unwrap();
        soln += cost(numpad_pos, target, pads + 1, &dirpad, &mut memo);
        soln += 1;
        numpad_pos = target;
    }

    let numpart = p::<u64>(&input[0..=2]);

    Some(soln as u64 * numpart)
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input.lines().map(|l| solve(l, 2).unwrap()).sum::<u64>()
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
    input.lines().map(|l| solve(l, 25).unwrap()).sum::<u64>()
}
