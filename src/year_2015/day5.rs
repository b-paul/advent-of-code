pub fn part_1(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .filter(|line| {
            let substr = !line.contains("ab")
                && !line.contains("cd")
                && !line.contains("pq")
                && !line.contains("xy");
            let line: Vec<_> = line.chars().collect();
            let mut vowel_count = 0;
            for char in line.iter() {
                match char {
                    'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
                    _ => (),
                }
            }
            let mut dupe = false;
            for i in 1..line.len() {
                if line[i] == line[i - 1] {
                    dupe = true;
                    break;
                }
            }
            vowel_count >= 3 && dupe && substr
        })
        .count()
}

fn nice_2(str: &str) -> bool {
    let line: Vec<_> = str.chars().collect();
    let mut dupe = false;
    for i in 2..line.len() {
        if line[i] == line[i - 2] {
            dupe = true;
            break;
        }
    }
    let mut double = false;
    'double: for i in 0..(line.len() - 1) {
        for j in 0..(line.len() - 1) {
            if j + 1 == i || j == i || j == i+1 {
                continue;
            }
            if line[i] == line[j] && line[i+1] == line[j+1] {
                double = true;
                break 'double;
            }
        }
    }
    dupe && double
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .filter(|l| nice_2(l))
        .count()
}

#[test]
fn part2() {
    assert!(nice_2("qjhvhtzxzqqjkmpb"));
    assert!(nice_2("xxyxx"));
    assert!(!nice_2("uurcxstgmygtbstg"));
    assert!(!nice_2("ieodomkazucvgmuy"));
    assert!(!nice_2("aaa"));
}
