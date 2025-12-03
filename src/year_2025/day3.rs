fn solve<const N: usize>(input: &str) -> u64 {
    input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .map_windows::<_, _, N>(|v| v.to_vec())
                .reduce(|m, mut w| {
                    // how do i functionalise this ?!?!
                    for i in 0..m.len() {
                        if m[i] < w[i] {
                            break;
                        }
                        w[i] = m[i];
                    }
                    w
                })
                .map(|m| m.into_iter().fold(0, |a, b| 10 * a + b))
                .unwrap()
        })
        .sum::<u64>()
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    solve::<2>(input)
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    solve::<12>(input)
}
