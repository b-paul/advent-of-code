fn from_snafu(s: String) -> isize {
    let mut num = 0;
    let mut pow = 1;

    for ch in s.chars().rev() {
        num += pow
            * match ch {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                // idk
                _ => 0,
            };
        pow *= 5;
    }

    num
}

fn to_snafu(mut num: isize) -> String {
    let mut v = vec![];
    while num != 0 {
        v.push(num % 5);
        num /= 5;
    }

    for i in 0..v.len() {
        if v[i] > 2 {
            v[i] -= 5;
            if i == v.len() - 1 {
                v.push(1);
            } else {
                v[i + 1] += 1;
            }
        }
    }

    let mut string = String::new();
    for num in v.iter().rev() {
        string += match num {
            -2 => "=",
            -1 => "-",
            0 => "0",
            1 => "1",
            2 => "2",
            _ => unreachable!()
        };
    }

    string
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut total = 0;
    for line in input.lines() {
        total += from_snafu(line.to_string());
    }

    to_snafu(total)
}

#[test]
fn testp1() {
    assert_eq!(1, from_snafu("1".to_string()));
    assert_eq!(2, from_snafu("2".to_string()));
    assert_eq!(3, from_snafu("1=".to_string()));
    assert_eq!(4, from_snafu("1-".to_string()));
    assert_eq!(5, from_snafu("10".to_string()));
    assert_eq!(6, from_snafu("11".to_string()));
    assert_eq!(7, from_snafu("12".to_string()));
    assert_eq!(8, from_snafu("2=".to_string()));
    assert_eq!(9, from_snafu("2-".to_string()));
    assert_eq!(10, from_snafu("20".to_string()));
    assert_eq!(15, from_snafu("1=0".to_string()));
    assert_eq!(20, from_snafu("1-0".to_string()));
    assert_eq!(2022, from_snafu("1=11-2".to_string()));
    assert_eq!(12345, from_snafu("1-0---0".to_string()));
    assert_eq!(314159265, from_snafu("1121-1110-1=0".to_string()));

    assert_eq!("1=-0-2".to_string(), to_snafu(1747));
    assert_eq!("12111".to_string(), to_snafu(906));
    assert_eq!("2=0=".to_string(), to_snafu(198));
    assert_eq!("21".to_string(), to_snafu(11));
    assert_eq!("2=01".to_string(), to_snafu(201));
    assert_eq!("111".to_string(), to_snafu(31));
    assert_eq!("20012".to_string(), to_snafu(1257));
    assert_eq!("112".to_string(), to_snafu(32));
    assert_eq!("1=-1=".to_string(), to_snafu(353));
    assert_eq!("1-12".to_string(), to_snafu(107));
    assert_eq!("12".to_string(), to_snafu(7));
    assert_eq!("1=".to_string(), to_snafu(3));
    assert_eq!("122".to_string(), to_snafu(37));

    assert_eq!(
        part_1(
            "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"
            .to_string()
        ),
        "2=-1=0".to_string()
    );
}

pub fn part_2(_input: &str) -> impl std::fmt::Display {
    "got trolled".to_string()
}

#[test]
fn testp2() {}
