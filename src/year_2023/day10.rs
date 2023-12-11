use crate::helper::grid::Grid;
use crate::helper::adjacency::adjacent_4_ud;

fn format_chars(c: char) -> char {
    match c {
        '|' => '│',
        '-' => '─',
        'L' => '└',
        'J' => '┘',
        '7' => '┐',
        'F' => '┌',
        '.' => ' ',
        _ => c,
    }
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    const STRS: [&str; 4] = ["S-J7", "S|LJ", "S|7F", "S-LF"];

    let grid = input.parse::<Grid<char>>().unwrap();
    let start = grid.find(&'S').unwrap();

    let pgrid = grid.clone().map(format_chars);
    println!("{pgrid}");

    let mut max_depth = 0;
    grid.bfs_4(
        start,
        |_, _, d| max_depth = max_depth.max(d),
        |dir, from, to| {
            STRS[dir as usize].contains(from) && STRS[dir.opposite() as usize].contains(to)
        },
    );

    max_depth
}

#[test]
fn part1() {
    let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    let output = 8;
    assert_eq!(part_1(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    const STRS: [&str; 4] = ["S-J7", "S|LJ", "S|7F", "S-LF"];

    let grid = input.parse::<Grid<char>>().unwrap();
    let start = grid.find(&'S').unwrap();

    let pgrid = grid.clone().map(format_chars);
    println!("{pgrid}");

    let mut main = grid.clone().map(|_| false);
    grid.bfs_4(
        start,
        |p, _, _| main[p] = true,
        |dir, from, to| {
            STRS[dir as usize].contains(from) && STRS[dir.opposite() as usize].contains(to)
        },
    );

    let grid = grid.map_i(|p, x| if main[p] { x } else { '.' });

    let pgrid = grid.clone().map(format_chars);
    println!("{pgrid}");

    // Make a bigger grid with gaps between pipes if they aren't connected
    let mut adjusted_grid = Grid::new_filled('.', 2 * grid.width() + 1, 2 * grid.height() + 1);
    for ((x, y), c) in grid.iter_idx() {
        if *c == '.' {
            continue;
        }
        let nx = 2 * x + 1;
        let ny = 2 * y + 1;
        adjusted_grid[(nx, ny)] = '#';
        for (p, dir) in adjacent_4_ud(nx, ny) {
            if STRS[dir as usize].contains(*c) {
                adjusted_grid[p] = '#';
            }
        }
    }

    println!("{adjusted_grid}");

    adjusted_grid.floodfill_4('.', '@', (0, 0));

    println!("{adjusted_grid}");

    let grid = grid.map_i(|(x, y), c| {
        match adjusted_grid[(2*x + 1, 2*y + 1)] {
            '.' => 'I',
            '@' => 'O',
            _ => c,
        }
    }).map(format_chars);
    println!("{grid}");

    grid.iter_idx()
        .filter(|((_, _), c)| **c == 'I')
        .count()
}

#[test]
fn part2() {
    let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    let output = 10;
    assert_eq!(part_2(input).to_string(), output.to_string());
    assert!(false);
}
