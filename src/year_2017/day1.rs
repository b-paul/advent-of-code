pub fn part_1(input: &str) -> impl std::fmt::Display {
    // The only non-digit character in the string would be a new line.
    let nums = input
        .chars()
        .flat_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();

    nums.iter()
        .enumerate()
        .filter(|(i, &n)| n == nums[(i + 1) % nums.len()])
        .map(|(_, n)| n)
        .sum::<u32>()
}

#[test]
fn test1() {
    let input = "1122";
    let output = 3;
    assert_eq!(part_1(input).to_string(), output.to_string());
    let input = "1111";
    let output = 4;
    assert_eq!(part_1(input).to_string(), output.to_string());
    let input = "1234";
    let output = 0;
    assert_eq!(part_1(input).to_string(), output.to_string());
    let input = "91212129";
    let output = 9;
    assert_eq!(part_1(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    // The only non-digit character in the string would be a new line.
    let nums = input
        .chars()
        .flat_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();

    nums.iter()
        .enumerate()
        .filter(|(i, &n)| n == nums[(i + (nums.len() / 2)) % nums.len()])
        .map(|(_, n)| n)
        .sum::<u32>()
}

#[test]
fn test2() {
    let input = "1212";
    let output = 6;
    assert_eq!(part_2(input).to_string(), output.to_string());
    let input = "1221";
    let output = 0;
    assert_eq!(part_2(input).to_string(), output.to_string());
    let input = "123425";
    let output = 4;
    assert_eq!(part_2(input).to_string(), output.to_string());
    let input = "123123";
    let output = 12;
    assert_eq!(part_2(input).to_string(), output.to_string());
    let input = "12131415";
    let output = 4;
    assert_eq!(part_2(input).to_string(), output.to_string());
}
