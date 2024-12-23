use std::collections::{HashMap, HashSet};

fn sol<'a>(
    depth: u32,
    visited: &mut HashSet<(&'a str, u32)>,
    connection: &'a str, 
    connections: HashSet<&'a str>,
    graph: &'a HashMap<&'a str, Vec<&'a str>>
) -> Vec<HashSet<&'a str>> {
    if !visited.insert((connection, depth)) {
        return vec![];
    }

    let mut lans = Vec::new();

    'out: for jump in &graph[connection] {
        for connected in &connections {
            if !graph[connected].contains(jump) {
                continue 'out;
            }
        }
        
        let mut cp = connections.clone();
        cp.insert(jump);

        lans.push(cp.clone());
        lans.extend_from_slice(&sol(depth + 1, visited, jump, cp, graph));
    }

    lans
}


fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/23").unwrap();

    let mut c = HashSet::new();
    let mut g: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in inp.lines().filter(|l| !l.is_empty()) {
        let (lhs, rhs) = line.split_once("-").unwrap();
        c.insert(lhs);
        c.insert(rhs);

        g.entry(lhs).and_modify(|s| s.push(rhs)).or_insert(vec![rhs]);
        g.entry(rhs).and_modify(|s| s.push(lhs)).or_insert(vec![lhs]);
    }


    let mut pairs = HashSet::new();

    for i in &c {
        let graphs = sol(0, &mut HashSet::new(), i, HashSet::from([*i]), &g);

        for graph in graphs {
            let mut vec = graph.into_iter().collect::<Vec<_>>();
            vec.sort();
            pairs.insert(vec);
        }
    }

    let mut ma = 0;
    let mut p1 = 0;
    let mut p2 = Vec::new();

    for pair in pairs {
        for m in &pair {
            if m.starts_with("t") {
                p1 += 1;
                break;
            }
        }

        if ma < pair.len() {
            ma = pair.len();
            p2 = pair;
        }
    }

    println!("p1: {}", p1);
    println!("p2: {}", p2.join(","));
}
