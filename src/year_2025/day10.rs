use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    // bit manipulation brute force!! woooo
    // this puzzle looks very similar to the lights out puzzle and should be solvable with Z_2
    // linear algebra but... the inputs are small enough to brtue force!
    input
        .lines()
        .map(|s| {
            let (solved, s) = s[1..].split_once(']').unwrap();
            let (ms, _) = s[1..].split_once('{').unwrap();

            // b'#' is 35 in ascii and b'.' is 26 in ascii, so we can just check parity of the byte
            let solved = solved
                .bytes()
                .rev()
                .fold(0, |n, b| (n << 1) | (b as u16 & 1));
            let ms = ms
                .split_whitespace()
                .map(|s| {
                    s[1..s.len() - 1]
                        .split(',')
                        .map(p::<u16>)
                        .fold(0, |n, i| n | (1 << i))
                })
                .collect_vec();

            (0..1 << ms.len())
                .filter(|&n: &u16| {
                    ms.iter().enumerate().fold(
                        0,
                        |s, (i, m)| {
                            if 1 << i & n != 0 {
                                s ^ m
                            } else {
                                s
                            }
                        },
                    ) == solved
                })
                .map(|n| n.count_ones())
                .min()
                .unwrap()
        })
        .sum::<u32>()
}

#[test]
fn test() {
    let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    let output = 33;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

/*
fn simplex(tabeleau: &mut [Vec<f64>]) {
    let width = tabeleau[0].len();
    let height = tabeleau.len();
    loop {
        let Some(pivot_col) = tabeleau[0]
            .iter()
            .skip(1)
            .position(|&n| n > 0.)
            .map(|n| n + 1)
        else {
            return;
        };
        if pivot_col == width - 1 {
            return;
        }
        let pivot_row = (1..tabeleau.len())
            .filter(|&i| tabeleau[i][pivot_col] > 0.)
            .sorted_by(|&a, &b| {
                (tabeleau[a][width - 1] * tabeleau[b][pivot_col])
                    .partial_cmp(&(tabeleau[b][width - 1] * tabeleau[a][pivot_col]))
                    .unwrap()
            })
            .next()
            .unwrap();

        println!("{pivot_col} {pivot_row}");

        let x = tabeleau[pivot_row][pivot_col];
        for c in 0..width {
            tabeleau[pivot_row][c] /= x;
        }

        for r in (0..height).filter(|&r| r != pivot_row) {
            let n = tabeleau[r][pivot_col];
            for c in 0..width {
                tabeleau[r][c] -= n * tabeleau[pivot_row][c];
            }
        }

        /*
        for l in tabeleau.iter() {
            println!("{l:?}");
        }
        println!();
        */
    }
}
*/

pub fn part_2(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|s| {
            let (_, s) = s[1..].split_once(']').unwrap();
            let (ms, s) = s[1..].split_once('{').unwrap();
            let (js, _) = s.split_once('}').unwrap();

            let js = js.split(',').map(p::<i32>).collect_vec();
            let ms = ms
                .split_whitespace()
                .map(|s| {
                    s[1..s.len() - 1].split(',').map(p::<usize>).fold(
                        vec![0; js.len()],
                        |mut v, i| {
                            v[i] = 1;
                            v
                        },
                    )
                })
                .collect_vec();
            // transposed implicitly

            let m = ms.len();
            let n = js.len();

            /*
            // compute an echelon form of this matrix over Z, algorithm from
            // <https://cs.uwaterloo.ca/~astorjoh/diss2up.pdf>
            // M = UA where A is in echelon form
            // we're solving for x where Mx = j i.e. UA = j
            // then A = U^-1 j, and so we compute both a and U^-1 j

            let mut r = 0;
            for k in 0..m {
                for i in r+1..n {
                    let (_, s, t, u, v) = gcdex(ms[k][r], ms[k][i]);
                    for j in 0..m {
                        let (a, b) = (ms[j][r], ms[j][i]);
                        ms[j][r] = s * a + t * b;
                        ms[j][i] = u * a + v * b;
                    }
                    let (a, b) = (js[r], js[i]);
                    js[r] = s * a + t * b;
                    js[i] = u * a + v * b;
                }
                if ms[r][r] != 0 {
                    r += 1;
                }
                if r >= n {
                    break;
                }
            }

            for l in &ms {
                println!("{l:?}");
            }
            println!();
            println!("{js:?}");
            println!();

            // From the echelon form we can read off a basis of the null space of M (the rows
            // except the first non-zero entry is negated). We aim to find a minimal solution to M
            // with no negative entries, and so what we can do is find any solution to M, then
            // reduce it by these null space vectors. Since linear algebra is nice, we should just
            // be able to keep trying reductions until we get the minimal vector... i think ...
            // Regardless, we need to first find a solution (any solution) and we can compute this
            // from the echelon form rather nicely!
            let mut sol = vec![0; m];
            let mut pivot = 0;
            'out: for row in 0..n {
                while ms[pivot][row] == 0 {
                    pivot += 1;
                    if pivot >= m {
                        break 'out;
                    }
                }

                sol[pivot] = ms[pivot][row] * js[row];
                for i in 0..row {
                    sol[i] -= ms[pivot][i] * sol[pivot];
                }
            }
            println!("{sol:?}");
            println!();

            // This solution might have negative entries, and so we have to correct this (by just
            // growing the vector a ton lol)
            pivot = 0;
            'out: for row in 0..n {
                while ms[pivot][row] == 0 {
                    pivot += 1;
                    if pivot >= m {
                        break 'out;
                    }
                }

                let n = -sol[pivot];
                sol[pivot] += n;
                for j in pivot+1..m {
                    sol[j] -= n * ms[j][row];
                }
            }
            println!("{sol:?}");
            println!();

            /*
            // We only need to compute the minimum number of button presses, not the actual
            // sequence of button presses. We can view this as minimising the sum of coefficients
            // of x satisfying Mx = j. We can then reduce this sum using constraints given by each
            // column of the echelon form matrix to eliminate the variables that have their own
            // "echelon columns" (think a + b + c = js[i] -> a = -b - c + js[i] so we replace a
            // with the right hand side).

            let mut d = vec![1; m];
            let mut sum = 0;

            let mut pivot = 0;
            'out: for row in 0..n {
                while ms[pivot][row] == 0 {
                    pivot += 1;
                    if pivot >= m {
                        break 'out;
                    }
                }

                for j in pivot+1..m {
                    d[j] -= d[pivot] * ms[j][row];
                }
                sum += d[pivot] * js[row] * ms[pivot][row];
                d[pivot] = 0;
                println!("{sum} {d:?}");
            }
            println!("{sum} {d:?}");

            sum
            */
            */

            /*
            for l in &ms {
                println!("{l:?}");
            }
            println!();
            println!("{js:?}");
            println!();

            // I SPENT HOURS TRYING TO DO THIS WITH LINEAR ALGEBRA STUFF I LEARNED BUT turns out
            // you can just use the simplex method (i think)! Since the matrix formed by the moves
            // is "totally unimodular" (the matrix contains only 1s and 0s so the only possible
            // determinants of square subsets are 1 0 and -1 i think??? idk about more than 2x2
            // uhhh i hope it works), the simplex method will always give an integer solution!

            let mut tabeleau1 = Vec::new();

            let mut row = vec![1.];
            row.append(&mut vec![-1.; n]);
            row.append(&mut vec![0.; 2 + m]);
            tabeleau1.push(row);

            let mut row = vec![0.; n + 1];
            row.push(1.);
            row.append(&mut vec![-1.; m]);
            row.push(0.);
            tabeleau1.push(row);

            for i in 0..n {
                let mut row = vec![0.; 1 + i];
                row.push(1.);
                row.append(&mut vec![0.; n - i]);
                for r in &ms {
                    row.push(r[i]);
                }
                row.push(js[i]);

                for (j, x) in row.iter().enumerate() {
                    tabeleau1[0][j] += x;
                }

                tabeleau1.push(row);
            }

            /*
            for l in &tabeleau1 {
                println!("{l:?}");
            }
            println!();
            */

            simplex(&mut tabeleau1);

            /*
            for l in &tabeleau1 {
                println!("{l:?}");
            }
            println!();

            println!("1");
            */

            let mut tabeleau2 = Vec::new();
            for row in tabeleau1.into_iter().skip(1) {
                tabeleau2.push(row[n + 1..].to_vec());
            }

            /*
            for l in &tabeleau2 {
                println!("{l:?}");
            }
            println!();
            */

            simplex(&mut tabeleau2);

            for l in &tabeleau2 {
                println!("{l:?}");
            }
            println!();

            println!(
                "{}",
                (tabeleau2[0].last().unwrap() / tabeleau2[0][0]) as i64
            );
            (tabeleau2[0].last().unwrap() / tabeleau2[0][0]).ceil() as i64
                */

            // IT WASN'T TOTALLY UNIMODULAR I HATE THIS

            // I GIVE UP !! I'M USING A LIBARRY

            use good_lp::{default_solver, variable, variables, Expression, Solution, SolverModel};
            variables! {problem: };
            let vars = vec![variable().integer().min(0); m];
            let vars: Vec<_> = problem.add_all(vars);
            let objective: Expression = vars.iter().sum();
            let mut model = problem.minimise(&objective).using(default_solver);
            for i in 0..n {
                model = model.with(
                    (0..m)
                        .filter(|&j| ms[j][i] == 1)
                        .map(|j| vars[j])
                        .sum::<Expression>()
                        .eq(js[i]),
                );
            }
            let sol = model.solve().unwrap();
            println!("{}", sol.eval(&objective));

            sol.eval(objective) as i32
        })
        .sum::<i32>()
}
