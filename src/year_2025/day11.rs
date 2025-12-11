use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

fn search<'a>(
    node: &'a str,
    end: &'a str,
    edges: &HashMap<&'a str, Vec<&'a str>>,
    paths: &mut HashMap<&'a str, u64>,
) -> u64 {
    if node == end {
        1
    } else if paths.contains_key(node) {
        paths[node]
    } else {
        let ans = edges[node]
            .iter()
            .map(|t| search(t, end, edges, paths))
            .sum::<u64>();
        paths.insert(node, ans);
        ans
    }
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    /*
    let edges = input.lines().map(|s| {
        let (a, b) = s.split_once(": ").unwrap();
        let b = b.split(' ').map(|s| ((), s)).collect_vec();

        (a, b)
    }).collect::<HashMap<_, _>>();

    println!("{edges:?}");

    let g = EdgeMap {
        map: edges,
    };

    g.write_graphviz(["you"].into_iter(), "graph.dot");
    // TODO fix string and unit type render
    // TODO search graph
    */

    // graph is acyclic
    let edges = input
        .lines()
        .map(|s| {
            let (a, b) = s.split_once(": ").unwrap();
            let b = b.split(' ').collect_vec();
            (a, b)
        })
        .collect::<HashMap<_, _>>();

    let mut paths = HashMap::new();

    search("you", "out", &edges, &mut paths)
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    /*
    let edges = input.lines().map(|s| {
        let (a, b) = s.split_once(": ").unwrap();
        let b = b.split(' ').map(|s| ((), s)).collect_vec();

        (a, b)
    }).collect::<HashMap<_, _>>();

    println!("{edges:?}");

    let g = EdgeMap {
        map: edges,
    };

    g.write_graphviz(["svr"].into_iter(), "graph.dot");
    */

    let mut edges = input
        .lines()
        .map(|s| {
            let (a, b) = s.split_once(": ").unwrap();
            let b = b.split(' ').collect_vec();
            (a, b)
        })
        .collect::<HashMap<_, _>>();
    edges.insert("out", vec![]);

    let mut paths1 = HashMap::new();
    let mut paths2 = HashMap::new();
    let mut paths3 = HashMap::new();

    search("dac", "out", &edges, &mut paths1)
        * search("fft", "dac", &edges, &mut paths2)
        * search("svr", "fft", &edges, &mut paths3)
}
