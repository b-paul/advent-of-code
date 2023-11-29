pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut floor = 0;

    for ch in input.bytes() {
        if ch == b'(' {
            floor += 1;
        } else if ch == b')' {
            floor -= 1;
        }
    }

    floor.to_string()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut floor = 0;

    for (i, ch) in input.bytes().enumerate() {
        if ch == b'(' {
            floor += 1;
        } else if ch == b')' {
            floor -= 1;
        }
        if floor < 0 {
            // Answer is 1 indexed so we need to add 1 to the answer
            return (i + 1).to_string();
        }
    }

    floor.to_string()
}
