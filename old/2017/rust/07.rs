// Advent of Code 2017, day 7
// (c) aichingert

use std::collections::HashMap;

fn part1(towers: &HashMap<String, (Vec<String>, u32)>) -> String {
    let keys = towers.keys().collect::<Vec<&String>>();
    let mut bottom: &String = keys[0];
    let mut dependant: bool = true;

    while dependant {
        dependant = false;

        for key in 1..keys.len() {
            if towers[keys[key]].0.contains(bottom) {
                dependant = true;
                bottom = keys[key];
            }
        }
    }

    bottom.to_string()
}

fn part2(towers: &HashMap<String, (Vec<String>,u32)>, top: &String) -> u32 {
    let mut weights = Vec::<(&String,u32)>::new();
    let mut prev = Vec::<(&String,u32)>::new();
    let mut finished = false;
    let mut next = top;

    while !finished {
        for tower in towers[next].0.iter() {
            weights.push((&tower, weight(towers, &tower)));
        }

        weights.sort_by(|a,b| b.1.cmp(&a.1));
        if weights.len() < 2 || weights[0].1 == weights[1].1 {
            finished = true;
        } else {
            next = weights[0].0;
            prev = weights.clone();
            weights.clear();
        }
    }

    towers[prev[0].0].1 - (prev[0].1 - prev[1].1)
}

fn weight(towers: &HashMap<String, (Vec<String>,u32)>, current: &String) -> u32 {
    let mut sum = towers[current].1;

    for tower in towers[current].0.iter() {
        sum += weight(towers, &tower);
    }

    sum
}

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
    let top = part1(&towers);

    println!("Part 1: {}", &top);
    println!("Part 2: {}", part2(&towers, &top));
}
