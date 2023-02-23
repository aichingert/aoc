// Advent of Code 2016, day 19
// (c) aichingert

use std::collections::VecDeque;

fn part1(inp: &VecDeque<u32>) -> u32 {
    let mut clone = inp.clone();

    while clone.len() > 1 {
        clone.remove(1);
        let first = clone.remove(0).unwrap();
        clone.push_back(first);
    }

    clone[0]
}

fn part2(inp: &VecDeque<u32>) -> u32 {
    let mut clone = inp.clone();

    while clone.len() > 1 {
        clone.remove(clone.len()/2);
        let first = clone.remove(0).unwrap();
        clone.push_back(first);
    }

    clone[0]
}

fn main() {
    let inp = (1..=std::fs::read_to_string("../input/19").unwrap().trim().parse::<u32>().unwrap()).collect::<VecDeque<u32>>();

    println!("Part 1: {}", part1(&inp));
    println!("Part 2: {}", part2(&inp));
}
