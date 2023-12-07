use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut waiting_list = vec![];
    let mut numbers = HashMap::new();
    for line in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    {
        // This includes the : but idc
        let name = &line[0][0..4];
        // Number case
        if line.len() == 2 {
            numbers.insert(name, line[1].parse::<isize>().unwrap());
        } else if numbers.contains_key(line[1]) && numbers.contains_key(line[3]) {
            let n1 = numbers.get(line[1]).unwrap();
            let n2 = numbers.get(line[3]).unwrap();
            numbers.insert(
                name,
                match line[2] {
                    "+" => n1 + n2,
                    "-" => n1 - n2,
                    "*" => n1 * n2,
                    "/" => n1 / n2,
                    _ => unreachable!(),
                },
            );
        } else {
            waiting_list.push((line[1], line[3], line[2], name));
        }

        let mut abort = false;
        while !abort {
            abort = true;
            let mut remove = vec![];
            for (i, (a, b, c, d)) in waiting_list.iter().enumerate() {
                if numbers.contains_key(a) && numbers.contains_key(b) {
                    remove.push(i);
                    abort = false;
                    let n1 = numbers.get(a).unwrap();
                    let n2 = numbers.get(b).unwrap();
                    numbers.insert(
                        d,
                        match *c {
                            "+" => n1 + n2,
                            "-" => n1 - n2,
                            "*" => n1 * n2,
                            "/" => n1 / n2,
                            _ => unreachable!(),
                        },
                    );
                }
            }
            for i in remove.iter().rev() {
                waiting_list.remove(*i);
            }
        }
    }

    numbers.get("root").unwrap().to_string()
}

#[test]
fn testp1() {
    assert_eq!(
        part_1(
            "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"
        )
        .to_string(),
        152.to_string()
    );
}

#[derive(Debug, Clone)]
enum Val {
    Const(i64),
    Var(String),
}

#[derive(Debug, Clone)]
enum Monkey {
    Eq(Val, Val),
    Op(Val, Val, char),
    Const(i64),
    Human,
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut monkeys = Vec::new();
    for line in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
    {
        let name = line[0][0..4].to_string();
        let monkey = if name == "root" {
            Monkey::Eq(Val::Var(line[1].to_string()), Val::Var(line[3].to_string()))
        } else if name == "humn" {
            Monkey::Human
        } else if line.len() == 2 {
            Monkey::Const(line[1].parse().unwrap())
        } else {
            Monkey::Op(
                Val::Var(line[1].to_string()),
                Val::Var(line[3].to_string()),
                line[2].chars().next().unwrap(), // lo
            )
        };

        monkeys.push((name, monkey))
    }
    let mut queue: VecDeque<_> = monkeys.clone().into_iter().map(|(name, _)| name).collect();

    while let Some(name) = queue.pop_front() {
        // : skull
        let (idx, (_, monkey)) = monkeys.iter().find_position(|(n, _)| n == &name).unwrap();
        let monkey = monkey.clone();

        match monkey {
            Monkey::Const(n) => {
                for (_, monkey2) in monkeys.iter_mut() {
                    *monkey2 = match monkey2 {
                        // this match assumes that we dont get the same monkey twice in an
                        // operation/in root
                        Monkey::Eq(Val::Var(name2), var2) if name2 == &name => {
                            Monkey::Eq(Val::Const(n), var2.clone())
                        }
                        Monkey::Eq(var2, Val::Var(name2)) if name2 == &name => {
                            Monkey::Eq(var2.clone(), Val::Const(n))
                        }
                        Monkey::Op(Val::Var(name2), var2, op) if name2 == &name => {
                            Monkey::Op(Val::Const(n), var2.clone(), *op)
                        }
                        Monkey::Op(var2, Val::Var(name2), op) if name2 == &name => {
                            Monkey::Op(var2.clone(), Val::Const(n), *op)
                        }
                        _ => monkey2.clone(), // lmoa
                    };
                }
            }
            Monkey::Op(Val::Const(n1), Val::Const(n2), op) => {
                monkeys[idx].1 = Monkey::Const(match op {
                    '+' => n1 + n2,
                    '-' => n1 - n2,
                    '*' => n1 * n2,
                    '/' => n1 / n2,
                    _ => unreachable!(),
                });
                queue.push_back(name);
            }
            // after a check, it turns out the constant term in the Root value is always the rhs
            // (in the test and in my test data!)
            Monkey::Eq(Val::Var(var), Val::Const(n)) => {
                let (idx2, (_, monkey2)) =
                    monkeys.iter().find_position(|(n, _)| n == &var).unwrap();
                let monkey2 = monkey2.clone();
                match monkey2 {
                    Monkey::Human => return n.to_string(),
                    Monkey::Op(Val::Var(name2), Val::Const(n2), op) => {
                        monkeys[idx2].1 = Monkey::Eq(
                            Val::Var(name2),
                            Val::Const(match op {
                                '+' => n - n2,
                                '-' => n + n2,
                                '*' => n / n2,
                                '/' => n * n2,
                                _ => unreachable!(),
                            }),
                        );
                    }
                    Monkey::Op(Val::Const(n2), Val::Var(name2), op) => {
                        monkeys[idx2].1 = Monkey::Eq(
                            Val::Var(name2),
                            Val::Const(match op {
                                '+' => n - n2, // n = n2 + name => name = n - n2
                                '-' => n2 - n, // n = n2 - name => name = n2 - n
                                '*' => n / n2, // n = n2 * name => name = n / n2
                                '/' => n2 / n, // n = n2 / name => name = n2 / n
                                _ => unreachable!(),
                            }),
                        );
                    }
                    _ => {
                        queue.push_back(name);
                    }
                }
                if !queue.contains(&var) {
                    queue.push_back(var);
                }
            }

            _ => queue.push_back(name),
        }
    }

    "failed to find a solution :(".to_string()
}

#[test]
fn testp2() {
    assert_eq!(
        part_2(
            "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"
        )
        .to_string(),
        301.to_string()
    );
}
