use bitvec::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile1 {
    id: u32,
    edges: [u32; 8],
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut tiles = Vec::new();

    for lines in input.split("\n\n") {
        // jank parsing
        let lines: Vec<&str> = lines.split('\n').collect();
        if lines[0].is_empty() {
            break;
        }
        let id: u32 = lines[0].split(' ').nth(1).unwrap()[0..4].parse().unwrap();
        let tile: Vec<Vec<bool>> = lines[1..]
            .into_iter()
            .map(|s| {
                s.chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        let mut edges = [0; 8];
        for i in 0..10 {
            for edge in edges.iter_mut() {
                *edge <<= 1;
            }

            edges[0] |= tile[0][i] as u32;
            edges[1] |= tile[0][9 - i] as u32;
            edges[2] |= tile[i][0] as u32;
            edges[3] |= tile[9 - i][0] as u32;
            edges[4] |= tile[9][i] as u32;
            edges[5] |= tile[9][9 - i] as u32;
            edges[6] |= tile[i][9] as u32;
            edges[7] |= tile[9 - i][9] as u32;
        }
        let tile = Tile1 { id, edges };
        tiles.push(tile);
    }

    let mut edges: HashMap<u32, u32> = HashMap::new();

    for tile in tiles.iter() {
        for edge in tile.edges {
            if !edges.contains_key(&edge) {
                edges.insert(edge, 1);
            } else if let Some(i) = edges.get_mut(&edge) {
                *i += 1;
            }
        }
    }

    let corners: Vec<_> = tiles
        .into_iter()
        .filter(|tile| {
            let mut connectors = 0;
            for edge in tile.edges {
                if edges.get(&edge) == Some(&2) {
                    connectors += 1;
                }
            }
            connectors == 4
        })
        .collect();

    corners
        .iter()
        .map(|t| t.id as u64)
        .product::<u64>()
        .to_string()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile2 {
    id: u32,
    edges: [u32; 8],
    inside: Vec<Vec<bool>>,
}

fn invert(rotation: u32) -> u32 {
    match rotation {
        0 => 0,
        1 => 1,
        2 => 6,
        3 => 3,
        4 => 4,
        5 => 5,
        6 => 2,
        7 => 7,
        _ => unreachable!()
    }
}

impl Tile2 {
    fn rotate(&mut self, rotation: u32) {
        match rotation {
            0 => {},
            1 => {
                // flip
                for y in 0..4 {
                    for x in 0..8 {
                        let tmp = self.inside[y][x];
                        self.inside[y][x] = self.inside[y][7-x];
                        self.inside[y][7-x] = tmp;
                    }
                }
                // flip the edges
                { let tmp = self.edges[0]; self.edges[0] = self.edges[1]; self.edges[1] = tmp; }
                { let tmp = self.edges[4]; self.edges[4] = self.edges[5]; self.edges[5] = tmp; }
                { let tmp = self.edges[2]; self.edges[2] = self.edges[7]; self.edges[7] = tmp; }
                { let tmp = self.edges[3]; self.edges[3] = self.edges[6]; self.edges[6] = tmp; }
            },
            2 => {
                // rotate
                for y in 0..4 {
                    for x in 0..4 {
                        let tmp = self.inside[y][x];
                        self.inside[y][x] = self.inside[x][7-y];
                        self.inside[x][7-y] = self.inside[7-y][7-x];
                        self.inside[7-y][7-x] = self.inside[7-x][y];
                        self.inside[7-x][y] = tmp;
                    }
                }
                // rotate the edges :sob:
                {
                    let tmp = self.edges[0];
                    self.edges[0] = self.edges[6];
                    self.edges[6] = self.edges[4];
                    self.edges[4] = self.edges[2];
                    self.edges[2] = tmp;
                }
                {
                    let tmp = self.edges[1];
                    self.edges[1] = self.edges[7];
                    self.edges[7] = self.edges[5];
                    self.edges[5] = self.edges[3];
                    self.edges[3] = tmp;
                }
            },
            3 => { self.rotate(1); self.rotate(2); },
            4 => { self.rotate(2); self.rotate(2); },
            5 => { self.rotate(1); self.rotate(4); },
            6 => { self.rotate(2); self.rotate(2); self.rotate(2); },
            7 => { self.rotate(1); self.rotate(6); },
            _ => unreachable!(),
        }
    }
}

fn rotate(vec: &mut BitVec, rotation: u32) {
    match rotation {
        0 => {},
        1 => {
            // flip
            let rf = vec.clone();
            for y in 0..48 {
                for x in 0..96 {
                    let tmp = rf[y * 96 + x];
                    vec.set(y * 96 + x, rf[y * 96 + (95 - x)]);
                    vec.set(y * 96 + (95 - x), tmp);
                }
            }
        },
        2 => {
            // rotate
            let rf = vec.clone();
            for y in 0..48 {
                for x in 0..48 {
                    let tmp = rf[y * 96 + x];
                    vec.set(y * 96 + x, rf[x * 96 + (95 - y)]);
                    vec.set(x * 96 + (95 - y), rf[(95 - y) * 96 + (95 - x)]);
                    vec.set((95 - y) * 96 + (95 - x), rf[(95 - x) * 96 + y]);
                    vec.set((95 - x) * 96 + y, tmp);
                }
            }
        },
        3 => { rotate(vec, 1); rotate(vec, 2); },
        4 => { rotate(vec, 2); rotate(vec, 2); },
        5 => { rotate(vec, 1); rotate(vec, 4); },
        6 => { rotate(vec, 2); rotate(vec, 2); rotate(vec, 2); },
        7 => { rotate(vec, 1); rotate(vec, 6); },
        _ => unreachable!(),
    }
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut tiles = Vec::new();

    for lines in input.split("\n\n") {
        // jank parsing
        let lines: Vec<&str> = lines.split('\n').collect();
        if lines[0].is_empty() {
            break;
        }
        let id: u32 = lines[0].split(' ').nth(1).unwrap()[0..4].parse().unwrap();
        let tile: Vec<Vec<bool>> = lines[1..]
            .into_iter()
            .map(|s| {
                s.chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        let mut edges = [0; 8];
        for i in 0..10 {
            for edge in edges.iter_mut() {
                *edge <<= 1;
            }

            edges[0] |= tile[0][i] as u32;
            edges[1] |= tile[0][9 - i] as u32;
            edges[2] |= tile[i][0] as u32;
            edges[3] |= tile[9 - i][0] as u32;
            edges[4] |= tile[9][i] as u32;
            edges[5] |= tile[9][9 - i] as u32;
            edges[6] |= tile[i][9] as u32;
            edges[7] |= tile[9 - i][9] as u32;
        }
        let mut inside = Vec::new();
        for x in 1..9 {
            let mut row = Vec::new();
            for y in 1..9 {
                row.push(tile[x][y]);
            }
            inside.push(row);
        }
        let tile = Tile2 { id, edges, inside };
        tiles.push(tile);
    }

    let mut edges: HashMap<u32, u32> = HashMap::new();

    for tile in tiles.iter() {
        for edge in tile.edges {
            if !edges.contains_key(&edge) {
                edges.insert(edge, 1);
            } else if let Some(i) = edges.get_mut(&edge) {
                *i += 1;
            }
        }
    }

    let mut corner = tiles
        .iter()
        .filter(|tile| {
            let mut connectors = 0;
            for edge in tile.edges {
                if edges.get(&edge) == Some(&2) {
                    connectors += 1;
                }
            }
            connectors == 4
        })
        .next().unwrap().clone();

    corner.rotate(1); // i needed to do this i swear

    // Rotate the corner so that it's two matching edges are in the bottom and right (bad code yikes)
    loop {
        let mut found = 0;
        for tile in tiles.iter() {
            if tile.id == corner.id { continue; }
            for edge in tile.edges {
                if edge == corner.edges[0] { found += 1; }
                if edge == corner.edges[6] { found += 1; }
            }
        }
        if found == 0 { break; }
        corner.rotate(2);
    }

    let mut grid: Vec<Vec<Option<Tile2>>> = Vec::new();
    for _ in 0..12 {
        let mut row = Vec::new();
        for _ in 0..12 {
            row.push(None);
        }
        grid.push(row);
    }

    grid[0][0] = Some(corner);
    let mut queue = VecDeque::new();
    let mut placed = HashSet::new();
    let mut queued = HashSet::new();
    queue.push_back((0, 0));
    placed.insert(grid[0][0].clone().unwrap().id);
    queued.insert((0, 0));

    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy, idx) in [
            (0, -1, 0), (1, 0, 2), (0, 1, 4), (-1, 0, 6),
            (0, -1, 1), (1, 0, 3), (0, 1, 5), (-1, 0, 7),
        ] {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            if new_x < 0 || new_x >= 12 || new_y < 0 || new_y >= 12
                || queued.contains(&(new_x as usize, new_y as usize)) {
                continue;
            }
            let new_x = new_x as usize;
            let new_y = new_y as usize;
            let cur_tile = grid[y][x].clone().unwrap();
            println!("{new_x} {new_y} {idx} {} {}", cur_tile.id, cur_tile.edges[idx]);
            for tile in tiles.iter() {
                if tile.id == cur_tile.id {
                    continue;
                }
                if placed.contains(&tile.id) {
                    continue;
                }
                let mut found_edge = false;
                let mut edge_idx = 0;
                for (i, edge) in tile.edges.into_iter().enumerate() {
                    // :vomiting_face:
                    if edge == cur_tile.edges[idx] {
                        println!("{}", tile.id);
                        found_edge = true;
                        edge_idx = i;
                    }
                };
                if found_edge {
                    queue.push_back((new_x, new_y));
                    queued.insert((new_x, new_y));
                    let mut tile = tile.clone();
                    println!("Start index {} {:?}", idx, cur_tile.edges);
                    println!("{:?}", tile.edges);
                    tile.rotate(invert(edge_idx as u32));
                    println!("{:?}", tile.edges);
                    tile.rotate(idx as u32 ^ 4);
                    println!("{:?}", tile.edges);
                    println!("End");
                    placed.insert(tile.id);
                    grid[new_y][new_x] = Some(tile);
                    break;
                }
            }
        }
    }

    //println!("{:?}", grid[0][0]);
    //println!("{:?}", grid[0][1]);
    println!("{:?}", grid[0][1]);
    for tile in tiles.iter() {
        for edge in tile.edges {
            if edge == 37 {
                println!("{:?}", tile);
            }
        }
    }

    let grid2: Vec<Vec<_>> = grid.clone().into_iter().map(|v| v.into_iter().map(|t| t.map(|x| x.edges)).collect()).collect();
    println!("{:?}", grid2);

    let grid: Vec<Vec<_>> = grid.into_iter().map(|v| v.into_iter().map(|t| t.unwrap()).collect()).collect();
    let p: Vec<Vec<_>> = grid.clone().into_iter().map(|v| v.into_iter().map(|t| t.id).collect()).collect();
    println!("{:?}", p);

    let mut bits = bitvec![];

    for y in 0..12 {
        for row in 0..8 {
            for x in 0..12 {
                let vec = &grid[y][x].inside[row];
                for square in vec {
                    bits.push(*square);
                }
            }
        }
    }

    for y in 0..96 {
        for x in 0..96 {
            match bits[y*96 + x] {
                false => print!("."),
                true => print!("#"),
            }
        }
        println!();
    }

    let mut sea_monster = bitvec![0; 96*96];
    for (y, line) in [
        "                  # ",
        "#    ##    ##    ###",
         "#  #  #  #  #  #   ",
    ].iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                ' ' => sea_monster.set(y * 96 + x, false),
                '#' => sea_monster.set(y * 96 + x, true),
                _ => unreachable!()
            }
        }
    }

    for rotation in 0..8 {
        let mut bits = bits.clone();
        rotate(&mut bits, rotation);
        let mut sea_monster = sea_monster.clone();
        for _y in 0..93 {
            for _x in 0..76 {
                if sea_monster.clone() & bits.clone() == sea_monster {
                    println!("{} {}", _x, _y);
                }
                //println!("{:?}", (sea_monster.clone() & bits.clone()).count_ones());

                sea_monster.shift_right(1);
            }
            sea_monster.shift_right(20);
        }
    }

    "TODO!".to_string()
}
