// Advent of Code 2015, day 9
// (c) aichingert

#[path="../../utils/rust/permutations.rs"] mod permutations;
use permutations::permutations;
use std::collections::{HashMap, HashSet};

fn solve(dist: &HashMap<(String,String), u32>, cities: &HashSet<String>, part: bool) -> u32 {
    let mut ans: u32 = match part {
        true => u32::MAX,
        false => 0,
    };
    let mut vec: Vec<String> = cities.iter().map(|s| s.to_string()).collect();
    let perms = permutations(vec.len(), &mut vec);

    for perm in perms {
        match part {
            true => ans = ans.min(dist[&(perm[0].clone(),perm[1].clone())] + dist[&(perm[1].clone(), perm[2].clone())]),
            false => ans = ans.max(dist[&(perm[0].clone(),perm[1].clone())] + dist[&(perm[1].clone(), perm[2].clone())]),
        };
    }

    ans
}

fn parse() -> (HashMap<(String, String), u32>, HashSet<String>) {
    let inp = std::fs::read_to_string("../input/09").unwrap();
    let mut cities: HashSet<String> = HashSet::new();
    let mut dist: HashMap<(String, String), u32> = HashMap::new();

    for line in inp.lines() {
        let values: Vec<&str> = line.split(' ').collect();
        dist.insert((values[0].to_string(),values[2].to_string()), values[4].parse::<u32>().unwrap());
        dist.insert((values[2].to_string(),values[0].to_string()), values[4].parse::<u32>().unwrap());
        cities.insert(values[0].to_string());
        cities.insert(values[2].to_string());
    }

    (dist, cities)
}

fn main() {
    let (dist, cities) = parse();

    println!("Part 1: {}", solve(&dist, &cities, true));
    println!("Part 2: {}", solve(&dist, &cities, false));
}
