// Advent of Code 2015, day 19
// (c) aichingert

use std::collections::{HashMap, HashSet};

fn replace_pos(pos: usize, pat: &[char], with: &[char], cur: &[char]) -> Option<Vec<char>> {
    let mut new = Vec::new();
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

fn part1(possibilities: &HashMap<Vec<char>, Vec<Vec<char>>>, cur: &Vec<char>) -> usize {
    let mut ans: HashSet<Vec<char>> = HashSet::new();

    for k in possibilities.keys() {
        for i in 0..possibilities[k].len() {
            let mut pos = 0usize;

            while let Some(rep) = replace_pos(pos, k, &possibilities[k][i], cur) {
                ans.insert(rep);
                pos += 1;
            }
        }
    }

    ans.len()
}

fn part2(possibilities: &HashMap<Vec<char>, Vec<Vec<char>>>, calculated: &mut HashSet<Vec<char>>, finish: &Vec<char>, cur: &Vec<char>, steps: usize) -> Option<usize> {
    if calculated.contains(cur) || cur.len() > finish.len() {
        return None;
    }

    for k in possibilities.keys() {
        for i in 0..possibilities[k].len() {
            let mut pos = 0usize;

            while let Some(rep) = replace_pos(pos, k, &possibilities[k][i], cur) {
                if &rep == finish {
                    return Some(steps);
                } else {
                    if let Some(stps) = part2(possibilities, calculated, finish, &rep, steps+1) {
                        return Some(stps);
                    }
                    calculated.insert(rep);
                }
                pos += 1;
            }
        }
    }

    None
}

fn parse() -> (HashMap<Vec<char>, Vec<Vec<char>>>, Vec<char>) {
    let bind = std::fs::read_to_string("../input/19").unwrap();
    let mut l = bind.lines().collect::<Vec<&str>>();
    let mut p: HashMap<Vec<char>, Vec<Vec<char>>> = HashMap::new();
    let (s, _) = (l.pop().unwrap(), l.pop());

    for i in 0..l.len() {
        let vls = l[i].split(" => ").collect::<Vec<&str>>();
        let k = vls[0].chars().collect::<Vec<char>>();

        if let Some(mut cur) = p.remove(&k) {
            cur.push(vls[1].chars().collect::<Vec<char>>());
            p.insert(k, cur);
        } else {
            p.insert(k, vec![vls[1].chars().collect::<Vec<char>>()]);
        }
    }

    (p, s.chars().collect::<Vec<char>>())
}

fn main() {
    let (possibilities, start) = parse();

    println!("Part 1: {}", part1(&possibilities, &start));
    println!("Part 2: {:?}", part2(&possibilities, &mut HashSet::new(), &start, &"e".chars().collect::<Vec<char>>(), 1));
}
