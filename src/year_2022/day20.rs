use itertools::Itertools;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut list = Vec::new();
    for num in input.lines() {
        list.push((num.parse::<isize>().unwrap(), false));
    }
    let len = list.len();

    loop {
        let mut index = 0;
        let mut done = true;
        for (i, (_, visited)) in list.iter().enumerate() {
            if !visited {
                index = i;
                done = false;
                break;
            }
        }
        if done {
            break;
        }

        let num = list[index].0;
        list.remove(index);
        let new_index = (num + index as isize - 1).rem_euclid(list.len() as isize) as usize + 1;
        list.insert(new_index, (num, true));
    }
    let idxof = list.iter().find_position(|e| e.0 == 0).unwrap().0;

    (list[(idxof + 1000) % len].0 + list[(idxof + 2000) % len].0 + list[(idxof + 3000) % len].0)
        .to_string()
}

#[test]
fn testp1() {
    assert_eq!(
        part_1(
            "1
2
-3
3
-2
0
4"
        ).to_string(),
        3.to_string()
    );
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut list = Vec::new();
    for (i, num) in input.lines().enumerate() {
        list.push((num.parse::<isize>().unwrap() * 811589153, i));
    }
    let len = list.len();

    for _ in 0..10 {
        for iter in 0.. {
            let mut index = 0;
            let mut done = true;
            for (i, (_, id)) in list.iter().enumerate() {
                if id == &iter {
                    index = i;
                    done = false;
                    break;
                }
            }
            if done {
                break;
            }

            let num = list[index].0;
            list.remove(index);
            let new_index = (num + index as isize - 1).rem_euclid(list.len() as isize) as usize + 1;
            list.insert(new_index, (num, iter));
        }
    }
    let idxof = list.iter().find_position(|e| e.0 == 0).unwrap().0;

    (list[(idxof + 1000) % len].0 + list[(idxof + 2000) % len].0 + list[(idxof + 3000) % len].0)
        .to_string()
}

#[test]
fn testp2() {
    assert_eq!(
        part_2(
            "1
2
-3
3
-2
0
4"
        ).to_string(),
        1623178306.to_string()
    );
}
