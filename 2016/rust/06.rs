// Advent of Code 2016, day 6
// (c) aichingert

use std::collections::HashMap;

fn solve(inp: &Vec<Vec<char>>, part: bool) -> String {
    let mut counter: Vec<HashMap<char, u16>> = vec![HashMap::new();inp[0].len()];
    let mut ans: Vec<char> = Vec::new();

    for i in 0..inp.len() {
        for j in 0..inp[i].len() {
            if let Some(cur) = counter[j].remove(&inp[i][j]) {
                counter[j].insert(inp[i][j], cur+1);
            } else {
                counter[j].insert(inp[i][j], 1);
            }
        }
    }

    for i in 0..counter.len() {
        let mut loc = if part {
            ('_', 0)
        } else {
            ('_', u16::MAX)
        };

        for k in counter[i].keys() {
            if part && loc.1 < counter[i][k] {
                loc = (*k, counter[i][k]);
            }
            if !part && loc.1 > counter[i][k] {
                loc = (*k, counter[i][k]);
            }
        }
        ans.push(loc.0);
    }

    ans.into_iter().collect::<String>()
}

fn main() {
    let inp = std::fs::read_to_string("../input/06").unwrap();
    let inp = inp.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>();

    println!("Part 1: {}", solve(&inp, true));
    println!("Part 2: {}", solve(&inp, false));
}
