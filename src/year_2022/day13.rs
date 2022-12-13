use itertools::Itertools;

#[derive(Debug, Clone)]
enum Packet {
    List(Vec<Packet>),
    Num(usize),
}

impl From<String> for Packet {
    fn from(str: String) -> Packet {
        if str.starts_with('[') {
            let mut v = vec![];

            let mut chars = str.chars();
            chars.next();
            chars.next_back();
            let s = chars.as_str();

            if s.is_empty() {
                return Packet::List(v);
            }

            let bytes = s.as_bytes();
            let mut brack = 0;

            let mut start_idx = 0;

            for (i, b) in bytes.iter().enumerate() {
                if *b == b'[' {
                    brack += 1;
                } else if *b == b']' {
                    brack -= 1;
                } else if brack == 0 && *b == b',' {
                    let string = s[start_idx..i].to_string();
                    v.push(string.into());

                    start_idx = i + 1;
                }
            }
            let string = s[start_idx..].to_string();
            v.push(string.into());

            Packet::List(v)
        } else {
            Packet::Num(str.parse().unwrap())
        }
    }
}

fn ordered(p1: Packet, p2: Packet) -> Option<bool> {
    match p1 {
        Packet::List(l1) => match p2 {
            Packet::List(l2) => {
                for elem in l1.iter().zip_longest(l2.iter()) {
                    match elem {
                        itertools::EitherOrBoth::Both(a, b) => {
                            if let Some(c) = ordered(a.clone(), b.clone()) {
                                //println!("order: {:?} {:?}  {}", a, b, c);
                                return Some(c);
                            }
                        }
                        // p2 has finished
                        itertools::EitherOrBoth::Left(_) => return Some(false),
                        // p1 has finished
                        itertools::EitherOrBoth::Right(_) => return Some(true),
                    }
                }
                None
            }
            Packet::Num(n2) => ordered(Packet::List(l1), Packet::List(vec![Packet::Num(n2)])),
        },
        Packet::Num(n1) => match p2 {
            Packet::List(l2) => ordered(Packet::List(vec![Packet::Num(n1)]), Packet::List(l2)),
            Packet::Num(n2) => {
                if n1 < n2 {
                    Some(true)
                } else if n1 > n2 {
                    Some(false)
                } else {
                    None
                }
            }
        },
    }
}

pub fn part_1(input: String) -> String {
    let mut total = 0;
    for (i, lines) in input.split("\n\n").enumerate().map(|(i, l)| (i + 1, l)) {
        let packets: Vec<&str> = lines.split('\n').collect();
        if ordered(packets[0].to_string().into(), packets[1].to_string().into()).unwrap() {
            total += i;
        }
    }

    total.to_string()
}

#[test]
fn testp1() {
    assert_eq!(
        part_1(
            "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
                .to_string()
        ),
        13.to_string()
    )
}

pub fn part_2(input: String) -> String {
    let mut lines = input
        .lines()
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();

    lines.push("[[2]]");
    lines.push("[[6]]");

    lines.sort_by(|a, b| {
        let p1 = a.to_string().into();
        let p2 = b.to_string().into();
        let ordering = ordered(p1, p2);

        match ordering {
            Some(true) => std::cmp::Ordering::Less,
            Some(false) => std::cmp::Ordering::Greater,
            None => std::cmp::Ordering::Equal,
        }
    });

    let total = (lines.iter().position(|&l| l == "[[2]]").unwrap() + 1)
        * (lines.iter().position(|&l| l == "[[6]]").unwrap() + 1);

    total.to_string()
}

#[test]
fn testp2() {
    assert_eq!(
        part_2(
            "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
                .to_string()
        ),
        140.to_string()
    )
}
