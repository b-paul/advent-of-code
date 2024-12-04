use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

fn intersects(b1: &[Vec<isize>], b2: &[Vec<isize>]) -> bool {
    let x1a = b1[0][0];
    let x2a = b1[1][0];
    let xaa = x1a.min(x2a);
    let xba = x1a.max(x2a);
    let xra = xaa..=xba;
    let y1a = b1[0][1];
    let y2a = b1[1][1];
    let yaa = y1a.min(y2a);
    let yba = y1a.max(y2a);
    let yra = yaa..=yba;
    let x1 = b2[0][0];
    let x2 = b2[1][0];
    let xa = x1.min(x2);
    let xb = x1.max(x2);
    let xr = xa..=xb;
    let y1 = b2[0][1];
    let y2 = b2[1][1];
    let ya = y1.min(y2);
    let yb = y1.max(y2);
    let yr = ya..=yb;

    (xra.contains(&xa) || xra.contains(&xb) || xr.contains(&xaa) || xr.contains(&xba))
        && (yra.contains(&ya) || yra.contains(&yb) || yr.contains(&yaa) || yr.contains(&yba))
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut bricks = input
        .lines()
        .map(|l| {
            l.split('~')
                .map(|s| s.split(',').map(p::<isize>).collect_vec())
                .collect_vec()
        })
        .collect_vec();

    bricks.sort_by_key(|brick| (brick[0][2].max(brick[1][2]), brick[0][2].min(brick[1][2])));

    for i in 0..bricks.len() {
        let mut moved = false;
        for j in (0..i).rev() {
            if intersects(&bricks[i], &bricks[j]) {
                let z1 = bricks[j][0][2];
                let z2 = bricks[j][1][2];
                let zm = z1.max(z2);
                let (l, r) = if bricks[i][0][2] < bricks[i][1][2] {
                    (0, 1)
                } else {
                    (1, 0)
                };
                let dz = bricks[i][l][2] - zm - 1;
                assert!(dz >= 0);
                if moved && dz != 0 {
                    break;
                }
                bricks[i][l][2] -= dz;
                bricks[i][r][2] -= dz;
                moved = true;
            }
        }
        if !moved {
            let (l, r) = if bricks[i][0][2] < bricks[i][1][2] {
                (0, 1)
            } else {
                (1, 0)
            };
            let dz = bricks[i][l][2] - 1;
            bricks[i][l][2] -= dz;
            bricks[i][r][2] -= dz;
        }
        bricks.sort_by_key(|brick| (brick[0][2].max(brick[1][2]), brick[0][2].min(brick[1][2])));
    }

    //println!("{bricks:?}");
    // for giving to scarpet, a minecraft scripting language :exploding_head:
    print!("[");
    let mut i = 0;
    for (j, brick) in bricks.iter().enumerate() {
        let xl = brick[0][0].min(brick[1][0]);
        let xr = brick[0][0].max(brick[1][0]);
        let yl = brick[0][1].min(brick[1][1]);
        let yr = brick[0][1].max(brick[1][1]);
        let zl = brick[0][2].min(brick[1][2]);
        let zr = brick[0][2].max(brick[1][2]);
        for v in [xl..=xr, yl..=yr, zl..=zr]
            .into_iter()
            .multi_cartesian_product()
        {
            let (x, y, z) = (v[0], v[1], v[2]);
            print!("{x}, {z}, {y}, {j}, ");
            i += 1;
        }
    }
    println!("];");
    println!("{i}");

    let mut supports = bricks
        .clone()
        .iter_mut()
        .enumerate()
        .map(|(i, _)| (i, BTreeSet::new()))
        .collect::<HashMap<_, _>>();
    let mut supported = bricks
        .clone()
        .iter_mut()
        .enumerate()
        .map(|(i, _)| (i, BTreeSet::new()))
        .collect::<HashMap<_, _>>();

    for (i, b) in bricks.iter().enumerate().rev() {
        let z1a = b[0][2];
        let z2a = b[1][2];
        let zma = z1a.min(z2a);
        for (j, b2) in bricks[0..i].iter().enumerate().rev() {
            let z1 = b2[0][2];
            let z2 = b2[1][2];
            let zm = z1.max(z2);
            if intersects(b, b2) && zma - zm == 1 {
                supports.get_mut(&j).unwrap().insert(i);
                supported.get_mut(&i).unwrap().insert(j);
            }
        }
    }

    println!("{supports:?}");
    println!("{supported:?}");

    let ok = supports
        .into_iter()
        .filter(|(_, vec)| vec.iter().all(|j| supported[j].len() > 1))
        .collect_vec();
    //println!("{ok:?}");

    ok.len()
}

#[test]
fn test() {
    let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    let output = 7;
    assert_eq!(part_2(input).to_string(), output.to_string());
    assert!(false);
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut bricks = input
        .lines()
        .map(|l| {
            l.split('~')
                .map(|s| s.split(',').map(p::<isize>).collect_vec())
                .collect_vec()
        })
        .collect_vec();

    bricks.sort_by_key(|brick| (brick[0][2].max(brick[1][2]), brick[0][2].min(brick[1][2])));

    for i in 0..bricks.len() {
        let mut moved = false;
        for j in (0..i).rev() {
            if intersects(&bricks[i], &bricks[j]) {
                let z1 = bricks[j][0][2];
                let z2 = bricks[j][1][2];
                let zm = z1.max(z2);
                let (l, r) = if bricks[i][0][2] < bricks[i][1][2] {
                    (0, 1)
                } else {
                    (1, 0)
                };
                let dz = bricks[i][l][2] - zm - 1;
                assert!(dz >= 0);
                if moved && dz != 0 {
                    break;
                }
                bricks[i][l][2] -= dz;
                bricks[i][r][2] -= dz;
                moved = true;
            }
        }
        if !moved {
            let (l, r) = if bricks[i][0][2] < bricks[i][1][2] {
                (0, 1)
            } else {
                (1, 0)
            };
            let dz = bricks[i][l][2] - 1;
            bricks[i][l][2] -= dz;
            bricks[i][r][2] -= dz;
        }
        bricks.sort_by_key(|brick| (brick[0][2].max(brick[1][2]), brick[0][2].min(brick[1][2])));
    }

    let mut supports = bricks
        .clone()
        .iter_mut()
        .enumerate()
        .map(|(i, _)| (i, BTreeSet::new()))
        .collect::<HashMap<_, _>>();
    let mut supported = bricks
        .clone()
        .iter_mut()
        .enumerate()
        .map(|(i, _)| (i, BTreeSet::new()))
        .collect::<HashMap<_, _>>();

    for (i, b) in bricks.iter().enumerate().rev() {
        let z1a = b[0][2];
        let z2a = b[1][2];
        let zma = z1a.min(z2a);
        for (j, b2) in bricks[0..i].iter().enumerate().rev() {
            let z1 = b2[0][2];
            let z2 = b2[1][2];
            let zm = z1.max(z2);
            if intersects(b, b2) && zma - zm == 1 {
                supports.get_mut(&j).unwrap().insert(i);
                supported.get_mut(&i).unwrap().insert(j);
            }
        }
    }

    println!("{supports:?}");
    println!("{supported:?}");

    supports
        .iter()
        .filter(|(_, vec)| !vec.iter().all(|j| supported[j].len() > 1))
        .map(|(j, vec)| {
            let mut res = 0;
            let mut queue = vec.iter()
                .map(|i| (-bricks[*i][0][2].max(bricks[*i][1][2]), i))
                .collect::<BinaryHeap<_>>();
            let mut visited = HashSet::new();

            let mut removed = Vec::new();

            removed.push(j);

            while let Some((_, i)) = queue.pop() {
                if visited.contains(&i) {
                    continue;
                }
                visited.insert(i);
                let s = supported[i]
                    .iter()
                    .filter(|i| !removed.contains(i))
                    .collect::<VecDeque<_>>();
                println!("{i} {s:?} {removed:?}");
                if s.is_empty() {
                    removed.push(i);
                    res += 1;
                    let mut s = supports[i]
                        .iter()
                        .filter(|i| !removed.contains(i))
                        .map(|i| (-bricks[*i][0][2].max(bricks[*i][1][2]), i))
                        .collect::<BinaryHeap<_>>();
                    queue.append(&mut s);
                }
            }
            println!("{j}: {removed:?}");

            res
        })
        .sum::<usize>()
}
