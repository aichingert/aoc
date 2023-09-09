// Advent of Code 2018, day 1
// (c) aichingert

use std::collections::HashSet;

fn part1(frq: &Vec<i32>) -> i32 {
    frq.iter().sum::<i32>()
}

fn part2(frq: &Vec<i32>) -> i32 {
    let (mut cur, mut pt) = (frq[0],0usize);
    let mut frqs = HashSet::<i32>::new();

    while frqs.insert(cur) {
        if pt + 1 >= frq.len() { pt = 0; } else { pt += 1; }
        cur += frq[pt];
    }

    cur
}

fn main() {
    let inp = std::fs::read_to_string("../input/01").unwrap()
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("Part 1: {}", part1(&inp));
    println!("Part 2: {}", part2(&inp));
}
