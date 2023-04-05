// Advent of Code 2016, day 10
// (c) aichingert

use std::collections::HashMap;

fn part1(robots: &mut HashMap<u32, Vec<u32>>, rules: &HashMap<u32, (String,u32,String,u32)>, part_one: bool) -> u32 {
    let mut output: HashMap<u32, Vec<u32>> = HashMap::new();

    loop {
        let mut next: u32 = 0;

        for key in robots.keys() {
            if robots[key].len() == 2 {
                next = *key;
                break;
            }
        }

        let cur: &mut Vec<u32> = robots.entry(next).or_default();

        match part_one {
            true => if cur[0] == 61 && cur[1] == 17 || cur[0] == 17 && cur[1] == 61 {
                return next;
            },
            false => if cur.len() == 0 {
                return output[&0][0] * output[&1][0] * output[&2][0];
            }
        }
        

        let (low,high) = if cur[0] > cur[1] {
            (cur[1],cur[0])
        } else {
            (cur[0],cur[1])
        };
        cur.remove(0);
        cur.remove(0);

        match rules[&next].0.as_str() {
            "bot" => {
                let robo = robots.entry(rules[&next].1).or_default();
                robo.push(low);
            },
            "output" => {
                let out = output.entry(rules[&next].1).or_insert(Vec::new());
                out.push(low);
            },
            _ => unreachable!()
        }

        match rules[&next].2.as_str() {
            "bot" => {
                let robo = robots.entry(rules[&next].3).or_default();
                robo.push(high);
            },
            "output" => {
                let out = output.entry(rules[&next].3).or_insert(Vec::new());
                out.push(high);
            },
            _ => unreachable!()
        }
    }
}

fn parse() -> (HashMap<u32, Vec<u32>>, HashMap<u32, (String,u32,String,u32)>) {
    let mut robots: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut rules: HashMap<u32, (String,u32,String,u32)> = HashMap::new();

    for line in std::fs::read_to_string("../input/10").unwrap().lines() {
        let vls: Vec<&str> = line.split(' ').collect();

        match vls[0] {
            "value" => {
                let cur: &mut Vec<u32> = robots.entry(vls[5].parse().unwrap()).or_insert(Vec::new());
                cur.push(vls[1].parse().unwrap());
            },
            "bot" => {
                rules.insert(vls[1].parse().unwrap(), (vls[5].to_string(), vls[6].parse().unwrap(),vls[10].to_string(),vls[11].parse().unwrap()));
            },
            _ => panic!("invalid input for day 10")
        }
    }

    (robots,rules)
}

fn main() {
    let (mut robots,rules) = parse();

    println!("Part 1: {}", part1(&mut robots.clone(), &rules, true));
    println!("Part 2: {}", part1(&mut robots, &rules, false));
}
