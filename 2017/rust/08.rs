// Advent of Code 2017, day 8
// (c) aichingert

use std::collections::HashMap;

fn solve() -> (i32,i32) {
    let mut sol: (i32,i32) = (0,0);
    let mut reg: HashMap<&str, i32> = HashMap::new();
    let lines = std::fs::read_to_string("../input/08").unwrap();

    for line in lines.lines() {
        let vals = line.split(' ').collect::<Vec<&str>>();
        
        let cond = match vals[5] {
            ">" => *reg.entry(vals[4]).or_insert(0) > vals[6].parse().unwrap(),
            "<" => *reg.entry(vals[4]).or_insert(0) < vals[6].parse().unwrap(),
            ">="=> *reg.entry(vals[4]).or_insert(0) >= vals[6].parse().unwrap(),
            "<="=> *reg.entry(vals[4]).or_insert(0) <= vals[6].parse().unwrap(),
            "=="=> *reg.entry(vals[4]).or_insert(0) == vals[6].parse().unwrap(),
            "!="=> *reg.entry(vals[4]).or_insert(0) != vals[6].parse().unwrap(),
            _ => panic!("invalid operation")
        };

        if cond {
            update(&mut reg, &vals, &mut sol);
        }
    }

    for key in reg.keys() {
        sol.0 = sol.0.max(reg[key]);
    }

    sol
}

fn update<'a>(reg: &mut HashMap<&'a str, i32>, vals: &Vec<&'a str>, sol: &mut (i32,i32)) {
    let cur = reg.entry(vals[0]).or_insert(0);

    match vals[1] {
        "inc" => *cur += vals[2].parse::<i32>().unwrap(),
        "dec" => *cur -= vals[2].parse::<i32>().unwrap(),
        _ => unreachable!()
    };

    sol.1 = sol.1.max(*cur);
}

fn main() {
    let (part1,part2) = solve();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
