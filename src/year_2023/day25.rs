use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut g = HashMap::new();
    let mut connections = HashMap::new();
    let mut names = HashSet::new();

    let mut first = "";

    for l in input.lines() {
        let mut s = l.split(": ");
        let name = s.next().unwrap();
        first = name;
        names.insert(name);
        for con in s.next().unwrap().split(' ') {
            names.insert(con);
            g
                .entry(name)
                .or_insert(BTreeSet::new())
                .insert(con);
            connections
                .entry(name)
                .or_insert(BTreeSet::new())
                .insert(con);
            connections
                .entry(con)
                .or_insert(BTreeSet::new())
                .insert(name);
        }
    }

    // graphviz -Kkneato -Tsvg the output
    println!("graph {{");
    for (name, keys) in g.iter() {
        print!("{name} -- {{");
        for key in keys {
            print!("{key} ");
        }
        println!("}}");
    }
    println!("}}");

    /*
    // hfx-pzl
    // bvb-cmg
    // nvd-jqt
    connections.get_mut("hfx").unwrap().remove("pzl");
    connections.get_mut("pzl").unwrap().remove("hfx");
    connections.get_mut("bvb").unwrap().remove("cmg");
    connections.get_mut("cmg").unwrap().remove("bvb");
    connections.get_mut("nvd").unwrap().remove("jqt");
    connections.get_mut("jqt").unwrap().remove("nvd");
    println!("graph {{");
    for (name, keys) in connections.iter() {
        print!("{name} -- {{");
        for key in keys {
            print!("{key} ");
        }
        println!("}}");
    }
    println!("}}");
    */

    // bff-rhk
    // qpp-vnm
    // kfr-vkp
    connections.get_mut("bff").unwrap().remove("rhk");
    connections.get_mut("rhk").unwrap().remove("bff");
    connections.get_mut("qpp").unwrap().remove("vnm");
    connections.get_mut("vnm").unwrap().remove("qpp");
    connections.get_mut("kfr").unwrap().remove("vkp");
    connections.get_mut("vkp").unwrap().remove("kfr");

    let mut stack = Vec::new();
    stack.push(first);
    let mut gr1 = HashSet::new();

    while let Some(n) = stack.pop() {
        for con in connections.get(&n).unwrap().iter() {
            if !gr1.contains(con) {
                stack.push(con);
                gr1.insert(con);
            }
        }
    }

    let count = names.len();
    let g1 = gr1.len();
    println!("{gr1:?} {g1} {count}");

    (count - g1) * g1
}

#[test]
fn test() {
    let input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
    let output = 54;
    assert_eq!(part_1(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    "todo"
}
