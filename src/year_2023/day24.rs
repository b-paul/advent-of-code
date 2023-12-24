use crate::helper::prelude::*;
use itertools::Itertools;
use nalgebra::{Matrix2, Vector2};
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    const MIN: f64 = 200000000000000.;
    const MAX: f64 = 400000000000000.;

    let hailstones = input
        .lines()
        .map(|l| {
            let mut s = l.split(" @ ");
            let pos = s
                .next()
                .unwrap()
                .split(", ")
                .map(|s| s.trim())
                .map(p::<f64>)
                .collect_vec();
            let vel = s
                .next()
                .unwrap()
                .split(", ")
                .map(|s| s.trim())
                .map(p::<f64>)
                .collect_vec();
            (pos, vel)
        })
        .collect_vec();

    let mut count = 0;

    for (i, (pa, va)) in hailstones.iter().enumerate() {
        for (j, (pb, vb)) in hailstones[i + 1..].iter().enumerate() {
            // x(t) = px + t vx
            // y(t) = py + t vy
            // pxa + t vxa = pxb + t vxb
            // t vxa - tb vxb = pxb - pxa
            // t vya - tb vyb = pyb - pya
            let (pxa, pya) = (pa[0], pa[1]);
            let (vxa, vya) = (va[0], va[1]);
            let (pxb, pyb) = (pb[0], pb[1]);
            let (vxb, vyb) = (vb[0], vb[1]);
            let m = Matrix2::new(vxa, -vxb, vya, -vyb);
            let Some(i) = m.try_inverse() else {
                continue;
            };
            let v = Vector2::new(pxb - pxa, pyb - pya);
            let t = i * v;
            //println!("{t}");
            let ta = t[0];
            let tb = t[1];
            if ta < 0. || tb < 0. {
                continue;
            }
            let x = pxa + ta * vxa;
            let y = pya + ta * vya;

            if (MIN..=MAX).contains(&x) && (MIN..=MAX).contains(&y) {
                count += 1;
            }
        }
    }

    count
}

#[test]
fn test() {
    let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
    let output = 47;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let hailstones = input
        .lines()
        .take(5)
        .map(|l| {
            let mut s = l.split(" @ ");
            let pos = s
                .next()
                .unwrap()
                .split(", ")
                .map(|s| s.trim())
                .map(p::<f64>)
                .collect_vec();
            let vel = s
                .next()
                .unwrap()
                .split(", ")
                .map(|s| s.trim())
                .map(p::<f64>)
                .collect_vec();
            (pos, vel)
        })
        .collect_vec();
    let equations = hailstones.len();

    println!("from sympy import *");

    for i in 0..equations {
        print!("t{i}, ");
    }
    print!("px, py, pz, vx, vy, vz = symbols('");
    for i in 0..equations {
        print!("t{i} ");
    }
    println!("px py pz vx vy vz')");
    println!("soln = solve([");

    for (i, (pa, va)) in hailstones.iter().enumerate() {
        let (pxa, pya, pza) = (pa[0], pa[1], pa[2]);
        let (vxa, vya, vza) = (va[0], va[1], va[2]);
        println!("px - ({pxa} + t{i} * ({vxa} - vx)),");
        println!("py - ({pya} + t{i} * ({vya} - vy)),");
        println!("pz - ({pza} + t{i} * ({vza} - vz)),");
    }
    println!("], [");
    for i in 0..equations {
        println!("t{i},");
    }
    println!("px, py, pz, vx, vy, vz], dict=True)");

    // want to find tv such that this has a solution
    //
    // 900 equations
    // 300 time variables
    // 6 others
    // ????
    //
    // 15 equations
    // 10 time values
    // 6 others
    //
    // tp = pa + ta (va - tv)
    // tp = pb + tb (vb - tv)
    // tp = pc + tc (vc - tv)
    //
    // M (t1) =
    //   (t2)
    //   (t3)
    //
    // tp + ta tv = pa + ta va
    // tp + tb tv = pb + tb vb
    //
    // tp = tab va - taa tv - pa
    // tab va - taa tv - pa = tbb vb - tba tv - pb
    // tab va - tbb vb + (tba - taa) tv = pa - pb // 44850 of these equations
    // tp = tbb vb - tba tv - pb
    // tp = tcb vc - tca tv - pc
    // tp = tdb vd - tda tv - pd
    // tp = teb ve - tea tv - pe
    //
    // pa = tab va + tab 0
    //
    // INTEGER SOLUTION!!
    // AAAAHUMIHUIQGQASTN
    //
    // tpx - pax = tab vax - taa tvx
    // tpy - pay = tab vay - taa tvy
    // tpz - paz = tab vaz - taa tvz
    //
    // tpx - pbx = tbb vbx - tba tvx
    // tpy - pby = tbb vby - tba tvy
    // tpz - pbz = tbb vbz - tba tvz
    //
    // taa tvx - tab vax = pax - tpx
    // taa tvy - tab vay = pay - tpy
    // taa tvz - tab vaz = paz - tpz
    // ( tvx -vax tpx )  ( taa ) = ( pax )
    // ( tvy -vay tpy )  ( tab )   ( pay )
    // ( tvz -vaz tpz )  ( 1   )   ( paz )
    // i.e. that matrix is invertible
    // i.e. the determinant is zero
    // ?!?!
    // -vax (tvy tpz - tvz tpy) + vay (blah) - vaz (blah) != 0

    "#put thi into sympy lol"
}
