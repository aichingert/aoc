// Advent of Code 2016, day 10
// (c) aichingert

use std::collections::HashMap;

fn solve(robots: &mut HashMap<u32, Vec<u32>>, rules: &HashMap<u32, Vec<(String,u32)>>, part_one: bool) -> u32 {
    let mut output: HashMap<u32, Vec<u32>> = HashMap::new();

    loop {
        let next: Option<u32> = robots.keys().find(|&&key| robots[&key].len() == 2).copied();

        let next = match next {
            None => return output[&0][0] * output[&1][0] * output[&2][0],
            Some(n) => n,
        };

        let cur: &mut Vec<u32> = robots.entry(next).or_default();

        if part_one && (cur[0] == 61 && cur[1] == 17 || cur[0] == 17 && cur[1] == 61) {
            return next;
        }

        let low_and_high = match cur[0] > cur[1] {
            true => vec![cur.remove(1),cur.remove(0)],
            false => vec![cur.remove(0),cur.remove(0)],
        };

        for i in 0..2 {
            match rules[&next][i].0.as_str() {
                "bot" => {
                    let robo = robots.entry(rules[&next][i].1).or_insert(Vec::new());
                    robo.push(low_and_high[i]);
                },
                "output" => {
                    let out = output.entry(rules[&next][i].1).or_insert(Vec::new());
                    out.push(low_and_high[i]);
                },
                _ => unreachable!()
            }
        }
    }
}

fn parse() -> (HashMap<u32, Vec<u32>>, HashMap<u32, Vec<(String,u32)>>) {
    let mut robots: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut rules: HashMap<u32, Vec<(String,u32)>> = HashMap::new();

    for line in std::fs::read_to_string("../input/10").unwrap().lines() {
        let vls: Vec<&str> = line.split(' ').collect();

        match vls[0] {
            "value" => {
                let cur: &mut Vec<u32> = robots.entry(vls[5].parse().unwrap()).or_insert(Vec::new());
                cur.push(vls[1].parse().unwrap());
            },
            "bot" => {
                rules.insert(vls[1].parse().unwrap(), vec![(vls[5].to_string(),vls[6].parse().unwrap()),(vls[10].to_string(),vls[11].parse().unwrap())]);
            },
            _ => panic!("invalid input for day 10")
        }
    }

    (robots,rules)
}

fn main() {
    let (mut robots,rules) = parse();

    println!("Part 1: {}", solve(&mut robots.clone(), &rules, true));
    println!("Part 2: {}", solve(&mut robots, &rules, false));
}
