// Advent of Code 2015, day 13
// (c) aichingert

use std::collections::{HashMap, HashSet};
#[path="../../utils/rust/permutations.rs"] mod permutations;
use permutations::permutations;

fn solve(hap: &HashMap<(String, String), i32>, ppl: &HashSet<String>) -> i32 {
    let mut ans = 0;
    let mut perms: Vec<Vec<String>> = Vec::new();
    permutations(ppl.len(), &mut ppl.iter().map(|p| p.clone()).collect::<Vec<String>>(), &mut perms);

    for perm in perms {
        let mut cur = 0;

        for i in 0..perm.len()-1 {
            cur += hap[&(perm[i].clone(), perm[i+1].clone())];
            cur += hap[&(perm[i+1].clone(), perm[i].clone())];
        }
        ans = ans.max(cur + hap[&(perm[0].clone(), perm[ppl.len()-1].clone())] + hap[&(perm[ppl.len()-1].clone(), perm[0].clone())])
    }

    ans
}

fn parse() -> (HashMap<(String, String), i32>, HashSet<String>) {
    let mut happiness: HashMap<(String,String), i32> = HashMap::new();
    let mut people: HashSet<String> = HashSet::new();

    for line in std::fs::read_to_string("../input/13").unwrap().lines() {
        let vls = line.split(' ').collect::<Vec<&str>>();

        let hap = match vls[2] {
            "gain" => vls[3].parse::<i32>().unwrap(),
            _ => vls[3].parse::<i32>().unwrap()*(-1),
        };

        happiness.insert((vls[0].to_string(), vls[10][..vls[10].len()-1].to_string()), hap);
        people.insert(vls[0].to_string());
    }

    (happiness, people)
}

fn main() {
    let (mut hap, mut ppl) = parse();

    println!("Part 1: {}", solve(&hap, &ppl));
    for p in ppl.iter() {
        hap.insert((p.clone(), "me".to_string()), 0);
        hap.insert(("me".to_string(), p.clone()), 0);
    }
    ppl.insert("me".to_string());
    println!("Part 2: {}", solve(&hap, &ppl));
}
