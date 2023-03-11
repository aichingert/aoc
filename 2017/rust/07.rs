// Advent of Code 2017, day 7
// (c) aichingert

use std::collections::HashMap;

fn part1(towers: &HashMap<String, (Vec<String>, u32)>) -> String {
    let mut clone: HashMap<String, (Vec<String>, u32)> = towers.clone();
    let mut keys = clone.keys().collect::<Vec<&String>>();
    let mut i: usize = 0;
    let mut dependant: Vec<&str> = Vec::new();

    while i < keys.len() {
        if clone[keys[i]].0.len() == 0 {
            dependant.push(keys[i]);
            i+=1;
            continue;
        }

        let mut j = 0usize;

        while j < clone[keys[i]].0.len() {
            let name = &clone[keys[i]].0[j];
            dependant.push(name);
            j += 1;
        }
        i += 1;
    }

    let mut bottom = "".to_string();
    for key in clone.keys() {
        if !dependant.contains(&key.as_str()) {
            bottom = key.clone();
            break;
        }
    }

    bottom
}

fn part2() {}

fn parse() -> HashMap<String, (Vec<String>, u32)> {
    let mut towers = HashMap::new();

    for line in std::fs::read_to_string("../input/07").unwrap().lines() {
        match line.split_once(" -> ") {
            Some((fst, scn)) => {
                let (name, weight): (&str, &str) = fst.split_once(' ').unwrap();

                towers.insert(name.to_string(), (scn.split(", ").map(|s| s.to_string()).collect::<Vec<String>>(), weight[1..weight.len()-1].parse::<u32>().unwrap()));
            },
            None => {
                let (name, weight): (&str, &str) = line.split_once(' ').unwrap();
                towers.insert(name.to_string(), (vec![], weight[1..weight.len()-1].parse::<u32>().unwrap()));
            }
        }
    }

    towers
}

fn main() {
    let towers: HashMap<String, (Vec<String>, u32)> = parse();

    println!("Part 1: {}", part1(&towers));
    //println!("Part 2: {}", part2());
}
