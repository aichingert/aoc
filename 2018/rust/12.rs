// Advent of Code 2018, day 12
// (c) aichingert

use std::collections::{HashSet,HashMap};

fn part1(cur: &mut HashSet<i32>, rules: &HashMap<String, String>, steps: u64) -> i32 {
    let mut min = -5;
    let mut max = 900;

    for _ in 0..steps {
        let mut next = HashSet::new();
        let mut elements = String::new();
        for j in min..min+5 {
            elements.push(if cur.contains(&j) { '#' } else { '.' });
        }

        for i in min+2..max {
            if rules.contains_key(&elements) && &rules[&elements] == "#" {
                next.insert(i);
            }

            elements.remove(0);
            elements.push(if cur.contains(&(i+3)) { '#' } else { '.' });
        }

        min -= 3;
        max += 3;
        *cur = next;
    }
    println!("{min} {max}");

    cur.iter().fold(0, |c,i| c + *i)
}

fn parse() -> (HashSet<i32>, HashMap<String, String>) {
    let inp = std::fs::read_to_string("../input/12").unwrap();
    let inp = inp.lines().collect::<Vec<&str>>();
    let mut cur = HashSet::new();
    let mut rules = HashMap::new();

    for (i,c) in inp[0].split(' ').collect::<Vec<&str>>()[2].chars().collect::<Vec<char>>().iter().enumerate() {
        if *c == '#' { cur.insert(i as i32); }
    }

    for i in 2..inp.len() {
        let (r#match, to) = inp[i].split_once(" => ").unwrap();
        rules.insert(r#match.to_string(), to.to_string());
    }

    (cur, rules)
}

fn main() {
    let (mut cur, rules) = parse();

    //println!("Part 1: {}", part1(&mut cur, &rules, 20));
    println!("1000: {}", part1(&mut cur.clone(), &rules, 1000));
    println!("2000: {}", part1(&mut cur, &rules, 2000));
}
