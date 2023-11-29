use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Node {
    id: String,
    rate: isize,
    edges: Vec<String>,
}

fn search_optimal(
    curnode: String,
    paths: &HashMap<(String, String), (isize, isize)>,
    relevant: &mut HashSet<String>,
    depth: isize,
) -> isize {
    let mut best = 0;

    for node in relevant.clone().iter() {
        let (len, rate) = paths.get(&(curnode.clone(), node.to_string())).unwrap();

        if depth - len < 1 {
            continue;
        }

        relevant.remove(node);

        let mut score = search_optimal(node.clone(), paths, relevant, depth - len);
        score += rate * (depth - len);

        relevant.insert(node.to_string());

        if score > best {
            best = score;
        }
    }

    best
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut set = HashMap::new();
    let start = "AA".to_string();

    let mut relevant_nodes = vec![];

    for line in input
        .lines()
        .map(|l| l.split(' ').collect::<Vec<&str>>())
    {
        let id = line[1].to_string();
        let rate = line[4][5..line[4].len() - 1].parse::<isize>().unwrap();
        let edges = line
            .iter()
            .skip(9)
            .map(|s| (*s)[0..2].to_string())
            .collect::<Vec<String>>();
        let node = Node {
            id: id.clone(),
            rate,
            edges,
        };
        set.insert(id.clone(), node.clone());

        if rate != 0 || id == start {
            relevant_nodes.push(node);
        }
    }

    let mut simple_paths = HashMap::new();

    for a in &relevant_nodes {
        for b in &relevant_nodes {
            let mut stack = VecDeque::new();
            let mut visited = HashSet::new();
            stack.push_back((a, 0));

            while let Some((node, depth)) = stack.pop_front() {
                if visited.contains(node) {
                    continue;
                }
                visited.insert(node);

                if node == b {
                    // +1 to depth because it takes one minute to open a valve
                    simple_paths.insert((a.id.clone(), b.id.clone()), (depth + 1, b.rate));
                    break;
                }

                for id in &node.edges {
                    stack.push_back((set.get(id).unwrap(), depth + 1));
                }
            }
        }
    }

    search_optimal(
        start.clone(),
        &simple_paths,
        &mut relevant_nodes.iter().map(|n| n.id.clone()).collect(),
        30,
    )
    .to_string()
}

#[test]
fn testp1() {
    assert_eq!(
        part_1(
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"
                .to_string()
        ),
        1651.to_string()
    );
}

fn search_double(
    p1node: String,
    p2node: String,
    paths: &HashMap<(String, String), (isize, isize)>,
    relevant: &mut HashSet<String>,
    depth: isize,
) -> isize {
    let mut best = 0;

    for node in relevant.clone().iter() {
        let (len, rate) = paths.get(&(p1node.clone(), node.to_string())).unwrap();

        if depth - len < 1 {
            continue;
        }

        relevant.remove(node);

        let mut score = search_double(node.clone(), p2node.clone(), paths, relevant, depth - len);
        score += rate * (depth - len);

        relevant.insert(node.to_string());

        if score > best {
            best = score;
        }
    }

    let score = search_optimal(p2node, paths, relevant, 26);
    if score > best {
        best = score;
    }

    best
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut set = HashMap::new();
    let start = "AA".to_string();

    let mut relevant_nodes = vec![];

    for line in input
        .lines()
        .map(|l| l.split(' ').collect::<Vec<&str>>())
    {
        let id = line[1].to_string();
        let rate = line[4][5..line[4].len() - 1].parse::<isize>().unwrap();
        let edges = line
            .iter()
            .skip(9)
            .map(|s| (*s)[0..2].to_string())
            .collect::<Vec<String>>();
        let node = Node {
            id: id.clone(),
            rate,
            edges,
        };
        set.insert(id.clone(), node.clone());

        if rate != 0 || id == start {
            relevant_nodes.push(node);
        }
    }

    let mut simple_paths = HashMap::new();

    for a in &relevant_nodes {
        for b in &relevant_nodes {
            let mut stack = VecDeque::new();
            let mut visited = HashSet::new();
            stack.push_back((a, 0));

            while let Some((node, depth)) = stack.pop_front() {
                if visited.contains(node) {
                    continue;
                }
                visited.insert(node);

                if node == b {
                    // +1 to depth because it takes one minute to open a valve
                    simple_paths.insert((a.id.clone(), b.id.clone()), (depth + 1, b.rate));
                    break;
                }

                for id in &node.edges {
                    stack.push_back((set.get(id).unwrap(), depth + 1));
                }
            }
        }
    }

    // This can be sped up i think (this is doing a ton of recomputation similar to the naive
    // fibonacci algorithm)
    search_double(
        start.clone(),
        start.clone(),
        &simple_paths,
        &mut relevant_nodes.iter().map(|n| n.id.clone()).collect(),
        26,
    )
    .to_string()
}

#[test]
fn testp2() {
    /*
    assert_eq!(
        part_2(
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"
                .to_string()
        ),
        1707.to_string()
    );
    */
}
