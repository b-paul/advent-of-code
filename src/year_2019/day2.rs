pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut mem: Vec<_> = input
        .trim()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    mem[1] = 12;
    mem[2] = 2;

    let mut i = 0;
    loop {
        match mem[i] {
            1 => {
                let a = mem[i + 1] as usize;
                let b = mem[i + 2] as usize;
                let c = mem[i + 3] as usize;
                mem[c] = mem[a] + mem[b];
            }
            2 => {
                let a = mem[i + 1] as usize;
                let b = mem[i + 2] as usize;
                let c = mem[i + 3] as usize;
                mem[c] = mem[a] * mem[b];
            }
            99 => break,
            _ => unreachable!(),
        }

        i += 4;
    }
    mem[0]
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mem: Vec<_> = input
        .trim()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    for noun in 0..100 {
        for verb in 0..100 {
            let mut mem = mem.clone();
            mem[1] = noun;
            mem[2] = verb;

            let mut i = 0;
            loop {
                match mem[i] {
                    1 => {
                        let a = mem[i + 1] as usize;
                        let b = mem[i + 2] as usize;
                        let c = mem[i + 3] as usize;
                        mem[c] = mem[a] + mem[b];
                    }
                    2 => {
                        let a = mem[i + 1] as usize;
                        let b = mem[i + 2] as usize;
                        let c = mem[i + 3] as usize;
                        mem[c] = mem[a] * mem[b];
                    }
                    99 => break,
                    _ => break,
                }

                i += 4;
            }
            if mem[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    mem[0]
}

#[test]
fn part2() {}
