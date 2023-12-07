// TODO actually parse the input instead of hand parsing :grimacing:

fn apply_oper(i: usize, item: &mut u32) {
    *item = match i {
        0 => *item * 7,
        1 => *item + 3,
        2 => *item + 4,
        3 => *item + 5,
        4 => *item * 5,
        5 => *item * *item,
        6 => *item + 8,
        7 => *item + 1,
        _ => unreachable!(),
    };
    *item /= 3;
}

fn opertest(i: usize, item: &mut u32) {
    *item = match i {
        0 => *item * 19,
        1 => *item + 6,
        2 => *item * *item,
        3 => *item + 3,
        _ => unreachable!()
    };
    *item /= 3;
}

fn throw(i: usize, item: u32) -> usize {
    match i {
        0 => if item % 2 == 0 { 7 } else { 1 },
        1 => if item % 7 == 0 { 2 } else { 4 },
        2 => if item % 13 == 0 { 5 } else { 4 },
        3 => if item % 19 == 0 { 6 } else { 0 },
        4 => if item % 11 == 0 { 5 } else { 3 },
        5 => if item % 5 == 0 { 6 } else { 3 },
        6 => if item % 3 == 0 { 0 } else { 7 },
        7 => if item % 17 == 0 { 2 } else { 1 },
        _ => unreachable!()
    }
}

fn throwtest(i: usize, item: u32) -> usize {
    match i {
        0 => if item % 23 == 0 { 2 } else { 3 },
        1 => if item % 19 == 0 { 2 } else { 0 },
        2 => if item % 13 == 0 { 1 } else { 3 },
        3 => if item % 17 == 0 { 0 } else { 1 },
        _ => unreachable!()
    }
}

pub fn part_1(_input: &str) -> impl std::fmt::Display {
    let mut items: Vec<Vec<u32>> = vec![
        vec![62, 92, 50, 63, 62, 93, 73, 50],
        vec![51, 97, 74, 84, 99],
        vec![98, 86, 62, 76, 51, 81, 95],
        vec![53, 95, 50, 85, 83, 72],
        vec![59, 60, 63, 71],
        vec![92, 65],
        vec![78],
        vec![84, 93, 54],
    ];
    let mut inspections = [0; 8];
    for _ in 0..20 {
        for i in 0..8 {
            for item in items[i].iter_mut() {
                apply_oper(i, item);
                inspections[i] += 1;
            }
            let mut remove_list = vec![];
            for (j, item) in items[i].clone().iter().enumerate().rev() {
                remove_list.push(j);
                items[throw(i, *item)].push(*item);
            }
            for j in remove_list {
                items[i].remove(j);
            }
        }
    }
    inspections.sort();
    (inspections[6]*inspections[7]).to_string()
}

#[test]
fn testp1() {
    let _sample = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
        .to_string();
    let mut items = vec![
        vec![79, 98],
        vec![54, 65, 75, 74],
        vec![79, 60, 97],
        vec![74],
    ];
    let mut inspections = [0; 4];
    for _ in 0..20 {
        for i in 0..4 {
            for item in items[i].iter_mut() {
                opertest(i, item);
                inspections[i] += 1;
            }
            let mut remove_list = vec![];
            for (j, item) in items[i].clone().iter().enumerate().rev() {
                remove_list.push(j);
                items[throwtest(i, *item)].push(*item);
            }
            for j in remove_list {
                items[i].remove(j);
            }
        }
    }
    inspections.sort();
    assert_eq!((inspections[2]*inspections[3]), 10605);
}

fn apply_oper2(i: usize, item: &mut u64) {
    *item = match i {
        0 => *item * 7,
        1 => *item + 3,
        2 => *item + 4,
        3 => *item + 5,
        4 => *item * 5,
        5 => *item * *item,
        6 => *item + 8,
        7 => *item + 1,
        _ => unreachable!(),
    };
    *item %= 2*7*13*19*11*5*3*17;
}

fn opertest2(i: usize, item: &mut u64) {
    *item = match i {
        0 => *item * 19,
        1 => *item + 6,
        2 => *item * *item,
        3 => *item + 3,
        _ => unreachable!()
    };
    *item %= 96577;
}

fn throw2(i: usize, item: u64) -> usize {
    match i {
        0 => if item % 2 == 0 { 7 } else { 1 },
        1 => if item % 7 == 0 { 2 } else { 4 },
        2 => if item % 13 == 0 { 5 } else { 4 },
        3 => if item % 19 == 0 { 6 } else { 0 },
        4 => if item % 11 == 0 { 5 } else { 3 },
        5 => if item % 5 == 0 { 6 } else { 3 },
        6 => if item % 3 == 0 { 0 } else { 7 },
        7 => if item % 17 == 0 { 2 } else { 1 },
        _ => unreachable!()
    }
}

fn throwtest2(i: usize, item: u64) -> usize {
    match i {
        0 => if item % 23 == 0 { 2 } else { 3 },
        1 => if item % 19 == 0 { 2 } else { 0 },
        2 => if item % 13 == 0 { 1 } else { 3 },
        3 => if item % 17 == 0 { 0 } else { 1 },
        _ => unreachable!()
    }
}

pub fn part_2(_input: &str) -> impl std::fmt::Display {
    let mut items: Vec<Vec<u64>> = vec![
        vec![62, 92, 50, 63, 62, 93, 73, 50],
        vec![51, 97, 74, 84, 99],
        vec![98, 86, 62, 76, 51, 81, 95],
        vec![53, 95, 50, 85, 83, 72],
        vec![59, 60, 63, 71],
        vec![92, 65],
        vec![78],
        vec![84, 93, 54],
    ];
    let mut inspections = [0u64; 8];
    for _ in 0..10000 {
        for i in 0..8 {
            for item in items[i].iter_mut() {
                apply_oper2(i, item);
                inspections[i] += 1;
            }
            let mut remove_list = vec![];
            for (j, item) in items[i].clone().iter().enumerate().rev() {
                remove_list.push(j);
                items[throw2(i, *item)].push(*item);
            }
            for j in remove_list {
                items[i].remove(j);
            }
        }
    }
    inspections.sort();
    (inspections[6]*inspections[7]).to_string()
}

#[test]
fn testp2() {
    let _sample = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
        .to_string();
    let mut items = vec![
        vec![79, 98],
        vec![54, 65, 75, 74],
        vec![79, 60, 97],
        vec![74],
    ];
    let mut inspections = [0u64; 4];
    for _ in 0..10000 {
        for i in 0..4 {
            for item in items[i].iter_mut() {
                opertest2(i, item);
                inspections[i] += 1;
            }
            let mut remove_list = vec![];
            for (j, item) in items[i].clone().iter().enumerate().rev() {
                remove_list.push(j);
                items[throwtest2(i, *item)].push(*item);
            }
            for j in remove_list {
                items[i].remove(j);
            }
        }
    }
    inspections.sort();
    assert_eq!((inspections[2]*inspections[3]), 2713310158);
}
