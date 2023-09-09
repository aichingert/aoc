// Advent of Code 2015, day 9
// (c) aichingert

#[path = "../../utils/rust/permutations.rs"]
mod permutations;
use permutations::permutations;
use std::collections::{HashMap, HashSet};

fn solve(dist: &HashMap<(String, String), u32>, cities: &HashSet<String>) -> (u32, u32) {
    let (mut part_one, mut part_two) = (u32::MAX, 0u32);
    let mut vec: Vec<String> = cities.iter().map(|s| s.to_string()).collect();
    let perms = permutations(vec.len(), &mut vec);

    for perm in perms {
        part_one = part_one.min(
            (0..perm.len() - 1)
                .map(|i| dist[&(perm[i].clone(), perm[i + 1].clone())])
                .sum::<u32>(),
        );
        part_two = part_two.max(
            (0..perm.len() - 1)
                .map(|i| dist[&(perm[i].clone(), perm[i + 1].clone())])
                .sum::<u32>(),
        );
    }

    (part_one, part_two)
}

fn parse() -> (HashMap<(String, String), u32>, HashSet<String>) {
    let inp = std::fs::read_to_string("../input/09").unwrap();
    let mut cities: HashSet<String> = HashSet::new();
    let mut dist: HashMap<(String, String), u32> = HashMap::new();

    for line in inp.lines() {
        let values: Vec<&str> = line.split(' ').collect();
        dist.insert(
            (values[0].to_string(), values[2].to_string()),
            values[4].parse::<u32>().unwrap(),
        );
        dist.insert(
            (values[2].to_string(), values[0].to_string()),
            values[4].parse::<u32>().unwrap(),
        );
        cities.insert(values[0].to_string());
        cities.insert(values[2].to_string());
    }

    (dist, cities)
}

fn main() {
    let (dist, cities) = parse();
    let (part_one, part_two) = solve(&dist, &cities);

    println!("Part 1: {part_one}");
    println!("Part 2: {part_two}");
}
