pub fn part_1(input: &str) -> impl std::fmt::Display {
    input.lines().filter_map(|l| {
        let ws: Vec<_> = l.split(' ').collect();
        let id = ws[1][0..(ws[1].len() - 1)].parse::<u32>().unwrap();
        
        for i in 1.. {
            if ws.len() < 2*i + 1 {
                break;
            }
            let n = ws[2*i].parse::<u32>().unwrap();
            let c = ws[2*i+1].chars().next().unwrap();
            if c == 'b' && n > 14 { return None; }
            if c == 'g' && n > 13 { return None; }
            if c == 'r' && n > 12 { return None; }
        }

        Some(id)
    }).sum::<u32>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    input.lines().map(|l| {
        let ws: Vec<_> = l.split(' ').collect();

        let mut bs = 0;
        let mut gs = 0;
        let mut rs = 0;
        
        for i in 1.. {
            if ws.len() < 2*i + 1 {
                break;
            }
            let n = ws[2*i].parse::<u32>().unwrap();
            let c = ws[2*i+1].chars().next().unwrap();
            if c == 'b' { bs = bs.max(n) }
            if c == 'g' { gs = gs.max(n) }
            if c == 'r' { rs = rs.max(n) }
        }

        bs * gs * rs
    }).sum::<u32>()
}
