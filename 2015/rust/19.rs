// Advent of Code 2015, day 19
// (c) aichingert

use std::collections::{HashMap, HashSet};

fn replace_pos(pos: usize, pat: &[char], with: &[char], cur: &[char]) -> Option<String> {
    let mut new = String::new();
    let mut cur_pos = 0usize;
    
    'out: for i in 0..cur.len() {
        for j in i..i+pat.len() {
            if cur[j] != pat[j-i] { 
                new.push(cur[i]);
                continue 'out;
            }
        }

        cur_pos += 1;
            
        if cur_pos <= pos {
            new.push(cur[i]); 
        } else {
            for j in 0..with.len() {
                new.push(with[j]);
            }
            for j in i+pat.len()..cur.len() {
                new.push(cur[j]);
            }

            return Some(new)
        }
    }

    None
}

fn part1(possibilities: &HashMap<String, Vec<String>>, cur: &Vec<char>) -> usize {
    let mut ans: HashSet<String> = HashSet::new();

    for k in possibilities.keys() {
        for i in 0..possibilities[k].len() {
            let mut pos = 0usize;
            let pat = k.chars().collect::<Vec<char>>();
            let with = possibilities[k][i].chars().collect::<Vec<char>>(); 

            while let Some(rep) = replace_pos(pos, &pat, &with, cur) {
                ans.insert(rep);
                pos += 1;
            }
        }
    }

    ans.len()
}

fn part2() {}

fn parse() -> (HashMap<String, Vec<String>>, String) {
    let bind = std::fs::read_to_string("../input/19").unwrap();
    let mut l = bind.lines().collect::<Vec<&str>>();
    let mut p: HashMap<String, Vec<String>> = HashMap::new();
    let (s, _) = (l.pop().unwrap(), l.pop());

    for i in 0..l.len() {
        let vls = l[i].split(" => ").collect::<Vec<&str>>();

        if let Some(mut cur) = p.remove(vls[0]) {
            cur.push(vls[1].to_string());
            p.insert(vls[0].to_string(), cur);
        } else {
            p.insert(vls[0].to_string(), vec![vls[1].to_string()]);
        }
    }

    (p, s.to_string())
}

fn main() {
    let (possibilities, start) = parse();

    println!("Part 1: {}", part1(&possibilities, &start.chars().collect::<Vec<char>>()));
    //println!("Part 2: {}", part2());
}
