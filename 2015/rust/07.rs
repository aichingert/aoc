// Advent of Code 2015, day 7
// (c) aichingert

use std::collections::HashMap;

fn solve(map: &mut HashMap<String, Instr>, reg: &mut HashMap<String, u32>, cur: &str) -> u32 {
    let res: Result<u32, _> = cur.parse::<u32>();

    if res.is_ok() {
        return res.unwrap();
    }

    if reg.contains_key(cur) {
        return reg[cur];
    }

    let instr = map[cur].clone();

    let res = match instr {
        Instr::Assign(x) => solve(map, reg, &x),
        Instr::Not(x) => !solve(map, reg, &x),
        Instr::And(x, y) => solve(map, reg, &x) & solve(map, reg, &y),
        Instr::Or(x, y) => solve(map, reg, &x) | solve(map, reg, &y),
        Instr::LShift(x, y) => solve(map, reg, &x) << y,
        Instr::RShift(x, y) => solve(map, reg, &x) >> y,
    };

    reg.insert(cur.to_string(), res);
    res
}

#[derive(Debug, Clone)]
enum Instr {
    Assign(String),
    Not(String),
    And(String, String),
    Or(String, String),
    LShift(String, u32),
    RShift(String, u32)
}

fn parse() -> HashMap<String, Instr> {
    let mut map: HashMap<String, Instr> = HashMap::new();

    for l in std::fs::read_to_string("../input/07").unwrap().lines() {
        let (rhs,lhs): (&str, &str) = l.split_once(" -> ").unwrap();
        let vls = rhs.split(' ').collect::<Vec<&str>>();

        match vls.len() {
            1 => map.insert(lhs.to_string(), Instr::Assign(vls[0].to_string())),
            2 => map.insert(lhs.to_string(), Instr::Not(vls[1].to_string())),
            _ => match vls[1] {
                    "AND" => map.insert(lhs.to_string(), Instr::And(vls[0].to_string(), vls[2].to_string())),
                    "OR" => map.insert(lhs.to_string(), Instr::Or(vls[0].to_string(), vls[2].to_string())),
                    "LSHIFT" => map.insert(lhs.to_string(), Instr::LShift(vls[0].to_string(), vls[2].parse().unwrap())),
                    "RSHIFT" => map.insert(lhs.to_string(), Instr::RShift(vls[0].to_string(), vls[2].parse().unwrap())),
                    _ => panic!("Invalid input!"),
            }
        };
    }

    map
}

fn main() {
    let mut map = parse();
    let mut reg: HashMap<String, u32> = HashMap::new();
    let res: u32 = solve(&mut map, &mut reg, "a");
    reg = HashMap::from([("b".to_string(), res)]);

    println!("Part 1: {}", res);
    println!("Part 2: {}", solve(&mut map, &mut reg, "a"));
}
