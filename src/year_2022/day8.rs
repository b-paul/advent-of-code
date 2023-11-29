fn is_visible(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let height = trees[y][x];
    let mut visible = false;
    visible |= !(0..x).any(|x| trees[y][x]>=height);
    visible |= !(x+1..trees[y].len()).any(|x| trees[y][x]>=height);
    visible |= !(0..y).any(|y| trees[y][x]>=height);
    visible |= !(y+1..trees.len()).any(|y| trees[y][x]>=height);

    visible
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut trees: Vec<Vec<u8>> = vec![];

    for line in input.lines() {
        let mut row = vec![];
        for byte in line.as_bytes() {
            row.push(byte - b'0');
        }
        trees.push(row);
    }

    let mut total = trees.len()*2 + trees[0].len()*2-4;

    for i in 1..(trees.len()-1) {
        for j in 1..(trees[i].len()-1) {
            if is_visible(&trees, j, i) {
                total += 1;
            }
        }
    }

    total.to_string()
}

fn scenic_score(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let height = trees[y][x];
    let mut score = 1;
    score *= (0..x).rev().position(|x| trees[y][x] >= height).map(|n|n+1).unwrap_or(x);
    score *= (x+1..trees[y].len()).position(|x| trees[y][x] >= height).map(|n|n+1).unwrap_or(trees[y].len()-x-1);
    score *= (0..y).rev().position(|y| trees[y][x] >= height).map(|n|n+1).unwrap_or(y);
    score *= (y+1..trees.len()).position(|y| trees[y][x] >= height).map(|n|n+1).unwrap_or(trees.len()-y-1);
    score
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut trees: Vec<Vec<u8>> = vec![];

    for line in input.lines() {
        let mut row = vec![];
        for byte in line.as_bytes() {
            row.push(byte - b'0');
        }
        trees.push(row);
    }

    let mut max = 0;

    for i in 1..(trees.len()-1) {
        for j in 1..(trees[i].len()-1) {
            let score = scenic_score(&trees, j, i);
            if score > max {
                max = score;
            }
        }
    }

    max.to_string()
}

#[test]
fn a() {
    let grid =
"30373
25512
65332
33549
35390";
    let mut trees: Vec<Vec<u8>> = vec![];
    for line in grid.lines() {
        let mut row = vec![];
        for byte in line.as_bytes() {
            row.push(byte - b'0');
        }
        trees.push(row);
    }
    assert_eq!(scenic_score(&trees, 2, 1), 4);
    assert_eq!(scenic_score(&trees, 2, 3), 8);
}
