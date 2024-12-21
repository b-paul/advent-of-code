use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

const NUMPAD: &'static str = "789
456
123
 0A";
const DIRPAD: &'static str = " ^A
<v>";

fn press<'a>(
    poss: Vec<GridEntry<'a, char>>,
    numpad: GridEntry<'a, char>,
    c: char,
) -> Option<(Vec<GridEntry<'a, char>>, GridEntry<'a, char>)> {
    if poss.len() == 0 {
        if *numpad.val() == ' ' {
            return None;
        }
        // moving numpad
        match c {
            '<' | '^' | '>' | 'v' => {
                let dir = read_dir(c).unwrap();
                let numpad = numpad.move_dir(dir)?;
                Some((poss, numpad))
            }
            'A' => {
                //panic!("Pressed A on the solution ?")
                None
            }
            _ => None,
        }
    } else {
        let head = poss.last().unwrap();
        if *head.val() == ' ' {
            return None;
        }
        let mut tail = poss[..poss.len() - 1].to_vec();
        match c {
            '<' | '^' | '>' | 'v' => {
                let dir = read_dir(c).unwrap();
                let head = head.move_dir(dir)?;
                tail.push(head);
                Some((tail, numpad))
            }
            'A' => {
                let (mut tail, numpad) = press(tail, numpad, *head.val())?;
                tail.push(*head);
                Some((tail, numpad))
            }
            _ => None,
        }
    }
}

fn solve<const PADS: usize>(input: &str) -> Option<u64> {
    let numpad = NUMPAD.parse::<Grid<char>>().unwrap();
    let dirpad = DIRPAD.parse::<Grid<char>>().unwrap();

    // I control a dirpad, which controls a dirpad, which controls the numpad.

    let mut numpad_pos = numpad.point(Point { x: 2, y: 3 })?;
    let base_dirpos = dirpad.point(Point { x: 2, y: 0 })?;

    let mut soln = Vec::new();

    for c in input.chars() {
        // "BFS" from numpad_pos, finding the shortest sequence to press this digit.
        // We can separate per digit, since once the digit is pressed every cursor must point to
        // the A on dirpads
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((vec![base_dirpos; PADS], numpad_pos, Vec::new()));
        visited.insert((vec![base_dirpos; PADS], numpad_pos));
        while let Some((many, numpad, mut cur_soln)) = queue.pop_front() {
            if *numpad.val() == c && many.iter().all(|p| *p.val() == 'A') {
                numpad_pos = numpad;
                cur_soln.push('A');
                soln.append(&mut cur_soln);
                break;
            }

            for c in "<^>vA".chars() {
                let Some((many, numpad)) = press(many.clone(), numpad, c) else {
                    continue;
                };
                let mut soln = cur_soln.clone();
                soln.push(c);
                if !visited.contains(&(many.clone(), numpad)) {
                    queue.push_back((many.clone(), numpad, soln));
                    visited.insert((many, numpad));
                }
            }
        }
    }

    let numpart = p::<u64>(&input[0..=2]);
    println!("{:?} {} {}", soln, soln.len() as u64, numpart);

    Some(soln.len() as u64 * numpart)
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    input.lines().map(|l| solve::<2>(l).unwrap()).sum::<u64>()
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
    input.lines().map(|l| solve::<25>(l).unwrap()).sum::<u64>()
}
