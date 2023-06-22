// Advent of Code 2015, day 3
// (c) aichingert

use std::collections::HashSet;

fn solve(inp: &str, part: bool) -> usize {
    let mut s: usize = if part { 1 } else { 0 };
    let mut loc: Vec<(i32, i32)> = vec![(0, 0); s + 1];
    let mut p: HashSet<(i32, i32)> = HashSet::from([loc[s]]);

    inp.chars().for_each(|c| {
        update(&mut loc[s], &c);
        p.insert(loc[s]);
        if part {
            s = 1 - s;
        }
    });

    p.len()
}

fn update(loc: &mut (i32, i32), c: &char) {
    match c {
        '<' => loc.0 -= 1,
        '>' => loc.0 += 1,
        'v' => loc.1 -= 1,
        '^' => loc.1 += 1,
        _ => panic!("Invalid input"),
    };
}

fn main() {
    let input = std::fs::read_to_string("../input/03").unwrap();

    println!("Part 1: {}", solve(input.as_str(), false));
    println!("Part 2: {}", solve(input.as_str(), true));
}
