// Advent of Code 2015, day 16
// (c) aichingert

use std::collections::HashMap;

const S: [(&'static str, i32); 10] =[("children", 3), ("cats", 7),("samoyeds", 2), ("pomeranians", 3), 
            ("akitas", 0),("vizslas", 0),("goldfish", 5), ("trees", 3), ("cars", 2), ("perfumes", 1)];

fn part1(aunts: &Vec<HashMap<String, i32>>) -> usize {
    let mut ans = (0usize, 0i32);

    for i in 0..aunts.len() {
        let mut cur = 0i32;
        for key in S.iter() {
            if aunts[i].contains_key(key.0) && aunts[i][key.0] == key.1 {
                cur += 1;
            }
        }

        if ans.1 < cur {
            ans = (i+1, cur);
        }
    }

    ans.0
}

fn part2(aunts: &Vec<HashMap<String, i32>>) -> usize {
    let mut ans = (0usize, 0i32);

    for i in 0..aunts.len() {
        let mut cur = 0i32;
        for key in S.iter() {
            if aunts[i].contains_key(key.0) {
                match key.0 {
                    "cats" | "trees" => if key.1 <= aunts[i][key.0] { cur += 1; }
                    "pomeranians" | "goldfish" => if key.1 >= aunts[i][key.0] { cur += 1; }
                    _  => if key.1 == aunts[i][key.0] { cur += 1; }
                };
            }
        }

        if ans.1 < cur {
            ans = (i+1, cur);
        }
    }

    ans.0
}

fn parse() -> Vec<HashMap<String, i32>> {
    let mut aunts = Vec::new();

    for l in std::fs::read_to_string("../input/16").unwrap().lines() {
        let (_, rhs) = l.split_once(": ").unwrap();
        let vls = rhs.split(", ").collect::<Vec<&str>>();
        let mut values: HashMap<String, i32> = HashMap::new();

        for v in vls {
            let (key, value) = v.split_once(": ").unwrap();
            let value = value.parse::<i32>().unwrap();
            values.insert(key.to_string(), value);
        }
        aunts.push(values);
    }

    aunts
}

fn main() {
    let aunts = parse();

    println!("Part 1: {}", part1(&aunts));
    println!("Part 2: {}", part2(&aunts));
}
