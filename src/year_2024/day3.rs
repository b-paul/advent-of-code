pub fn part_1(input: &str) -> impl std::fmt::Display {
    input
        .split("mul(")
        .filter_map(|substr| {
            let (a, b) = substr.split_once(",")?;
            let (b, _) = b.split_once(')')?;

            if !(1..=3).contains(&a.len()) || !(1..=3).contains(&b.len()) {
                return None;
            }

            let a = a.parse::<u32>().ok()?;
            let b = b.parse::<u32>().ok()?;

            Some(a * b)
        })
        .sum::<u32>()
}

#[test]
fn test() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let output = 48;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut mul = true;
    let mut sum = 0;

    for i in 0..input.len() {
        if input[i..].starts_with("do()") {
            mul = true;
        } else if input[i..].starts_with("don't()") {
            mul = false;
        } else if input[i..].starts_with("mul(") {
            let Some(prod) = (|| {
                let (_, input) = input[i..].split_once("mul(")?;
                let (a, b) = input.split_once(",")?;
                let (b, _) = b.split_once(')')?;

                if !(1..=3).contains(&a.len()) || !(1..=3).contains(&b.len()) {
                    return None;
                }

                let a = a.parse::<u32>().ok()?;
                let b = b.parse::<u32>().ok()?;

                Some(a * b)
            })() else {
                continue;
            };
            if mul {
                sum += prod;
            }
        }
    }

    sum
}
