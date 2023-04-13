// Advent of Code 2017, day 12
// (c) aichingert

use std::collections::{HashMap,HashSet};

fn solve() -> (usize, usize) {
    let mut connections: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut seen: HashSet<u32> = HashSet::new();
    let mut groups: Vec<Vec<u32>> = Vec::new();

    for line in std::fs::read_to_string("../input/12").unwrap().lines() {
        let (lhs,rhs) = line.split_once(" <-> ").unwrap();
        let cur: u32 = lhs.parse().unwrap();
        let pipes: Vec<u32> = rhs.split(", ").map(|s| s.parse().unwrap()).collect();

        connections.insert(cur, pipes);
    }

    for key in connections.keys() {
        let group = collect_group(&mut seen, &connections, key);

        if group.len() > 0 {
            groups.push(group);
        }
    }

    (groups.iter().find(|group| group.contains(&0)).unwrap().len(),groups.len())
}

fn collect_group(seen: &mut HashSet<u32>, connections: &HashMap<u32, Vec<u32>>, cur: &u32) -> Vec<u32> {
    if seen.contains(cur) {
        return Vec::new();
    }
    seen.insert(*cur);

    let mut group = vec![*cur];

    for pipe in connections[cur].iter() {
        group.append(&mut collect_group(seen, connections, pipe));
    }

    group
}

fn main() {
    let (part1, part2) = solve();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
